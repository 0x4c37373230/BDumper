extern crate native_windows_gui as nwg;

pub mod files {
    pub fn path_exists(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }

    pub fn create_file(file_type: &str) -> Result<std::fs::File, &str> {
        match file_type {
            ".txt" => {
                std::fs::File::create("./SymHook.txt").expect("ERROR: Could not create file");
                Ok(std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("./SymHook.txt")
                    .unwrap())
            }
            ".hpp" => {
                std::fs::File::create("SymHook.hpp").expect("ERROR: Could not create file");
                Ok(std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("./SymHook.hpp")
                    .unwrap())
            }
            _ => Err("Invalid File Type"),
        }
    }
}

pub mod pdb {
    use {
        nanoid::nanoid,
        pdb::{FallibleIterator, Rva},
        std::io::{BufRead, BufReader},
        std::{fs::File, io::Write},
    };

    pub struct BDSFunction {
        pub name: String,
        pub symbol: String,
        pub rva: Rva,
    }

    impl BDSFunction {
        fn create_instance(name: String, symbol: String, rva: Rva) -> BDSFunction {
            return BDSFunction { name, symbol, rva };
        }
    }

    pub fn name_id() -> String {
        let char_list: [char; 53] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
            'Y', 'Z',
        ];

        nanoid!(10, &char_list)
    }

    pub fn pdb_dump(pdb_path: &str, file_type: &str, mut dump_file: File) -> pdb::Result<()> {
        let file_path = File::open(&pdb_path)?;
        let mut pdb = pdb::PDB::open(file_path)?;
        let symbol_table = pdb.global_symbols()?;
        let address_map = pdb.address_map()?;
        let mut symbols = symbol_table.iter();

        while let Some(symbol) = symbols.next()? {
            match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) if data.function => {
                    let rva = data.offset.to_rva(&address_map).unwrap_or_default();

                    if file_type == ".txt" {
                        write!(dump_file, "{}\n{}\n\n", data.name, rva)
                            .expect("ERROR: Could not write to file");
                    } else if file_type == ".hpp" {
                        let fn_id = name_id();

                        write!(
                            dump_file,
                            "//{};\nconstexpr unsigned int {} = {};\n\n",
                            data.name, fn_id, rva
                        )
                        .expect("ERROR: Could not write to file");
                    } else {
                        break;
                    }
                }
                _ => {}
            }
        }
        nwg::simple_message("Completed", &format!("Completed dumping {}", pdb_path));

        Ok(())
    }

    pub fn find_function(pdb_path: &str, function_name: &str) -> Result<BDSFunction, String> {
        let file_path = File::open(&pdb_path).unwrap();
        let mut pdb = pdb::PDB::open(file_path).unwrap();
        let symbol_table = pdb.global_symbols().unwrap();
        let address_map = pdb.address_map().unwrap();
        let mut symbols = symbol_table.iter();

        while let Some(symbol) = symbols.next().unwrap() {
            match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) if data.function => {
                    let symbol = data.name.to_string().to_string();
                    let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                    let function_sym: Vec<&str> = function_name.split("::").collect();
                    let substr = format!("{}@{}", function_sym[1], function_sym[0]);

                    if symbol.contains(&substr) {
                        let found_function =
                            BDSFunction::create_instance(String::from(function_name), symbol, rva);
                        return Ok(found_function);
                    }
                }
                _ => {}
            }
        }

        Err(String::from(
            "Function was either not found or does not exist",
        ))
    }

    pub fn find_functions(pdb_path: &str, file_type: &str, mut dump_file: File, ) -> Result<(), String> {
        let file = File::open("./dumpFilter.txt").unwrap();
        let functions = BufReader::new(file);

        for line in functions.lines() {
            match find_function(pdb_path, &line.unwrap()) {
                Ok(bds_func) => {
                    if file_type == ".txt" {
                        write!(dump_file, "{}\n{}\n\n", bds_func.symbol, bds_func.rva)
                            .expect("ERROR: Could not write to file");
                    } else if file_type == ".hpp" {
                        let fn_id = name_id();

                        write!(
                            dump_file,
                            "//{};\nconstexpr unsigned int {} = {};\n\n",
                            bds_func.symbol, fn_id, bds_func.rva
                        )
                        .expect("ERROR: Could not write to file");
                    }
                }
                Err(str) => {
                    return Err(str);
                }
            }
        }

        Ok(())
    }
}
