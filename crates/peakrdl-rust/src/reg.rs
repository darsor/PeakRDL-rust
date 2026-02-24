//! Register abstraction used to read, write, and modify register values
#![allow(clippy::inline_always)]

use crate::{
    access::{self, Access},
    endian::Endian,
};
use core::marker::PhantomData;
use num_traits::{AsPrimitive, Bounded, PrimInt, identities::ConstZero};

pub trait Register: Copy {
    // NOTE: SystemRDL guarantees accesswidth <= regwidth, and both are 2^N bits where N >= 3
    type Regwidth: PrimInt + AsPrimitive<Self::Accesswidth> + ConstZero + 'static;
    type Accesswidth: PrimInt + AsPrimitive<Self::Regwidth> + Bounded;
    type Endian: Endian;

    /// Convert a raw bit value into a Register instance.
    ///
    /// # Safety
    ///
    /// The caller must ensure the raw bit value is valid for the given register.
    unsafe fn from_raw(val: Self::Regwidth) -> Self;

    /// Convert a Register instance into its raw bit value.
    fn to_raw(self) -> Self::Regwidth;
}

/// Register abstraction used to read, write, and modify register values
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T: Register, A: Access> {
    ptr: *mut T::Regwidth,
    phantom: PhantomData<A>,
}

unsafe impl<T: Register, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Register, A: Access> Sync for Reg<T, A> {}

// pointer conversion functions
impl<T: Register, A: Access> Reg<T, A> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware register of type `T` with access `A`.
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut T::Regwidth) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr.cast()
    }
}

// read access
impl<T: Register, A: access::Read> Reg<T, A> {
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
    pub fn read(&self) -> T {
        unsafe { read_register(self.ptr) }
    }
}

// write access
impl<T: Register, A: access::Write> Reg<T, A> {
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
    /// let reg0 = registers.regfile().reg_array()[0].read();
    /// registers.regfile().reg_array()[1].write_value(reg0);
    /// ```
    #[inline(always)]
    pub fn write_value(&self, val: T) {
        unsafe { write_register(self.ptr, val) }
    }
}

impl<T: Default + Register, A: access::Write> Reg<T, A> {
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
    pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = Default::default();
        let res = f(&mut val);
        self.write_value(val);
        res
    }
}

// read/write access
impl<T: Register, A: access::Read + access::Write> Reg<T, A> {
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
    pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut val = self.read();
        let res = f(&mut val);
        self.write_value(val);
        res
    }
}

unsafe fn read_register<R: Register>(ptr: *const R::Regwidth) -> R {
    let ptr = ptr.cast::<R::Accesswidth>();

    let accesswidth = 8 * core::mem::size_of::<R::Accesswidth>();
    let regwidth = 8 * core::mem::size_of::<R::Regwidth>();
    let num_subwords = regwidth / accesswidth;

    // read one subword at a time, starting at the lowest address
    let raw_value = (0..num_subwords)
        .map(|i| {
            // SAFETY: SystemRDL guarantees accesswidth <= regwidth, so we won't
            // read outside the bounds of the original pointer.
            unsafe { (i, ptr.wrapping_add(i).read_volatile()) }
        })
        .fold(R::Regwidth::ZERO, |reg, (i, subword)| {
            let significance = R::Endian::address_order_to_significance(i, num_subwords);
            let subword = R::Endian::from_register_endian(subword);
            reg | (subword.as_() << (significance * accesswidth))
        });
    // SAFETY: The value was just read directly from hardware, and should
    // therefore be a valid register value.
    unsafe { R::from_raw(raw_value) }
}

unsafe fn write_register<R: Register>(ptr: *mut R::Regwidth, value: R) {
    let ptr = ptr.cast::<R::Accesswidth>();
    let value = value.to_raw();

    let accesswidth = 8 * core::mem::size_of::<R::Accesswidth>();
    let regwidth = 8 * core::mem::size_of::<R::Regwidth>();
    let num_subwords = regwidth / accesswidth;
    let mask = R::Accesswidth::max_value().as_();

    // write one subword at a time, starting at the lowest address
    for i in 0..num_subwords {
        let significance = R::Endian::address_order_to_significance(i, num_subwords);
        let subword = (value >> (significance * accesswidth)) & mask;
        let subword = R::Endian::to_register_endian(subword.as_());
        // SAFETY: SystemRDL guarantees accesswidth <= regwidth, so we won't
        // write outside the bounds of the original pointer.
        unsafe {
            ptr.wrapping_add(i).write_volatile(subword);
        }
    }
}
