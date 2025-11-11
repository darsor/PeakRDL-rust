# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Clippy exceptions for common but harmless auto-generated code lints

### Changed

- Components named `mod` (a rust keyword) are now escaped as `mod_` instead of `r#mod` to avoid generating files named `mod.rs`
- Rename memory component `len()` method to `num_entries()`
- `UnknownVariant<T>` is now a public tuple struct

## [0.3.0] - 2025-11-10

### Added

- Memories now have an access attribute to restrict reads/writes for write-only
  and read-only memories

### Changed

- Little-endian accesses follow industry standards rather than SystemRDL
  spec (see the [errata](https://systemrdl-compiler.readthedocs.io/en/latest/dev_notes/rdl_spec_errata.html#byte-ordering-example-of-littleendian-mode-in-17-3-2-is-incorrect))
- Getters for enum-encoded fields return a `Result` instead of an `Option`,
  with the error variant containing the unknown field value.
- Getters for enum-encoded fields unwrap the returned `Result` if every bit
  pattern is represented
- Register methods for getting and setting fields are no longer `const`
- `Debug` impl for registers no longer prints the values of write-only fields

### Fixed

- Several instances of improper or missing rust keyword escaping
- Enum-encoded field `bits()` method returned `u8` instead of the field's
  primitive type

## [0.2.2] - 2025-10-15

### Fixed

- Fixed-point setter was generated for all fields, even if not writeable

## [0.2.1] - 2025-10-11

### Added

- Project backlinks for PyPI (#1, #2)
- Tests for Python 3.14

## [0.2.0] - 2025-10-10

### Added

- Support for is_signed, intwidth, and fracwidth UDPs
- Coveralls test coverage reporting

## [0.1.0] - 2025-09-28

Initial Release
