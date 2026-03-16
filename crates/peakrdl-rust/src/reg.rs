//! Register abstraction used to read, write, and modify register values
#![allow(clippy::inline_always)]

use core::convert::Infallible;

use crate::{
    access::{Access, Read, Write},
    endian::Endian,
    io::{PtrIO, RegisterIO},
};
use num_traits::{
    AsPrimitive, FromBytes, PrimInt, ToBytes, WrappingAdd, WrappingSub, identities::ConstZero,
};

/// Trait for primitive integer types accessed as registers.
pub trait RegInt:
    PrimInt
    + ConstZero
    + WrappingSub
    + WrappingAdd
    + FromBytes<Bytes: for<'a> TryFrom<&'a [u8], Error: core::fmt::Debug>>
    + ToBytes<Bytes: AsRef<[u8]>>
    + core::fmt::Debug
    + 'static
{
}
impl RegInt for u8 {}
impl RegInt for u16 {}
impl RegInt for u32 {}
impl RegInt for u64 {}
impl RegInt for u128 {}
impl RegInt for i8 {}
impl RegInt for i16 {}
impl RegInt for i32 {}
impl RegInt for i64 {}
impl RegInt for i128 {}

/// Trait implemented by all register types.
pub trait Register: Copy {
    // NOTE: SystemRDL guarantees accesswidth <= regwidth, and both are 2^N bits where N >= 3
    /// Primitive integer type representing the size of the full register value.
    type Regwidth: RegInt + AsPrimitive<Self::Accesswidth>;
    /// Primitive integer type representing the size of memory accesses used when
    /// reading/writing this register.
    type Accesswidth: RegInt + AsPrimitive<Self::Regwidth>;
    /// Access controls for this register.
    type Access: Access;
    /// Ordering of bytes within each accesswidth subword.
    type ByteEndian: Endian;
    /// Ordering of accesswidth subwords within the register.
    type WordEndian: Endian;

    /// Convert a raw bit value into a Register instance.
    ///
    /// # Safety
    ///
    /// The caller must ensure the raw bit value is valid for the given register.
    /// For example, by reading it directly from hardware.
    unsafe fn from_raw(val: Self::Regwidth) -> Self;

    /// Convert a Register instance into its raw bit value.
    fn to_raw(self) -> Self::Regwidth;
}

/// Register abstraction used to read, write, and modify register values.
///
/// This is generic over both the [`Register`] to access and the [`RegisterIO`] type
/// used to access the register.
///
/// The [`Register`] trait has associated types defining the register width,
/// access controls, and endianness which are used to customize the read/write
/// implementation for each register.
///
/// [`RegisterIO`] defaults to regular volatile pointer
/// I/O and is only needed for advanced use cases like tunneled registers.
#[derive(Debug, PartialEq, Eq)]
pub struct Reg<'io, R: Register, IO: RegisterIO = PtrIO> {
    ptr: *mut R::Regwidth,
    io: &'io IO,
}

// manually implemented to ease generic bounds (IO does not need to be Copy)
impl<R: Register, IO: RegisterIO> Copy for Reg<'_, R, IO> where R::Regwidth: Copy {}

// manually implemented to ease generic bounds (IO does not need to be Clone)
impl<R: Register, IO: RegisterIO> Clone for Reg<'_, R, IO>
where
    R::Regwidth: Clone,
{
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl<R: Register, IO: RegisterIO + Sync> Send for Reg<'_, R, IO> {}
unsafe impl<R: Register, IO: RegisterIO + Sync> Sync for Reg<'_, R, IO> {}

// pointer conversion functions
impl<R: Register> Reg<'static, R, PtrIO> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware register of type `R`.
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut R::Regwidth) -> Self {
        Self { ptr, io: &PtrIO }
    }
}

impl<'io, R: Register, IO: RegisterIO> Reg<'io, R, IO> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware register of type `R`.
    #[inline(always)]
    pub const unsafe fn from_ptr_with(ptr: *mut R::Regwidth, io: &'io IO) -> Self {
        Self { ptr, io }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(&self) -> *mut R {
        self.ptr.cast()
    }
}

// read access
impl<R: Register, IO: RegisterIO> Reg<'_, R, IO>
where
    R::Access: Read,
{
    /// Try to read a register value.
    ///
    /// If the register is to be modified (i.e., a read-modify-write), use the
    /// [`Reg::modify`] method instead.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let reg1_val = registers.regfile().register1().try_read().unwrap();
    /// let field1_val = reg1_val.field1();
    /// let field2_val = reg1_val.field2();
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    pub fn try_read(&self) -> Result<R, IO::Error> {
        unsafe { self.io.try_read_register(self.ptr) }
    }
}

impl<R: Register, IO: RegisterIO<Error = Infallible>> Reg<'_, R, IO>
where
    R::Access: Read,
{
    /// Read a register value.
    ///
    /// If the register is to be modified (i.e., a read-modify-write), use the
    /// [`Reg::modify`] method instead.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let reg1_val = registers.regfile().register1().read();
    /// let field1_val = reg1_val.field1();
    /// let field2_val = reg1_val.field2();
    /// ```
    #[inline(always)]
    #[allow(clippy::must_use_candidate)]
    pub fn read(&self) -> R {
        self.try_read().unwrap_infallible()
    }
}

