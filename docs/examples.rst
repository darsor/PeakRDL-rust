Examples
========

This section includes an example RDL file, the auto-generated ``cargo doc``
documentation for the exported module, and example usage.

.. contents:: Contents
      :local:
      :depth: 2

Example RDL
-----------

Files: :download:`turboencabulator.rdl <../tests/rdl_src/turboencabulator.rdl>`
and :download:`udps.rdl <../src/peakrdl_rust/udps/udps.rdl>`.

The turboencabulator code was exported using

.. code-block:: rust

    // in build.rs
    Generator::new()
        .rdl_file("udps.rdl")
        .rdl_file("turboencabulator.rdl")
        .top("turbo_encab")
        .format_output(true)
        .generate();

Cargo docs for the exported turboencabulator module can be viewed
here: `turboencabulator docs <examples/turboencabulator/index.html>`_.
Click on the source button in the docs to see the generated source for any module.

Note that the generated code relies on the `peakrdl-rust <https://crates.io/crates/peakrdl-rust>`__ crate on crates.io.


Example Usage
-------------

This section contains examples for common tasks and features that PeakRDL-rust
supports.

Note that many examples contain type annotations for clarity. These annotations
can typically be omitted in normal use.

Reading a Register
^^^^^^^^^^^^^^^^^^

.. literalinclude:: ../tests/rdl_src/turboencabulator.rs
   :language: rust
   :start-after: test_read() {
   :end-before: } // test-read
   :dedent: 4

Links:

* Docs for `registers <output.html#registers>`__
* Cargo docs for the top-level `TurboEncab <examples/turboencabulator/components/turbo_encab/index.html>`__ addrmap type
* Cargo docs for the `Status <examples/turboencabulator/components/turbo_encab/status/struct.Status.html>`__ register type
* Cargo docs for the `Reg <examples/peakrdl_rust/reg/struct.Reg.html>`__ type

Writing a Register
^^^^^^^^^^^^^^^^^^

.. literalinclude:: ../tests/rdl_src/turboencabulator.rs
   :language: rust
   :start-after: test_write() {
   :end-before: } // test-write
   :dedent: 4

Links:

* Docs for `registers <output.html#registers>`__
* Cargo docs for the top-level `TurboEncab <examples/turboencabulator/components/turbo_encab/index.html>`__ addrmap type
* Cargo docs for the `Ctrl <examples/turboencabulator/components/turbo_encab/ctrl/struct.Ctrl.html>`__ register type
* Cargo docs for the `Reg <examples/peakrdl_rust/reg/struct.Reg.html>`__ type

Modifying a Register
^^^^^^^^^^^^^^^^^^^^

.. literalinclude:: ../tests/rdl_src/turboencabulator.rs
   :language: rust
   :start-after: // test-modify-example
   :end-before: } // test-modify
   :dedent: 4

Links:

* Cargo docs for the top-level `TurboEncab <examples/turboencabulator/components/turbo_encab/index.html>`__ addrmap type
* Cargo docs for the `Ctrl <examples/turboencabulator/components/turbo_encab/ctrl/struct.Ctrl.html>`__ register type
* Cargo docs for the `Reg <examples/peakrdl_rust/reg/struct.Reg.html>`__ type

Arrays of Components
^^^^^^^^^^^^^^^^^^^^

.. literalinclude:: ../tests/rdl_src/turboencabulator.rs
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

.. literalinclude:: ../tests/rdl_src/turboencabulator.rdl
   :language: systemrdl
   :start-after: // start-enum-example
   :end-before: // end-enum-example
   :dedent: 12

PeakRDL-rust translates this encoding into an ``enum`` type that can be
used as follows:

.. literalinclude:: ../tests/rdl_src/turboencabulator.rs
   :language: rust
   :start-after: test_enum() {
   :end-before: } // test-enum
   :dedent: 4

Links:

* Cargo docs for the `Status <examples/turboencabulator/components/turbo_encab/grammeter/status/struct.Status.html>`__ register type
* Cargo docs for `GrammeterStateE <examples/turboencabulator/components/turbo_encab/grammeter/status/state/enum.GrammeterStateE.html>`__ field type
* Cargo docs for the `UnknownVariant <examples/peakrdl_rust/encode/struct.UnknownVariant.html>`__ type

