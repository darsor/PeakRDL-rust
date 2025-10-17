Examples
========

This section includes an example RDL file, the auto-generated ``cargo doc``
documentation for the exported crate, and example usage.

.. contents:: Contents
      :local:
      :depth: 2

Example RDL
-----------

File: :download:`turboencabulator.rdl <examples/turboencabulator.rdl>`.

Exported with ``peakrdl rust -o output/ --crate-name turboencabulator udps.rdl turboencabulator.rdl``

Cargo docs for the exported turboencabulator crate can be viewed
here: `turboencabulator docs <examples/turboencabulator/index.html>`_.
Click on the source button in the docs to see the generated source for any module.


Example Usage
-------------

This section contains examples for common tasks and features that PeakRDL-rust
supports.

Note that many examples contain type annotations for clarity. These annotations
can typically be omitted in normal use.

Reading a Register
^^^^^^^^^^^^^^^^^^

.. literalinclude:: examples/turboencabulator.rs
   :language: rust
   :start-after: test_read() {
   :end-before: } // test-read
   :dedent: 4

Links:

* Docs for `registers <output.html#registers>`__
* Cargo docs for the top-level `TurboEncab <examples/turboencabulator/components/turbo_encab/index.html>`__ addrmap type
* Cargo docs for the `Status <examples/turboencabulator/components/turbo_encab/status/struct.Status.html>`__ register type
* Cargo docs for the `Reg <examples/turboencabulator/reg/struct.Reg.html>`__ type
* Cargo docs for the `access <examples/turboencabulator/access/index.html>`__ module

Writing a Register
^^^^^^^^^^^^^^^^^^

.. literalinclude:: examples/turboencabulator.rs
   :language: rust
   :start-after: test_write() {
   :end-before: } // test-write
   :dedent: 4

Links:

* Docs for `registers <output.html#registers>`__
* Cargo docs for the top-level `TurboEncab <examples/turboencabulator/components/turbo_encab/index.html>`__ addrmap type
* Cargo docs for the `Ctrl <examples/turboencabulator/components/turbo_encab/ctrl/struct.Ctrl.html>`__ register type
* Cargo docs for the `Reg <examples/turboencabulator/reg/struct.Reg.html>`__ type
* Cargo docs for the `access <examples/turboencabulator/access/index.html>`__ module

Modifying a Register
^^^^^^^^^^^^^^^^^^^^

.. literalinclude:: examples/turboencabulator.rs
   :language: rust
   :start-after: // test-modify-example
   :end-before: } // test-modify
   :dedent: 4

Links:

* Cargo docs for the top-level `TurboEncab <examples/turboencabulator/components/turbo_encab/index.html>`__ addrmap type
* Cargo docs for the `Ctrl <examples/turboencabulator/components/turbo_encab/ctrl/struct.Ctrl.html>`__ register type
* Cargo docs for the `Reg <examples/turboencabulator/reg/struct.Reg.html>`__ type
* Cargo docs for the `access <examples/turboencabulator/access/index.html>`__ module

Arrays of Components
^^^^^^^^^^^^^^^^^^^^

.. literalinclude:: examples/turboencabulator.rs
   :language: rust
   :start-after: test_array() {
   :end-before: } // test-array
   :dedent: 4

Links:

* Docs for `arrays <output.html#arrays>`__
* Cargo docs for the top-level `TurboEncab <examples/turboencabulator/components/turbo_encab/index.html>`__ addrmap type
* Cargo docs for the `Grammeter <examples/turboencabulator/components/turbo_encab/grammeter/struct.Grammeter.html>`__ regfile type

Enum-Encoded Fields
^^^^^^^^^^^^^^^^^^^

The Turbo Encabulator SystemRDL contains a ``state`` field with a defined encoding:

.. rst-class:: scrollable-code

.. literalinclude:: examples/turboencabulator.rdl
   :language: systemrdl
   :start-after: // start-enum-example
   :end-before: // end-enum-example
   :dedent: 12

PeakRDL-rust translates this encoding into an ``enum`` type that can be
used as follows:

.. literalinclude:: examples/turboencabulator.rs
   :language: rust
   :start-after: test_enum() {
   :end-before: } // test-enum
   :dedent: 4

Links:

* Cargo docs for the `Status <examples/turboencabulator/components/turbo_encab/grammeter/status/struct.Status.html>`__ register type
* Cargo docs for `GrammeterStateE <examples/turboencabulator/components/turbo_encab/grammeter/status/state/enum.GrammeterStateE.html>`__ field type
* Cargo docs for the `UnknownVariant <examples/turboencabulator/encode/struct.UnknownVariant.html>`__ type

Accessing a Memory
^^^^^^^^^^^^^^^^^^

SystemRDL memories implement the ``Memory`` trait.

.. literalinclude:: examples/turboencabulator.rs
   :language: rust
   :start-after: test_memory() {
   :end-before: } // test-memory
   :dedent: 4

Virtual registers instantiated within memories are fully supported.

Links:

* Cargo docs for the `Measurements <examples/turboencabulator/components/turbo_encab/measurements/struct.Measurements.html>`__ memory type
* Cargo docs for the `Memory <examples/turboencabulator/mem/trait.Memory.html>`__ trait
* Cargo docs for the `MemEntry <examples/turboencabulator/mem/struct.MemEntry.html>`__ type
* Cargo docs for the `access <examples/turboencabulator/access/index.html>`__ module

Fixedpoint Fields
^^^^^^^^^^^^^^^^^

.. literalinclude:: examples/turboencabulator.rs
   :language: rust
   :start-after: test_fixedpoint() {
   :end-before: } // test-fixedpoint
   :dedent: 4

Links:

* Docs for `Fixed-Point Fields <udps/fixedpoint.html>`__
* Cargo docs for the `FixedPoint <examples/turboencabulator/fixedpoint/struct.FixedPoint.html>`__ type
* Cargo docs for the `Status <examples/turboencabulator/components/turbo_encab/grammeter/status/index.html>`__ register type
