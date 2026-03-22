# Changelog

All notable changes to the `peakrdl-rust` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

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
