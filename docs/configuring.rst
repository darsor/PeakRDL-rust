Configuring PeakRDL-rust
========================

If using the `PeakRDL command line tool <https://peakrdl.readthedocs.io/>`_,
some aspects of the ``rust`` command can be configured via the PeakRDL TOML
file. Any equivalent command-line options will always take precedence.

All Rust-specific options are defined under the ``[rust]`` TOML heading.

For example:

.. code-block:: toml

    [rust]
    crate_name = "my_registers"
    crate_version = "0.1.0"
    force = true
    no_fmt = false


.. data:: crate_name

    Specify the name of the generated Rust crate or module.

    If not provided, the crate name will be derived from the top-level
    SystemRDL component name using Rust naming conventions.


.. data:: crate_version

    Semantic version of the generated crate.

    Default: ``0.1.0``


.. data:: force

    Overwrite the output directory if it already exists.

    Default: ``false``


.. data:: no_fmt

    If true, don't attempt to format the generated rust code using
    ``cargo fmt``.

    Default: ``false``
