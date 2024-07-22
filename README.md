# tree-sitter-sleigh

Tree sitter parser for the Ghidra Sleigh language (read about it
[here](https://github.com/NationalSecurityAgency/ghidra/tree/master/GhidraDocs/languages/html)).

This is a raw parser, and doesn't semantically intepret the SLA language in any way
except to accurately parse it to a machine-usable structure. The intent behind this
project is to form the frontend of a SLA to Rust transpiler.

## Example

```rust
let language_path = std::path::PathBuf::from("../Processors/x86/data/languages/x86-64.sla");
let language_contents = std::fs::read_to_string(&language_path)?;
let parsed = tree_sitter_sleigh::parse(&language_contents)?;
println!("{:?}", parsed);
```

This will take a few minutes (the parser is not particularly fast, and these files are quite large, hence why this project is not really appropriate for use to repeatedly load SLA specifications). You'll eventually get some output like:

```
Sleigh {
    _open: (),
    version: Some(
        3,
    ),
    bigendian: false,
    align: 1,
    uniqbase: 1097856,
    maxdelay: None,
    uniqmask: None,
    numsections: None,
    _close: (),
    sourcefiles: SourceFiles {
        _start: (),
        source_files: [
            SourceFile {
                _start: (),
                name: "ia.sinc",
                index: 0,
                _end: (),
            },
            SourceFile {
                _start: (),
                name: "lockable.sinc",
                index: 1,
                _end: (),
            },
```

...and so on.