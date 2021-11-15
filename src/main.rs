#![windows_subsystem = "windows"]

mod pdb;
mod files;

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use {nwd::NwgUi, nwg::NativeUi};

#[derive(Default, NwgUi)]
pub struct BedrockDumper {
    #[nwg_control(size: (300, 260), position: (300, 300), title: "BDumper", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BedrockDumper::exit_program] )]
    window: nwg::Window,

    #[nwg_control(text: "BDumper is a .pdb file dumper made in Rust by Luke7720 designed to extract \
         function prototypes and RVAs (Relative Virtual Addresses) and \
         export them into either text or C++ header files\n\
         -----------------------------------------------------------------------
         ", size: (280, 100), position: (10, 10))]
    label: nwg::Label,

    #[nwg_control(text: "Input your .pdb file path here", size: (280, 25), position: (10, 110))]
    label2: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (10, 130))]
    pdb_path: nwg::TextInput,

    #[nwg_control(text: "Input your file type (.hpp or .txt) here", size: (280, 25), position: (10, 160))]
    label3: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (10, 180))]
    file_type: nwg::TextInput,

    #[nwg_control(text: "Dump Data", size: (280, 30), position: (10, 210))]
    #[nwg_events( OnButtonClick: [BedrockDumper::dump] )]
    dump: nwg::Button,
}

impl BedrockDumper {
    fn dump(&self) {
        let pdb_path: &str = &self.pdb_path.text();
        let file_type: &str = &self.file_type.text();

        if files::path_exists(&pdb_path) == false {
            nwg::simple_message("Error", &format!("File does not exist: {}", pdb_path));
            return;
        }
        files::create_file(&file_type);

        std::fs::File::create("./temp.txt").expect("ERROR: Could not create file");

        let mut dump_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./temp.txt")
            .unwrap();

        match file_type {
            ".txt" => dump_file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("./SymHook.txt")
                .unwrap(),
            ".hpp" => dump_file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("./SymHook.hpp")
                .unwrap(),
            _ => {}
        };

        std::fs::remove_file("./temp.txt").expect("ERROR: Could not remove file");

        pdb::pdb_dump(&pdb_path, file_type, dump_file).expect("ERROR: Failed to dump pdb contents");
        nwg::simple_message("Completed", &format!("Completed dumping {}", pdb_path));
    }

    fn exit_program(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BedrockDumper::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
