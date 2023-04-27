use crate::demangle;
use std::ffi::{CStr, CString};

/// A wrapper for the C demangling function in order to isolate the unsafe code
///
/// # Arguments
///
/// * `symbol`: A reference to a string that contains the MSVC symbol to demangle
///
/// returns: String
///
/// # Examples
///
/// ```
///
/// ```
pub fn undecorate(symbol: &str) -> String {
    unsafe {
        let cstr = CString::new(symbol).unwrap();
        let result: &CStr = CStr::from_ptr(demangle(cstr.as_ptr()));

        return result.to_str().unwrap().to_owned();
    }
}

/// Formats already demangled symbols to make the output nicer and more readable
///
/// # Arguments
///
/// * `symbol`: Demangled function prototype
///
/// returns: String
///
/// # Examples
///
/// ```
///
/// ```
pub fn cleanup_symbol(symbol: &str) -> String {
    let res = undecorate(symbol);
    let demangled_name = res.replace("const", " const").replace("(", "( ");
    let mut declaration: Vec<&str> = demangled_name.split(" ").collect();

    for i in 0..declaration.len() {
        if &declaration[i] as &str == "const" && declaration[i - 1].starts_with("__") && i != 0 {
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
