# BDumper
A windows BDS .pdb dumper written in rust which dumps symbols, reconstructed function prototypes from demangled symbols and RVAs (Relative Virtual Addresses) into either a C++ header (.hpp) or a text file (.txt) and can also find data corresponding to specific functions. This project was inspired by [Player]'s BDS .pdb dumper. The newest version has a GUI and can search and find specific functions while the older versions are CLI programs. The variables in the headers are named after the symbol's MD5 hashes.

## Dependencies

### Current version

- [md5 0.7.0](https://crates.io/crates/md5)
- [native-windows-gui 1.0.12](https://crates.io/crates/native-windows-gui)
- [native-windows-derive 1.0.3](https://crates.io/crates/native-windows-derive)
- [pdb 0.7.0](https://crates.io/crates/pdb)
- [cc-rs 1.0](https://crates.io/crates/cc)

## Usage

### Current Version UI
<p align="center">
  <img src="https://media.discordapp.net/attachments/891760155614642277/929838892495040553/Screenshot_70.png" />
</p>

### Full Dump

Input the pdb file path and the file type and then click the 'Dump Data' button which will create a SymHook file in the project/executable directory. The latter can be either '.txt' or '.hpp'. The C++ header mode uses the symbol md5 hashes as variable names

### Filtered Dump

Upon startup for the first time, BDumper creates a file with the name 'dumpFilter.txt'. This file will hold the functions to be dumped in this mode. BDumper will ignore lines starting with '#' (which are considered comments) and newlines. An example:

```
# Dump Filter Example
# This is a comment, BDumper ignores lines starting with '#' and empty lines

OceanMonumentFeature::isFeatureChunk
FossilFeature::place
PistonBlock::canSurvive
PistonBlockActor::_attachedBlockWalker
BlockSource::getSeenPercent
```

Click on the 'Filtered Dump' button to dump the desired functions.

### Function Search

If you need to quickly hook into a function or update an RVA, this function can find an individual function with the same method the filtered dump uses. Input the function name (`PistonBlockActor::_checkAttachedBlocks` as an example) and click on the 'FInd Function' button. A window will pop up with the symbol and RVA if the function exists
