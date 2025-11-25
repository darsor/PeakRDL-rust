# PeakRDL-rust

[![Coverage Status](https://coveralls.io/repos/github/darsor/PeakRDL-rust/badge.svg?branch=main)](https://coveralls.io/github/darsor/PeakRDL-rust?branch=main)

Generate Rust code for accessing control/status registers from a SystemRDL description.

This is currently in a beta state. Feel free to try it out and report any bugs encountered.

For documentation including API reference, configuration options, and detailed examples, visit:

**[PeakRDL-rust Documentation on Read the Docs](https://peakrdl-rust.readthedocs.io/)**

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
- [x] Rust keyword conflict test
- [x] Unwrap encoded field enums if exhaustive
- [x] Restrict read/write to memories
- [x] Add examples to docs
- [ ] Regwidth > native integer types
