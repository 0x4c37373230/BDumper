# BDumper
A windows BDS .pdb dumper written in rust which dumps symbols, reconstructed function prototypes from demangled symbols and RVAs (Relative Virtual Addresses) into either a C++ header (.hpp) or a text file (.txt) and can also find data corresponding to specific functions. This project was inspired by [Player]'s BDS .pdb dumper. The newest version has a GUI and can search and find specific functions while the older versions are CLI programs. The variables in the headers are named after the symbol's MD5 hashes.

## Dependencies

### Current version

- [md5 0.7.0](https://crates.io/crates/md5)
- [msvc_demangler](https://crates.io/crates/msvc-demangler)
- [native-windows-gui 1.0.12](https://crates.io/crates/native-windows-gui)
- [native-windows-derive 1.0.3](https://crates.io/crates/native-windows-derive)
- [pdb 0.7.0](https://crates.io/crates/pdb)

## Usage

### Current Version
<p align="center">
  <img src="https://media.discordapp.net/attachments/891760155614642277/912068785832333363/Screenshot_50.png" />
</p>
