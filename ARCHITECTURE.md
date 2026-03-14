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
