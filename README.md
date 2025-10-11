# PeakRDL-rust

Generate Rust code for accessing control/status registers from a SystemRDL description.

This is currently in an alpha state. Feel free to try it out and report any bugs
encountered.

## Installation

It can be installed from PyPI using

```bash
pip install peakrdl-rust[cli]
```

## Usage

For usage available options, use

```bash
peakrdl rust --help
```

## Documentation

For comprehensive documentation including API reference, configuration options, and detailed examples, visit:

**📖 [PeakRDL-rust Documentation on Read the Docs](https://peakrdl-rust.readthedocs.io/)**

The documentation includes:
- Getting started guide with examples
- Detailed description of generated Rust code
- Configuration options and customization
- Python API reference
- Best practices and advanced usage

## TODO

- [x] Arrays
- [x] Enum encoding
- [x] Reg impl with regwidth != accesswidth
- [x] Impl Default for registers
- [x] Test generator
- [x] Add field constants (width, offset, etc.)
- [x] Impl Debug for registers
- [x] Add ARCHITECTURE.md
- [x] Find/generate additional test input
- [x] Mem components
- [x] More efficient field tests
- [x] Set up github actions/PyPI publishing
- [x] Fixedpoint/signed UDPs
- [ ] Automatically run tests
- [ ] Regwidth > native integer types
- [ ] Rust keyword conflict test
