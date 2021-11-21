pub fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

pub fn create_file(file_type: &str) {
    match file_type {
        ".txt" => std::fs::File::create("./SymHook.txt").expect("ERROR: Could not create file"),
        ".hpp" => std::fs::File::create("SymHook.hpp").expect("ERROR: Could not create file"),
        _ => {
            nwg::simple_message("Error", &format!("Invalid file type: {}", file_type));
            return;
        }
    };
}
