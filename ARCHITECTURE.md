# PeakRDL-rust Architecture

The purpose of this file is to give a brief overview of the high-level architecture for contributors.

## File Map

* `exporter.py`: programmatic exporter API, main entrypoint for the rest of the code
* `__peakrdl__.py`: defines the PeakRDL plugin, handles command line options and calls exporter
* `templates/`: Jinja2 templates for the generated Rust crate
* `component_context.py`: defines python dataclasses for SystemRDL components, used as context for the Jinja2 templates. Includes scanner logic for generating
these component classes from the compiled SystemRDL design
* `generator.py`: copy files and render jinja templates to create the module
* `design_scanner.py`: scan through the RDL design to gather required information and check for unsupported constructs
