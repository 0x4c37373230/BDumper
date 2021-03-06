extern crate native_windows_gui as nwg;

use std::os::raw::c_char;

extern {
    fn demangle(s: *const c_char) -> *const c_char;
}

pub mod files {
    pub fn path_exists(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }

    pub fn create_file(file_type: &str) -> Result<std::fs::File, &str> {
        let file_path = if file_type == ".txt" {
            "./SymHook.txt"
        } else if file_type == ".hpp" {
            "./SymHook.hpp"
        } else {
            return Err("Invalid File Type");
        };

        match std::fs::File::create(file_path) {
            Ok(_) => Ok(std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap()),
            Err(_) => Err("Could not create file")
        }
    }
}

pub mod setup {
    use {crate::files, std::fs::File, std::io::Write};

    pub fn filter_manager() -> bool {
        if !files::path_exists("./dumpFilter.txt") {
            File::create("dumpFilter.txt").unwrap();

            return false;
        }
        true
    }

    pub fn dump_init(pdb_path: &str, file_type: &str) -> Result<File, String> {
        if files::path_exists(pdb_path) == false {
            return Err(String::from(&format!("File does not exist: {}", pdb_path)));
        }

        let mut dump_file = match files::create_file(&file_type) {
            Ok(file) => file,
            Err(str) => {
                return Err(String::from(&format!("{}: {}", str, file_type)));
            }
        };

        write!(
            dump_file,
            "/*###############################################################\
        \nBDS function symbols and RVAs\
        \nFile generated by BDumper, a rust bds pdb dumper made by Luke7720\
        \n###############################################################*/\n"
        )
        .expect("ERROR: Could not write to file");

        if file_type == ".hpp" {
            write!(dump_file, "#pragma once\n").expect("ERROR: Could not write to file");
        }

        Ok(dump_file)
    }
}

pub mod demangle {
    use std::ffi::{CStr, CString};
    use crate::demangle;

    pub fn undecorate(symbol: &str) -> String {
        unsafe {
            let cstr = CString::new(symbol).unwrap();
            let result: &CStr = CStr::from_ptr(demangle(cstr.as_ptr()));

            return result.to_str().unwrap().to_owned();
        }
    }

    pub fn cleanup_symbol(symbol: &str) -> String {
        let res = undecorate(symbol);
        let demangled_name = res.replace("const", " const").replace("(", "( ");
        let mut declaration: Vec<&str> = demangled_name.split(" ").collect();

        for i in 0..declaration.len() {
            if &declaration[i] as &str == "const"
                && declaration[i - 1].starts_with("__")
                && i != 0
            {
                let check_space = if &declaration[i - 1] as &str == " " {
                    i - 1
                } else {
                    i - 2
                };

                declaration.swap(i as usize, check_space);
            }
        }

        declaration
            .join(" ")
            .replace("class", "")
            .replace("struct", "")
            .replace("  ", " ")
            .replace("   ", " ")
            .replace("< ", "<")
            .replace(" >", ">")
            .replace(" &", "&")
            .replace(" *", "*")
            .replace("( ", "(")
        /*
        let flags = msvc_demangler::DemangleFlags::llvm();

        return match msvc_demangler::demangle(symbol, flags) {
            Ok(res) => {
                let demangled_name = res.replace("const", " const").replace("(", "( ");

                let mut declaration: Vec<&str> = demangled_name.split(" ").collect();

                for i in 0..declaration.len() {
                    if &declaration[i] as &str == "const"
                        && declaration[i - 1].starts_with("__")
                        && i != 0
                    {
                        let check_space = if &declaration[i - 1] as &str == " " {
                            i - 1
                        } else {
                            i - 2
                        };

                        declaration.swap(i as usize, check_space);
                    }
                }

                declaration
                    .join(" ")
                    .replace("class", "")
                    .replace("struct", "")
                    .replace("  ", " ")
                    .replace("   ", " ")
                    .replace("< ", "<")
                    .replace(" >", ">")
                    .replace(" &", "&")
                    .replace(" *", "*")
                    .replace("( ", "(")
            }
            Err(_) => "Unable to demangle symbol".to_string(),
        };
         */
    }
}

pub mod pdb {
    use {
        crate::demangle,
        pdb::{FallibleIterator, Rva},
        std::io::{BufRead, BufReader},
        std::{fs::File, io::Write, time::Instant},
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

    pub fn pdb_dump(pdb_path: &str, file_type: &str, mut dump_file: File, should_demangle: bool) -> pdb::Result<()> {
        let start = Instant::now();
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
                        write!(
                            dump_file,
                            "{}\n{}\n{}\n\n",
                            data.name,
                            demangle::cleanup_symbol(&data.name.to_string()),
                            rva
                        )
                        .expect("ERROR: Could not write to file");
                    } else if file_type == ".hpp" {
                        if should_demangle {
                            write!(
                                dump_file,
                                "//{}\n//{}\nconstexpr unsigned int MD5_{:x} = {};\n\n",
                                data.name,
                                demangle::cleanup_symbol(&data.name.to_string()),
                                md5::compute(data.name.to_string().to_string()),
                                rva
                            )
                            .expect("ERROR: Could not write to file");
                        } else {
                            write!(
                                dump_file,
                                "//{}\nconstexpr unsigned int MD5_{:x} = {};\n\n",
                                data.name,
                                md5::compute(data.name.to_string().to_string()),
                                rva
                            )
                            .expect("ERROR: Could not write to file");
                        }
                    } else {
                        break;
                    }
                }
                _ => {}
            }
        }
        nwg::simple_message(
            "Completed",
            &format!("Completed dumping {} in {:?}", pdb_path, start.elapsed()),
        );

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
                        let found_function = BDSFunction::create_instance(
                            demangle::cleanup_symbol(&symbol),
                            symbol,
                            rva,
                        );
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
            let line_ref: &str = &line.unwrap();

            if line_ref.starts_with("#") || line_ref.is_empty() {
                continue;
            }

            match find_function(pdb_path, line_ref) {
                Ok(bds_func) => {
                    if file_type == ".txt" {
                        write!(
                            dump_file,
                            "{}\n{}\n{}\n\n",
                            &bds_func.symbol,
                            demangle::cleanup_symbol(&bds_func.symbol),
                            bds_func.rva
                        )
                        .expect("ERROR: Could not write to file");
                    } else if file_type == ".hpp" {
                        write!(
                            dump_file,
                            "//{}\n//{}\nconstexpr unsigned int MD5_{:x} = {};\n\n",
                            &bds_func.symbol,
                            demangle::cleanup_symbol(&bds_func.symbol),
                            md5::compute(&bds_func.symbol),
                            bds_func.rva
                        )
                        .expect("ERROR: Could not write to file");
                    }
                }
                Err(str) => {
                    return Err(str);
                }
            }
        }

        nwg::simple_message(
            "Completed",
            &format!("Completed filtered dumping of {}", pdb_path),
        );
        Ok(())
    }
}
