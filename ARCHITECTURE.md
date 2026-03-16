# PeakRDL-rust Architecture

The purpose of this file is to give a brief overview of the high-level architecture for contributors.

## Python Package File Map

* `exporter.py`: programmatic exporter API, main entrypoint for the rest of the code
* `__peakrdl__.py`: defines the PeakRDL plugin, handles command line options and calls exporter
* `templates/`: Jinja2 templates for the generated Rust crate
* `component_context.py`: defines python dataclasses for SystemRDL components, used as context for the Jinja2 templates. Includes scanner logic for generating
these component classes from the compiled SystemRDL design
* `generator.py`: copy files and render jinja templates to create the module
* `design_scanner.py`: scan through the RDL design to gather required information and check for unsupported constructs

## Rust Crates

* `peakrdl-rust`: common types and traits implemented by the generated code, published to crates.io
* `peakrdl-rust-build`: build-script helper that calls the python exporter to generate code
* `smoke-test`: post-publication smoke test to ensure it works by downloading the generated python binary and generating code with it

## Pre-Publication Checklist

Version numbers:
* Exporter version: keep in sync and add git tag, published via GitHub CI
  * `pyproject.toml`
  * `uv.lock`
  * `peakrdl-rust-build/Cargo.toml`
  * `Cargo.lock`
  * `CHANGELOG.md`
  * `uv_peakrdl_rust.sh`
  * `peakrdl-rust-build/README.md`
* `peakrdl-rust` crate dependency version, manually published and versioned separately
  * `peakrdl-rust/Cargo.toml`
  * `peakrdl-rust/CHANGELOG.md`
  * `Cargo.lock`
  * smoke-test `Cargo.toml`
  * PEAKRDL_RUST_CRATE_MIN_VERSION in `__init__.py`
  * `peakrdl-rust-build/README.md`

## Driver API

Options:
1. `Driver` generic on all types, similar to Rust allocator API
2. Marker trait similar to `Access` which turns off implementation of default read/write functions, and enables custom implementations via user trait

`Driver` generic would require some basic read/write methods for accesswidth accesses.
Marker trait method would allow user to specify their own custom API.

Both methods require a generic on all types and could use a default generic type to simplify API in the common case.
Both methods allow multiple instantiations of the same register files using different drivers.

A tunneled interface introduces failure points. Volatile memory reads/writes are expected to always succeed.
Don't want users to have to unwrap/handle `Result`s in the common case.
 * Marker traits handle this by allowing the user to define their own read/write API
 * Driver implementation
 
Could simplify things for `Driver` implementers with a secondary trait (`DriverAccess`?) that just implements
simple peeks/pokes. Then a blanket `Driver` implementation for anything that implements `DriverAccess`.
Then user doesn't have to worry about accesswidth, endianness, etc.
