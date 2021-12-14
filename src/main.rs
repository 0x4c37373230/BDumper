#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use {nwd::NwgUi, nwg::NativeUi, bedrock_dumper::*};

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

    #[nwg_control(text: "Dump Data", size: (280, 30), position: (10, 180))]
    #[nwg_events( OnButtonClick: [BedrockDumper::dump] )]
    dump: nwg::Button,

    #[nwg_control(text: "Find Function", size: (280, 30), position: (300, 180))]
    #[nwg_events( OnButtonClick: [BedrockDumper::find] )]
    find: nwg::Button,
}

impl BedrockDumper {
    fn dump(&self) {
        let pdb_path: &str = &self.pdb_path.text();
        let file_type: &str = &self.file_type.text();

        if files::path_exists(&pdb_path) == false {
            nwg::simple_message("Error", &format!("File does not exist: {}", pdb_path));
            return;
        }

        let dump_file = match files::create_file(&file_type) {
            Ok(file) => file,
            Err(str) => {
                nwg::simple_message("Error", &format!("{}: {}", str, file_type));
                return;
            }
        };

        pdb::pdb_dump(&pdb_path, file_type, dump_file).expect("ERROR: Failed to dump pdb contents");
    }

    fn find(&self) {
        pdb::find_function(&self.pdb_path.text(), &self.func_name.text())
            .expect("ERROR: Failed to dump pdb contents or find function");
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
