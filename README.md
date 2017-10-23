Padded Cell [![Crates.io](https://img.shields.io/crates/v/pcell.svg)](https://crates.io/crates/pcell)
===
Description
---
A small rust tool for storing malware on any system safely.

Samples are compressed using the snap compression algorithm and then base64 encoded and saved to a local file based database (in TOML format).

This is still a toy and may break occasionally. I will personally use it for my samples.

Large databases make the program slower as they have to be read to memory on each use.

Usage
---

```
USAGE:
    pcell [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -x, --export <export>    Exports the given sample from the database
    -i, --import <import>    Imports a sample into the database
    -l, --list <list>        Lists all samples in database or the listing for the filename given.
```

Implementation Checklist
---
- [x] Importing
- [x] Exporting
- [x] Listing
- [x] Encoding
- [x] Compressing
- [x] md5
- [ ] sha256