Accessing a Memory
^^^^^^^^^^^^^^^^^^

SystemRDL memories implement the ``Memory`` trait.

.. literalinclude:: ../tests/rdl_src/turboencabulator.rs
   :language: rust
   :start-after: test_memory() {
   :end-before: } // test-memory
   :dedent: 4

Virtual registers instantiated within memories are fully supported.

Links:

* Cargo docs for the `Measurements <examples/turboencabulator/components/turbo_encab/measurements/struct.Measurements.html>`__ memory type
* Cargo docs for the `Memory <examples/peakrdl_rust/mem/trait.Memory.html>`__ trait
* Cargo docs for the `MemEntry <examples/peakrdl_rust/mem/struct.MemEntry.html>`__ type

Fixedpoint Fields
^^^^^^^^^^^^^^^^^

.. literalinclude:: ../tests/rdl_src/turboencabulator.rs
   :language: rust
   :start-after: test_fixedpoint() {
   :end-before: } // test-fixedpoint
   :dedent: 4

Links:

* Docs for `Fixed-Point Fields <udps/fixedpoint.html>`__
* Cargo docs for the `FixedPoint <examples/peakrdl_rust/fixedpoint/struct.FixedPoint.html>`__ type
* Cargo docs for the `Status <examples/turboencabulator/components/turbo_encab/grammeter/status/index.html>`__ register type

Advanced: Tunneled Registers
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Sometimes the registers to access are located on another device through a tunneled interface.
For example, a chip with internal registers that is accessed over a SPI bus. PeakRDL-rust supports
this by allowing the user to define a custom ``RegisterIO`` implementation.

.. code-block:: rust

    // ZST implementing the register accesses over SPI.
    // This can use a globally accessible SPI interface, or it can own the
    // interface. It is passed around by shared reference.
    struct SpiRegisterIO;

    // Implementers of the `RawRegisterIO` trait automatically implement
    // the `RegisterIO` trait. The blanket implementation takes care of
    // common details like accesswidth and endianness, so we don't have to
    // worry about that here.
    impl peakrdl_rust::io::RawRegisterIO for SpiRegisterIO {
        // Some SPI error type defined externally. If the register access is
        // infallible, set this to `core::convert::Infallible` and the `Reg`
        // type will expose infallible access methods (e.g., `modify` in addition
        // to `try_modify`).
        type Error = SpiError;

        unsafe fn try_read<T: RegInt>(&self, ptr: *const T) -> Result<T, Self::Error> {
            // The device only supports 16-bit register accesses
            assert_eq!(core::mem::size_of::<T>(), 2, "SPI registers must be 16 bits wide");
            let spi_addr = ptr.addr() as u8;
            let command: [u8; 3] = build_spi_read_access(spi_addr);
            let result: [u8; 3] = spi_transaction(command)?;
            let data: &[u8; 2] = extract_spi_read_data(result);
            Ok(T::from_ne_bytes(data.try_into().unwrap()))
        }

        unsafe fn try_write<T: RegInt>(&self, ptr: *mut T, value: T) -> Result<(), Self::Error> {
            // The device only supports 16-bit register accesses
            assert_eq!(core::mem::size_of::<T>(), 2, "SPI registers must be 16 bits wide");
            let spi_addr = ptr.addr() as u8;
            let spi_data: &[u8] = value.to_ne_bytes().as_ref();
            let command: [u8; 3] = build_spi_write_access(spi_addr, spi_data);
            spi_transaction(command)?;
            Ok(())
        }
    }

    fn main() {
        // pass the RegisterIO implementation into the addrmap constructor
        let spi_registers = SpiAddrmap::from_ptr_with(0 as _, &SpiRegisterIO);
        // now access registers as normal
        let reg0_value = spi_registers.reg0().try_read().unwrap();
    }

Links:

* Cargo docs for the `RawRegisterIO <examples/peakrdl_rust/io/trait.RawRegisterIO.html>`__ trait
* Cargo docs for the `RegisterIO <examples/peakrdl_rust/io/trait.RegisterIO.html>`__ trait
