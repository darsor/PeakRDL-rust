Generated Rust Code Contents
============================

The generated Rust code is inspired by `chiptool`_ and adapted to work for
SystemRDL. The chiptool README has a good explanation of some if its design
decisions as a departure from `svd2rust`_, which some developers may be more
familiar with, especially relating to component lifetime and borrowing.

.. _chiptool: https://github.com/embassy-rs/chiptool
.. _svd2rust: https://github.com/rust-embedded/svd2rust

In general, the generated code allows multiple owned copies of the same
component. All register memory accesses are treated as volatile.
Synchronization between subsystems using the same register is required, and
is best performed at a higher level for maximum flexibility.

The best documentation for the generated Rust code is generated with the
crate as doc comments and can be compiled using ``cargo doc``. The purpose
of this page is to provide a high-level overview of architectural decisions
and how the various pieces fit together.

SystemRDL -> Rust Mapping
-------------------------

The generated Rust code has a separate file/module for each SystemRDL
component. This simplifies code generation and allows the generated crate to
follow the hierarchical SystemRDL structure very closely.

Definitive Types
^^^^^^^^^^^^^^^^

Anonymous SystemRDL types are defined in the module hierarchy where they are
used. Definitive (named) SystemRDL types are placed in the module hierarchy
lexically (i.e., under the component they were declared in, not the one they
were used in). The named type module is then publically re-exported in each
component where it's used. This has a few benefits:

* Allows full reuse of definitive types
* Lets the developer locate and use data structures by following the hierarchically
  instantiated component names rather than needing to know where the type
  was defined in the SystemRDL
* Avoids namespace collision, since SystemRDL has separate namespaces for
  type names and instance names.

Type Naming Conventions
^^^^^^^^^^^^^^^^^^^^^^^

SystemRDL type and instance names are re-cased to the standard Rust
conventions in `RFC 430`_. For example, snake_case is used for modules and methods,
UpperCamelCase is used for types and enum variants, and SCREAMING_SNAKE_CASE
is used for constants. This helps avoid namespace collision and follows what
Rust developers expect from Rust code.

.. _RFC 430: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md

Addrmaps, Regfiles, and Memories
--------------------------------

Addrmaps, Regfiles, and Memories are represented as Rust structs with a single
data member: the address of the component. Sub-components are exposed using
accessor methods which return the subcomponent struct with the appropriate
address offset.

For example, an ``addrmap`` component might expose a ``regfile`` component for
a grouping of SPI registers via a method with the signature:

.. code-block:: rust

    pub const fn spi(&self) -> Spi;

Memories additionally implement the ``Memory`` trait, which provides methods
for accessing and iterating over specific indices within the memory. Virtual
registers (registers defined within a memory component) are supported and
are treated like any other register.

Each generated struct includes:

* An unsafe ``from_ptr()`` constructor that takes a base address pointer
* Accessor methods for each register and sub-block

Arrays
------

If a component is instantiated as an array, then the getter method for that
component returns a rust array of component structs. For example, an ``addrmap``
exposing an array of four SPI controller ``regfile``\ s might have the signature:

.. code-block:: rust

    pub const fn spi(&self) -> [Spi, 4];

The return value can be stored, or can be indexed directly. For example, if
only the second controller is needed, the compiler will generally optimize
``regs.spi()[1]`` to only compute the address offset for that single SPI
controller.

Multidimensional arrays are fully supported.

Registers
---------

Each SystemRDL ``reg`` component gets its own generated Rust struct with a
single private data member: the value of the register. This struct exposes
setter/getter methods for accessing the register's fields.

Each generated register struct includes:

* An implementation of the ``Register`` trait, which provides

  * An unsafe ``from_raw()`` constructor that takes the raw register value
  * A ``to_raw()`` method that returns the raw register value

* Getter methods for each readable register field
* Setter methods for each writable register field
* Constants for each register field, including

  * The bit offset within the register
  * The bit width of the field
  * A bit mask
  * Whether the field is signed/unsigned (if ``is_signed`` property is defined)
  * The number of integer/fractional bits (if a fixed-point field)

* A ``Debug`` impl that prints the current value of each field.
* A ``Default`` impl that returns the reset value of the register.

These register structs are not instantiated directly. Instead, a handle to a
register component will be of the type ``crate::reg::Reg<Register, Access>``.
The ``Reg`` type handles reading, writing, and modifying the register value
in memory, while the register-specific type it takes as a generic is used to
read and write fields within the register value. This allows multiple fields
to be written/read with a single memory access.

For example:

.. code-block:: rust

    let ctrl_reg: Reg<Ctrl, RW> = registers.spi().ctrl();

    // read the register value
    let ctrl_reg_value: Ctrl = ctrl_reg.read();
    // access the 'enable' field
    let enable_val: bool = ctrl_reg_value.enable();

    // read-modify-write a register
    ctrl_reg.modify(|ctrl: &mut Ctrl| {
        // 'ctrl' contains the current value of the Ctrl register
        value.set_enable(true);
        // the updated 'ctrl' register is written to memory after the closure exits
    });

The ``Reg`` struct is also generic over the access type of the register (R, W,
or RW). This makes it so that, for example, read-only registers don't expose
any methods for writing the value to memory.

Wide Registers
^^^^^^^^^^^^^^

Registers are currently limited to 128 bits due to the largest primitive
integer type being ``u128``. The accesswidth of registers is honored,
and accesses are performed starting at the lowest address.

Field Types
^^^^^^^^^^^

Depending on the field width and properties, different types may be used
for getting/setting the field value. These can include:

* bool: for single-bit non-numeric fields
* u8, u16, u32, etc.: for unsigned integer fields
* i8, i16, i32, etc.: for signed integer fields (sign-extended to the primitive width)
* A custom Rust ``enum`` type for fields with the ``encode`` property set

  * The return type for these fields' getters is ``Option<SomeEnum>``, and will
    return None if the field's bit pattern doesn't match any enum variant.

* An instance of the ``FixedPoint`` type for fields with the ``intwidth``
  or ``fracwidth`` properties defined.

Embedded Support
----------------
Generated code is compatible with ``no_std`` environments commonly used in embedded systems:

* No heap allocations
* Minimal runtime overhead
* Volatile memory access patterns
