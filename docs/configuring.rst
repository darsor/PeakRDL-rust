Configuring PeakRDL-rust
========================

If using the `PeakRDL command line tool <https://peakrdl.readthedocs.io/>`_,
some aspects of the ``rust`` command can be configured via the PeakRDL TOML
file. Any equivalent command-line options will always take precedence.

All Rust-specific options are defined under the ``[rust]`` TOML heading.

For example:

.. code-block:: toml

    [rust]
    force = true
    fmt = true
    byte_endian = "big"
    word_endian = "little"
    access_mode = "software"


.. data:: force

    Overwrite the output directory if it already exists.

    Default: ``false``


.. data:: fmt

    If true, attempt to format the generated rust code using ``rustfmt``.

    Default: ``false``


.. data:: byte_endian

    Ordering of bytes within `accesswidth`-sized accesses to the register
    file. Valid options are ``big`` or ``little``. Overrides the `littleendian`
    and `bigendian` addrmap properties.

    Default: addrmap endianness propery, or ``little`` if not defined


.. data:: word_endian

    Ordering of `accesswidth`-sized words within a wide register. Valid options are
    ``big`` or ``little``. Overrides the `littleendian` and `bigendian` addrmap
    properties. Note that the PeakRDL regblock exporters only support ``little``
    word endianness.

    Default: addrmap endianness propery, or ``little`` if not defined


.. data:: access_mode

    Controls which access properties are used for register and field access
    determination.

    Options:

    - ``software`` - Use software read/write access properties (sw property)
    - ``hardware`` - Use hardware read/write access properties (hw property)
    - ``read_only`` - Force all registers and fields to be read-only

    Default: ``software``
