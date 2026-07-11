//! Traits for customizing the register I/O implementation
use crate::{
    access::{Read, Write},
    endian::Endian,
    reg::{RegInt, Register},
};
use core::cell::RefCell;
use num_traits::{AsPrimitive, Bounded, ConstZero};

/// Raw register I/O trait
///
/// Types that implement this trait also automatically implement [`RegisterIO`].
pub trait RawRegisterIO {
    /// The error type this register transport returns. For infallible transports
    /// (e.g., direct volatile pointer accesses), this should be [`core::convert::Infallible`]
    /// so that the [`Reg`][crate::reg::Reg] type can provide an infallible API in addition
    /// to the `try_*` API.
    type Error;

    /// Try to read a primitive integer from memory.
    ///
    /// The returned value is in the register's native endianness (not necessarily
    /// the host's endianness).
    ///
    /// # Safety
    ///
    /// This method may dereference raw pointer. The caller must ensure the pointer
    /// is valid and points to a valid memory location.
    #[allow(clippy::missing_errors_doc)]
    unsafe fn try_read<T: RegInt>(&self, ptr: *const T) -> Result<T, Self::Error>;

    /// Try to write primitive integer to memory
    ///
    /// The provided value is in the register's native endianness (not necessarily
    /// the host's endianness).
    ///
    /// # Safety
    ///
    /// This method may dereference a raw pointer. The caller must ensure the pointer
    /// is valid and points to a valid writeable memory location.
    #[allow(clippy::missing_errors_doc)]
    unsafe fn try_write<T: RegInt>(&self, ptr: *mut T, value: T) -> Result<(), Self::Error>;
}

/// Register I/O
///
/// Register accesses are performed through implementers of this trait. This trait's
/// methods handle details like endianness, multi-word writes, etc.
///
/// Most user I/O interfaces should simply implement the [`RawRegisterIO`] trait, since a
/// blanket implementation exists for all implementers of [`RawRegisterIO`].
pub trait RegisterIO {
    type Error;

    /// Read a register value
    ///
    /// This method must respect the register's endianness and accesswidth,
    /// as encoded in the generic [`Register`]'s associated types.
    ///
    /// # Safety
    ///
    /// This method may dereference a raw pointer. The caller must ensure the pointer
    /// is valid and points to a valid memory location.
    #[allow(clippy::missing_errors_doc)]
    unsafe fn try_read_register<R: Register>(
        &self,
        ptr: *const R::Regwidth,
    ) -> Result<R, Self::Error>
    where
        R::Access: Read;

    /// Write a register value
    ///
    /// This method must respect the register's endianness and accesswidth,
    /// as encoded in the generic [`Register`]'s associated types.
    ///
    /// # Safety
    ///
    /// This method may dereference a raw pointer. The caller must ensure the pointer
    /// is valid and points to a valid writeable memory location.
    #[allow(clippy::missing_errors_doc)]
    unsafe fn try_write_register<R: Register>(
        &self,
        ptr: *mut R::Regwidth,
        value: R,
    ) -> Result<(), Self::Error>
    where
        R::Access: Write;
}

