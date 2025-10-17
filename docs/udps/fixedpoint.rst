.. _fixedpoint:

Fixed-Point Fields
==================

`Fixed-point <https://en.wikipedia.org/wiki/Fixed-point_arithmetic>`_ numbers
can be used to efficiently represent real numbers using integers. Fixed-point
numbers consist of some combination of integer bits and fractional bits. The
number of integer/fractional bits is usually implicitly tracked (not stored)
for each number, unlike for floating-point numbers.

For this Rust exporter, the number of integer/fractional bits is tracked by
the type system using const generics on the ``FixedPoint`` type.

Properties
----------
Fields can be declared as fixed-point numbers using the following two properties:

.. literalinclude:: ../../src/peakrdl_rust/udps/udps.rdl
    :lines: 16-24

The :ref:`is_signed<signed>` property can be used in conjunction with these
properties to declare signed fixed-point fields.

These UDP definitions, along with others supported by PeakRDL-regblock, can be
enabled by compiling the following file along with your design:
:download:`udps.rdl <../../src/peakrdl_rust/udps/udps.rdl>`.

.. describe:: intwidth

    *   The ``intwidth`` property defines the number of integer bits in the
        fixed-point representation (including the sign bit, if present).

.. describe:: fracwidth

    *   The ``fracwidth`` property defines the number of fractional bits in the
        fixed-point representation.

Representable Numbers
^^^^^^^^^^^^^^^^^^^^^

The range of representable real numbers is summarized in the table below.

.. list-table:: Representable Numbers
    :header-rows: 1

    *   - Signedness
        - Minimum Value
        - Maximum Value
        - Step Size

    *   - Unsigned
        - :math:`0`
        - :math:`2^{\mathrm{intwidth}} - 2^{-\mathrm{fracwidth}}`
        - :math:`2^{-\mathrm{fracwidth}}`

    *   - Signed
        - :math:`-2^{\mathrm{intwidth}-1}`
        - :math:`2^{\mathrm{intwidth}-1} - 2^{-\mathrm{fracwidth}}`
        - :math:`2^{-\mathrm{fracwidth}}`

Rust Types
^^^^^^^^^^

The exporter generates a ``FixedPoint`` type with const generics for the number
of integer bits and fractional bits. This type is returned/accepted by the
field's getter and setter methods. The purpose of this type is to enforce type
safety (i.e., not accidentally setting the field to a normal integer value) and
provide convenient conversion functions between float types and fixed-point numbers.

The `fixed`_ crate is not used because it has two limitations with respect to
SystemRDL:

.. _fixed: https://crates.io/crates/fixed

* The bit width of the fixed-point type is limited to the widths of Rust
  integer primitives.
* A negative number of integer or fractional bits is not allowed.

Conversion functions between ``FixedPoint`` and the types provided by the
``fixed`` crate may be implemented in the future.

Other Rules
^^^^^^^^^^^
*   Only one of ``intwidth`` or ``fracwidth`` need be defined. The other is
    inferred from the field bit width.
*   The bit width of the field shall be equal to ``intwidth`` + ``fracwidth``.
*   If both ``intwidth`` and ``fracwidth`` are defined for a field,  it is an
    error if their sum does not equal the bit width of the field.
*   Either ``fracwidth`` or ``intwidth`` can be a negative integer. Because
    SystemRDL does not have a signed integer type, the only way to achieve
    this is to define one of the widths as larger than the bit width of the
    component so that the other width is inferred as a negative number.
*   The properties defined above are mutually exclusive with the ``counter``
    property.
*   The properties defined above are mutually exclusive with the ``encode``
    property.

Examples
--------

A 12-bit signed fixed-point field with 4 integer bits and 8 fractional bits
can be declared with

.. code-block:: systemrdl
    :emphasize-lines: 3, 4

    field {
        sw=rw; hw=r;
        intwidth = 4;
        is_signed;
    } fixedpoint_num[11:0] = 0;

This field can represent values from -8.0 to 7.99609375
in steps of 0.00390625.
