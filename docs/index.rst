Introduction
============

PeakRDL-Rust is a Python package which can be used to generate Rust code
for accessing control/status registers from a SystemRDL definition.

Features:

* Generates Rust ``struct`` definitions and accessor methods for your hardware registers
* Preserves the hierarchical structure of SystemRDL
* Supports complex nested regfiles, arrays, and memory components
* Supports enumerated field types
* Supports signed and fixed-point field types
* Type-safe field access with compile-time guarantees
* Embedded-friendly code generation with ``no_std`` support
* Generates comprehensive test suites to validate register functionality
* Implements ``Debug`` and ``Default`` traits for generated register types


Installing
----------

Install from `PyPi`_ using pip

.. code-block:: bash

    python3 -m pip install peakrdl-rust[cli]

.. _PyPi: https://pypi.org/project/peakrdl-rust


Quick Start
-----------
The easiest way to use PeakRDL-rust is via the `PeakRDL command line tool <https://peakrdl.readthedocs.io/>`_:

.. code-block:: bash

    # Install the command line tool
    python3 -m pip install peakrdl

    # Generate a Rust crate in the example/ directory
    peakrdl rust example.rdl -o example/

Using the generated Rust code, you can access your device registers in a type-safe manner.
For example, the tool transforms this SystemRDL:

.. code-block:: systemrdl

    // TODO

Into a Rust crate you can use like:

.. code-block:: rust

    // TODO

For more in-depth examples, see TODO.


Key Benefits
------------

**Type Safety**
    All register and field accesses are checked at compile time, preventing
    common errors like writing to a read-only field or accessing non-existent registers or fields.

**Zero-Cost Abstractions**
    The generated code compiles down to direct memory accesses with no runtime overhead.

**Embedded Friendly**
    Compatible with ``no_std`` environments and embedded development workflows.

**Comprehensive Testing**
    Optionally generates test suites that validate register behavior and field access.

**Rich Metadata**
    Preserves SystemRDL documentation, field properties, and semantic information.


Getting Started
---------------

Ready to dive in? Here are the next steps:

1. :doc:`output` - Learn about the generated Rust code structure
2. :doc:`configuring` - Customize the code generation to your needs
3. :doc:`api` - Use PeakRDL-rust in your own Python scripts

For questions or issues, visit our `issue tracker <https://github.com/darsor/PeakRDL-rust/issues>`_.


Related Projects
----------------

PeakRDL-rust is part of the broader PeakRDL ecosystem:

- `PeakRDL CLI <https://peakrdl.readthedocs.io/>`__ - Command-line interface and project management
- `PeakRDL-regblock <https://github.com/SystemRDL/PeakRDL-regblock>`__ - Generate SystemVerilog RTL
- `PeakRDL-regblock-vhdl <https://github.com/SystemRDL/PeakRDL-regblock-vhdl>`__ - Generate VHDL RTL
- `PeakRDL-html <https://github.com/SystemRDL/PeakRDL-html>`__ - Generate HTML documentation
- `PeakRDL-cheader <https://github.com/SystemRDL/PeakRDL-cheader>`__ - Generate C header files
- `PeakRDL-ipxact <https://github.com/SystemRDL/PeakRDL-ipxact>`__ - Import/export IP-XACT XML

And many more `community plugins <https://peakrdl.readthedocs.io/en/latest/community.html>`_.

Links
-----

- `Source repository <https://github.com/darsor/PeakRDL-rust>`__
- `Releases <https://github.com/darsor/PeakRDL-rust/releases>`__
- `Changelog <https://github.com/darsor/PeakRDL-rust/blob/main/CHANGELOG.md>`__
- `Issue tracker <https://github.com/darsor/PeakRDL-rust/issues>`__
- `PyPi <https://pypi.org/project/peakrdl-rust>`__
- `SystemRDL Specification <https://accellera.org/downloads/standards/systemrdl>`__


.. toctree::
    :hidden:

    output
    configuring
    api
    licensing
    limitations

.. toctree::
    :hidden:
    :caption: Extended Properties

    udps/intro
    udps/signed
    udps/fixedpoint
