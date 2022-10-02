fn main() {
    cc::Build::new()
        .file("lib/demangling.c")
        .compile("demangling");
}
