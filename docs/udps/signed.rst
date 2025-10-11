.. _signed:

Signed Fields
=============

SystemRDL does not natively provide a way to mark fields as signed or unsigned.
The ``is_signed`` user-defined property fills this need.

For this Rust exporter, marking a field as signed changes the primitive integer
type of the field to a signed integer (for example, ``i16`` instead of ``u16``).
The getter method for the field returns a signed value, sign-extended to size
of the primitive type. The setter method takes a signed integer and truncates
any unused upper bits.

Properties
----------
A field can be marked as signed using the following user-defined property:

.. literalinclude:: ../../src/peakrdl_rust/udps/udps.rdl
    :lines: 10-14

This UDP definition, along with others supported by PeakRDL-regblock, can be
enabled by compiling the following file along with your design:
:download:`udps.rdl <../../src/peakrdl_rust/udps/udps.rdl>`.

.. describe:: is_signed

    *   Assigned value is a boolean.
    *   If true, the field's getter and setter methods will return/accept a
        signed Rust integer primitive.
    *   If false, the field's getter and setter methods will return/accept an
        unsigned Rust integer primitive.
    *   If not assigned, the field's getter and setter methods will return the
        appropriate type (bool, enum, or unsigned integer). In this case the
        field value is not considered numeric.

Other Rules
^^^^^^^^^^^

*   ``is_signed=true`` is mutually exclusive with the ``counter`` property.
*   ``is_signed=true`` is mutually exclusive with the ``encode`` property.

Examples
--------
Below are some examples of fields with different signedness.

Signed Fields
^^^^^^^^^^^^^
.. code-block:: systemrdl
    :emphasize-lines: 3, 8

    field {
        sw=rw; hw=r;
        is_signed;
    } signed_num[63:0] = 0;

    field {
        sw=r; hw=w;
        is_signed = true;
    } another_signed_num[19:0] = 20'hFFFFF; // -1

SystemRDL's own integer type is always unsigned. In order to specify a negative
reset value, the two's complement value must be used as shown in the second
example above.

Unsigned Fields
^^^^^^^^^^^^^^^
.. code-block:: systemrdl
    :emphasize-lines: 3

    field {
        sw=rw; hw=r;
        is_signed = false;
    } unsigned_num[63:0] = 0;