impl<T> RegisterIO for T
where
    T: RawRegisterIO,
{
    type Error = T::Error;

    unsafe fn try_read_register<R: Register>(
        &self,
        ptr: *const R::Regwidth,
    ) -> Result<R, Self::Error>
    where
        R::Access: Read,
    {
        let ptr = ptr.cast::<R::Accesswidth>();

        let accesswidth = 8 * core::mem::size_of::<R::Accesswidth>();
        let regwidth = 8 * core::mem::size_of::<R::Regwidth>();
        let num_subwords = regwidth / accesswidth;

        // Fast path: a single-word register is one volatile load. `num_subwords`
        // is a compile-time constant (derived from `size_of`), so for single-word
        // registers the multi-word loop below is dropped entirely and the access
        // folds to a single load at the call site.
        if num_subwords == 1 {
            // SAFETY: accesswidth == regwidth here, so this reads exactly the
            // register's bounds (same guarantee the loop relies on).
            let subword = unsafe { self.try_read::<R::Accesswidth>(ptr)? };
            let subword = R::ByteEndian::from_register_endian(subword);
            // SAFETY: value just read directly from hardware.
            return unsafe { Ok(R::from_raw(subword.as_())) };
        }

        // read one subword at a time, starting at the lowest address
        let raw_value = (0..num_subwords)
            .map(|i| {
                // SAFETY: SystemRDL guarantees accesswidth <= regwidth, so we won't
                // read outside the bounds of the original pointer.
                unsafe { (i, self.try_read(ptr.wrapping_add(i))) }
            })
            .try_fold(R::Regwidth::ZERO, |reg, (i, subword)| {
                let significance = R::WordEndian::address_order_to_significance(i, num_subwords);
                let subword = R::ByteEndian::from_register_endian(subword?);
                Ok(reg | (subword.as_() << (significance * accesswidth)))
            })?;
        // SAFETY: The value was just read directly from hardware, and should
        // therefore be a valid register value.
        unsafe { Ok(R::from_raw(raw_value)) }
    }

    unsafe fn try_write_register<R: Register>(
        &self,
        ptr: *mut R::Regwidth,
        value: R,
    ) -> Result<(), Self::Error>
    where
        R::Access: Write,
    {
        let ptr = ptr.cast::<R::Accesswidth>();
        let value = value.to_raw();

        let accesswidth = 8 * core::mem::size_of::<R::Accesswidth>();
        let regwidth = 8 * core::mem::size_of::<R::Regwidth>();
        let num_subwords = regwidth / accesswidth;
        let mask = R::Accesswidth::max_value().as_();

        // Fast path: a single-word register is one volatile store. `num_subwords`
        // is a compile-time constant, so for single-word registers the loop below
        // is dropped entirely and the access folds to a single store at the call site.
        if num_subwords == 1 {
            let subword = R::ByteEndian::to_register_endian(value.as_());
            // SAFETY: accesswidth == regwidth here, so this writes exactly the
            // register's bounds (same guarantee the loop relies on).
            return unsafe { self.try_write::<R::Accesswidth>(ptr, subword) };
        }

        // write one subword at a time, starting at the lowest address
        for i in 0..num_subwords {
            let significance = R::WordEndian::address_order_to_significance(i, num_subwords);
            let subword = (value >> (significance * accesswidth)) & mask;
            let subword = R::ByteEndian::to_register_endian(subword.as_());
            // SAFETY: SystemRDL guarantees accesswidth <= regwidth, so we won't
            // write outside the bounds of the original pointer.
            unsafe {
                self.try_write(ptr.wrapping_add(i), subword)?;
            }
        }
        Ok(())
    }
}

/// Default [`RegisterIO`] implementation.
///
/// Provides infallible register access through volatile pointer reads
/// and writes.
pub struct PtrIO;

impl RawRegisterIO for PtrIO {
    type Error = core::convert::Infallible;

    unsafe fn try_read<T: RegInt>(&self, ptr: *const T) -> Result<T, Self::Error> {
        Ok(unsafe { ptr.read_volatile() })
    }

    unsafe fn try_write<T: RegInt>(&self, ptr: *mut T, value: T) -> Result<(), Self::Error> {
        unsafe { ptr.write_volatile(value) };
        Ok(())
    }
}

/// Mocked [`RegisterIO`] implementation.
///
/// Implemented as an array of N bytes, register writes and reads
/// simply write to/from the internal array.
pub struct MockIO<const N: usize>(RefCell<[u8; N]>);

impl<const N: usize> MockIO<N> {
    /// Construct a new zeroed instance of the mocked register memory.
    #[must_use]
    pub fn new_zeroed() -> Self {
        Self(RefCell::new([0; N]))
    }

    /// Get the base register address of the instance (always 0).
    pub fn base_ptr(&self) -> *mut () {
        0 as _
    }
}

impl<const N: usize> RawRegisterIO for MockIO<N> {
    type Error = core::convert::Infallible;

    unsafe fn try_read<T: RegInt>(&self, ptr: *const T) -> Result<T, Self::Error> {
        let addr = ptr.addr();
        let size = core::mem::size_of::<T>();
        let data = self.0.borrow();
        let bytes = &data[addr..addr + size];
        Ok(T::from_ne_bytes(
            &bytes.try_into().expect("Incorrect slice length"),
        ))
    }

    unsafe fn try_write<T: RegInt>(&self, ptr: *mut T, value: T) -> Result<(), Self::Error> {
        let addr = ptr.addr();
        let size = core::mem::size_of::<T>();
        let mut data = self.0.borrow_mut();
        let bytes = &mut data[addr..addr + size];
        bytes.copy_from_slice(value.to_ne_bytes().as_ref());
        Ok(())
    }
}