// write access
impl<R: Register, IO: RegisterIO> Reg<'_, R, IO>
where
    R::Access: Write,
{
    /// Try to write a register value.
    ///
    /// Typically one would use [`Reg::try_write`] or [`Reg::try_modify`] to update a
    /// register's contents, but this method has a few different use cases such
    /// as updating a register with a stored value, or updating one register with
    /// the contents of another.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # write array index 0 value to index 1
    /// let reg0 = registers.regfile().reg_array()[0].try_read().unwrap();
    /// registers.regfile().reg_array()[1].try_write_value(reg0).unwrap();
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    pub fn try_write_value(&self, val: R) -> Result<(), IO::Error> {
        unsafe { self.io.try_write_register(self.ptr, val) }
    }
}

impl<R: Register, IO: RegisterIO<Error = Infallible>> Reg<'_, R, IO>
where
    R::Access: Write,
{
    /// Write a register value.
    ///
    /// Typically one would use [`Reg::write`] or [`Reg::modify`] to update a
    /// register's contents, but this method has a few different use cases such
    /// as updating a register with a stored value, or updating one register with
    /// the contents of another.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # write array index 0 value to index 1
    /// let reg0 = registers.regfile().reg_array()[0].read();
    /// registers.regfile().reg_array()[1].write_value(reg0);
    /// ```
    #[inline(always)]
    pub fn write_value(&self, val: R) {
        self.try_write_value(val).unwrap_infallible();
    }
}

impl<R: Default + Register, IO: RegisterIO> Reg<'_, R, IO>
where
    R::Access: Write,
{
    /// Try to write a register.
    ///
    /// This method takes a closure. The input to the closure is a mutable reference
    /// to the default value of the register. It can be updated in the closure. The
    /// updated value is then written to the hardware register.
    ///
    /// # Example
    ///
    /// ```ignore
    /// registers.regfile().register1().try_write(|r| {
    ///     // r contains the default (reset) value of the register
    ///     r.set_field1(0x1);
    ///     r.set_field2(0x0);
    /// }).unwrap();
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    pub fn try_write<T>(&self, f: impl FnOnce(&mut R) -> T) -> Result<T, IO::Error> {
        let mut val = Default::default();
        let res = f(&mut val);
        self.try_write_value(val)?;
        Ok(res)
    }
}

impl<R: Default + Register, IO: RegisterIO<Error = Infallible>> Reg<'_, R, IO>
where
    R::Access: Write,
{
    /// Write a register.
    ///
    /// This method takes a closure. The input to the closure is a mutable reference
    /// to the default value of the register. It can be updated in the closure. The
    /// updated value is then written to the hardware register.
    ///
    /// # Example
    ///
    /// ```ignore
    /// registers.regfile().register1().write(|r| {
    ///     // r contains the default (reset) value of the register
    ///     r.set_field1(0x1);
    ///     r.set_field2(0x0);
    /// });
    /// ```
    #[inline(always)]
    pub fn write<T>(&self, f: impl FnOnce(&mut R) -> T) -> T {
        self.try_write(f).unwrap_infallible()
    }
}

// read/write access
impl<R: Register, IO: RegisterIO> Reg<'_, R, IO>
where
    R::Access: Read + Write,
{
    /// Try to modify a register.
    ///
    /// This method takes a closure. The input to the closure is a mutable reference
    /// to the current value of the register. It can be updated in the closure. The
    /// updated value is then written back to the hardware register.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let orig_r = registers.regfile().register1().try_modify(|r| {
    ///     // r contains the current value of the register
    ///     orig_r = r.clone()
    ///     r.set_field1(r.field1());
    ///     r.set_field2(0x0);
    ///     // whatever value the closure returns is returned by the .try_modify() method
    ///     orig_r
    /// }).unwrap();
    /// ```
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    pub fn try_modify<T>(&self, f: impl FnOnce(&mut R) -> T) -> Result<T, IO::Error> {
        let mut val = self.try_read()?;
        let res = f(&mut val);
        self.try_write_value(val)?;
        Ok(res)
    }
}

impl<R: Register, IO: RegisterIO<Error = Infallible>> Reg<'_, R, IO>
where
    R::Access: Read + Write,
{
    /// Modify a register.
    ///
    /// This method takes a closure. The input to the closure is a mutable reference
    /// to the current value of the register. It can be updated in the closure. The
    /// updated value is then written back to the hardware register.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let orig_r = registers.regfile().register1().modify(|r| {
    ///     // r contains the current value of the register
    ///     orig_r = r.clone()
    ///     r.set_field1(r.field1());
    ///     r.set_field2(0x0);
    ///     // whatever value the closure returns is returned by the .modify() method
    ///     orig_r
    /// });
    /// ```
    #[inline(always)]
    pub fn modify<T>(&self, f: impl FnOnce(&mut R) -> T) -> T {
        self.try_modify(f).unwrap_infallible()
    }
}

trait UnwrapInfallible {
    type T;

    fn unwrap_infallible(self) -> Self::T;
}

impl<T> UnwrapInfallible for Result<T, Infallible> {
    type T = T;

    fn unwrap_infallible(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => match e {}, // exhaustive match on uninhabited type
        }
    }
}
