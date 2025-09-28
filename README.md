# PeakRDL-rust

Generate Rust code for accessing control/status registers from a SystemRDL description.

This is a work in progress. Check back later.

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
- [ ] Automatically run tests
- [ ] Regwidth > native integer types
- [ ] Fixedpoint/signed UDPs
