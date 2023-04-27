/// This checks if a file or directory exists
pub fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

/// This function will create the output file the dump will be written to
///
/// # Arguments
///
/// * `file_type`: Determines, well, the file type. It can be .hpp (for a C++ header) or .txt
/// for a text file
///
/// returns: Result<File, &str>
///
/// # Examples
///
/// ```
///
/// ```
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
        Err(_) => Err("Could not create file"),
    }
}
