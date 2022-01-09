#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use {bedrock_dumper::*, nwd::NwgUi, nwg::NativeUi};

#[derive(Default, NwgUi)]
pub struct BedrockDumper {
    #[nwg_control(size: (590, 225), position: (300, 300), title: "BDumper", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BedrockDumper::exit_program] )]
    window: nwg::Window,

    #[nwg_control(text: "BDumper is a .pdb file dumper made in Rust by Luke7720 designed to extract \
         function prototypes and RVAs (Relative Virtual Addresses) and export them into either text \
         or C++ header files. It can also find specific functions within the pdb\n\
         -----------------------------------------------------------------------\
         -----------------------------------------------------------------------
         ", size: (580, 70), position: (10, 10))]
    label: nwg::Label,

    #[nwg_control(text: "Input your .pdb file path here", size: (280, 25), position: (10, 80))]
    label2: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (10, 100))]
    pdb_path: nwg::TextInput,

    #[nwg_control(text: "Input your file type (.hpp or .txt) here", size: (280, 25), position: (300, 80))]
    label3: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (300, 100))]
    file_type: nwg::TextInput,

    #[nwg_control(text: "Input a function's name here", size: (280, 25), position: (10, 130))]
    label4: nwg::Label,

    #[nwg_control(text: "", size: (570, 25), position: (10, 150))]
    func_name: nwg::TextInput,

    #[nwg_control(text: "Dump Data", size: (185, 30), position: (10, 180))]
    #[nwg_events( OnButtonClick: [BedrockDumper::dump] )]
    dump: nwg::Button,

    #[nwg_control(text: "Find Function", size: (185, 30), position: (200, 180))]
    #[nwg_events( OnButtonClick: [BedrockDumper::find] )]
    find: nwg::Button,

    #[nwg_control(text: "Filtered Dump", size: (185, 30), position: (390, 180))]
    #[nwg_events( OnButtonClick: [BedrockDumper::filtered_dump] )]
    find_filtered: nwg::Button,
}

impl BedrockDumper {
    fn dump(&self) {
        let pdb_path: &str = &self.pdb_path.text();
        let file_type: &str = &self.file_type.text();

        match setup::dump_init(pdb_path, file_type) {
            Ok(dump_file) => pdb::pdb_dump(pdb_path, file_type, dump_file)
                .expect("ERROR: Failed to dump pdb contents"),
            Err(str) => {
                nwg::simple_message("Error", &str);
                return;
            }
        }
    }

    fn find(&self) {
        match pdb::find_function(&self.pdb_path.text(), &self.func_name.text()) {
            Ok(bds_func) => nwg::simple_message(
                "Found a match",
                &format!(
                    "Function name: {}\nSymbol: {}\nRVA: {}",
                    bds_func.name, bds_func.symbol, bds_func.rva
                ),
            ),
            Err(str) => nwg::simple_message("Error", &str),
        };
    }

    fn filtered_dump(&self) {
        let pdb_path: &str = &self.pdb_path.text();
        let file_type: &str = &self.file_type.text();

        match setup::dump_init(pdb_path, file_type) {
            Ok(dump_file) => {
                match pdb::find_functions(pdb_path, file_type, dump_file) {
                    Err(str) => {
                        nwg::simple_message("Error", &str);
                    }
                    _ => {}
                }
            },
            Err(str) => {
                nwg::simple_message("Error", &str);
            }
        }
    }

    fn exit_program(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    setup::filter_manager();

    let _app = BedrockDumper::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
