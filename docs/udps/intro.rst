Introduction
============

Although the official SystemRDL spec defines numerous properties that allow you
to define complex register map structures, sometimes they are not enough to
accurately describe a necessary feature. Fortunately the SystemRDL spec allows
the language to be extended using "User Defined Properties" (UDPs). The
PeakRDL-regblock-vhdl tool understands several UDPs that are described in this
section.

To enable these UDPs, compile this RDL file prior to the rest of your design:
:download:`udps.rdl <../../src/peakrdl_rust/udps/udps.rdl>`.

.. list-table:: Summary of UDPs
    :header-rows: 1

    *   - Name
        - Component
        - Type
        - Description

    *   - is_signed
        - field
        - boolean
        - Defines the signedness of a field.

          See: :ref:`signed`.

    *   - intwidth
        - field
        - unsigned integer
        - Defines the number of integer bits in the fixed-point representation
          of a field.

          See: :ref:`fixedpoint`.

    *   - fracwidth
        - field
        - unsigned integer
        - Defines the number of fractional bits in the fixed-point representation
          of a field.

          See: :ref:`fixedpoint`.
