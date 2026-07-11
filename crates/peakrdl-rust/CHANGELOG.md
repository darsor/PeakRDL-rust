# Changelog

All notable changes to the `peakrdl-rust` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Changed

- `RegisterIO::try_read_register`/`try_write_register` now take a single-word
  fast path (`num_subwords == 1`). For single-word registers — the common case —
  the access folds to a single volatile load/store instead of the generic
  subword loop (`map`/`try_fold` and a `for` loop). `num_subwords` is a
  compile-time constant, so the multi-word loop is dropped entirely for
  single-word registers; it is retained as a fallback for multi-word registers,
  so behavior is unchanged. (Measured ~10% `.text` reduction on a register-heavy
  RISC-V `-Oz` firmware.)

## [0.2.1] - 2026-03-23

### Fixed

- Added `#![no_std]` for proper building when included in `no_std` projects.

## [0.2.0] - 2026-03-22

### Changed

- `Access` is now an associated type of the `Register` trait, so `Access` is no longer a direct generic of the `Reg` type
- Split the `Endian` associated type of the `Register` trait into `ByteEndian` and `WordEndian`
- `Reg` is now generic over a `RegisterIO` type, allowing custom register access implementations

### Added

- `RegisterIO` and `RawRegisterIO` traits
- Fallible `try_read`, `try_write`, `try_modify` methods on the `Reg` type

## [0.1.1] - 2026-03-13

### Fixed

- Broken links in README

## [0.1.0] - 2026-03-11

Initial Release
