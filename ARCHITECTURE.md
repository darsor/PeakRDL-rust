# PeakRDL-rust Architecture

The purpose of this file is to give a brief overview of the high-level architecture for contributors.

## File Map

* `exporter.py`: programmatic exporter API, main entrypoint for the rest of the code
* `__peakrdl__.py`: defines the PeakRDL plugin, handles command line options and calls exporter
* `templates/`: Jinja2 templates for the generated Rust crate
* `component_context.py`: defines python dataclasses for SystemRDL components, used as context for the Jinja2 templates. Includes scanner logic for generating
these component classes from the compiled SystemRDL design
* `crate_generator.py`: copy files and render jinja templates to create the crate
* `design_scanner.py`: scan through the RDL design to gather required information and check for unsupported constructs
* `test_generator.py`: similar to `component_context.py` and `crate_generator.py`, but for automatically generated Rust integration tests

## Architecture Decisions

### Rust Code Structure

The generated Rust code is inspired by [chiptool](https://github.com/embassy-rs/chiptool) and adapted to work for SystemRDL. The chiptool README has a good explanation of some if its design decisions as a departure from [svd2rust](https://github.com/rust-embedded/svd2rust), which users may be more familiar with.

### SystemRDL -> Rust Mapping

The generated Rust code has a separate file/module for each SystemRDL component. This simplifies code generation and allows us to follow the hierarchical SystemRDL structure very closely.

Named SystemRDL types are placed in the hierarchy lexically (i.e., under the component they were declared in, not the one they were used in). The named type module is then publically re-exported in each component where it's used. This has a few benefits:
* Allows full reuse of definitive types
* Lets the user locate/use data structures by following the hierarchically instantiated component names rather than needing to know where the type was defined in the SystemRDL
* Avoids namespace collision, since SystemRDL has separate namespaces for type names and instance names.

SystemRDL type and instance names are re-cased to the standard Rust conventions in [RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md) (i.e., snake_case for modules and methods, UpperCamelCase for types and enum variants, etc.). This helps avoid namespace collision and follows what Rust users expect.
