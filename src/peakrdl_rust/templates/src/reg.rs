//! Register abstraction used to read, write, and modify register values

use core::marker::PhantomData;

/// Read-write register access token
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;

/// Read-only register access token
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;

/// Write-only register access token
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
}

pub trait Access: sealed::Access + Copy {}

impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}

/// Register abstraction used to read, write, and modify register values
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T: Copy, A: Access> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A)>,
}

unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}

// pointer conersion functions
impl<T: Copy, A: Access> Reg<T, A> {
    #[allow(clippy::missing_safety_doc)]
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr: ptr as _,
            phantom: PhantomData,
        }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr as _
    }
}

// read access
impl<T: Copy, A: Read> Reg<T, A> {
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
    pub fn read(&self) -> T {
        unsafe { (self.ptr as *mut T).read_volatile() }
    }
}

// write access
impl<T: Copy, A: Write> Reg<T, A> {
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
        unsafe { (self.ptr as *mut T).write_volatile(val) }
    }
}

impl<T: Default + Copy, A: Write> Reg<T, A> {
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
impl<T: Copy, A: Read + Write> Reg<T, A> {
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
    ///     orig r
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
