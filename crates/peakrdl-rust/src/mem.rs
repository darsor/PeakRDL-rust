//! Memory abstraction used to read, write, and iterate over memory entries

use crate::{
    access::{Access, Read, Write},
    endian::Endian,
};
use core::{
    iter::{ExactSizeIterator, FusedIterator},
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};
use num_traits::PrimInt;

/// Behaviors common to all SystemRDL memories
pub trait Memory: Sized {
    /// Primitive integer type used to represented a memory entry
    type Memwidth: PrimInt;
    type Access: Access;
    type Endian: Endian;

    #[must_use]
    fn first_entry_ptr(&self) -> *mut Self::Memwidth;

    /// Number of memory entries
    #[must_use]
    fn num_entries(&self) -> usize;

    /// Bit width of each memory entry
    #[must_use]
    fn width(&self) -> usize;

    /// Access the memory entry at a specific index. Panics if out of bounds.
    #[must_use]
    fn index(&self, idx: usize) -> MemEntry<Self::Memwidth, Self::Access, Self::Endian> {
        if idx < self.num_entries() {
            unsafe { MemEntry::from_ptr(self.first_entry_ptr().wrapping_add(idx)) }
        } else {
            panic!(
                "Tried to index {} in a memory with only {} entries",
                idx,
                self.num_entries()
            );
        }
    }

    /// Iterate over a range of memory entries
    #[must_use]
    fn slice(
        &self,
        range: impl RangeBounds<usize>,
    ) -> MemEntryIter<Self::Memwidth, Self::Access, Self::Endian> {
        let low_idx = match range.start_bound() {
            Bound::Included(idx) => *idx,
            Bound::Excluded(idx) => *idx + 1,
            Bound::Unbounded => 0,
        };
        let high_idx = match range.end_bound() {
            Bound::Included(idx) => *idx,
            Bound::Excluded(idx) => *idx - 1,
            Bound::Unbounded => self.num_entries() - 1,
        };
        let num_entries = high_idx - low_idx + 1;
        MemEntryIter {
            next: self.index(low_idx),
            remaining: num_entries,
        }
    }

    /// Iterate over all memory entries
    #[must_use]
    fn iter(&self) -> MemEntryIter<Self::Memwidth, Self::Access, Self::Endian> {
        self.slice(..)
    }
}

/// Representation of a single memory entry
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemEntry<T: PrimInt, A: Access, E: Endian> {
    ptr: *mut T,
    phantom_access: PhantomData<A>,
    phantom_endian: PhantomData<E>,
}

impl<T: PrimInt, A: Access, E: Endian> MemEntry<T, A, E> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware memory entry of size `T` with access `A` and endianness `E`.
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr,
            phantom_access: PhantomData,
            phantom_endian: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T: PrimInt, A: Read, E: Endian> MemEntry<T, A, E> {
    /// Read the value of the hardware memory entry.
    #[must_use]
    pub fn read(&self) -> T {
        // SAFETY: MemEntry can only be constructed through from_ptr(),
        // which means the user has guaranteed the address points to
        // a suitable hardware memory.
        E::from_register_endian(unsafe { self.ptr.read_volatile() })
    }
}

impl<T: PrimInt, A: Write, E: Endian> MemEntry<T, A, E> {
    /// Write the provided value to the hardware memory entry.
    pub fn write(&mut self, value: T) {
        // SAFETY: MemEntry can only be constructed through from_ptr(),
        // which means the user has guaranteed the address points to
        // a suitable hardware memory.
        unsafe { self.ptr.write_volatile(E::to_register_endian(value)) }
    }
}

/// Iterator over memory entries
#[derive(Debug)]
pub struct MemEntryIter<T: PrimInt, A: Access, E: Endian> {
    next: MemEntry<T, A, E>,
    remaining: usize,
}

impl<T: PrimInt, A: Access, E: Endian> Iterator for MemEntryIter<T, A, E> {
    type Item = MemEntry<T, A, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            let new_next = unsafe { MemEntry::from_ptr(self.next.as_ptr().wrapping_add(1)) };
            Some(core::mem::replace(&mut self.next, new_next))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: PrimInt, A: Access, E: Endian> DoubleEndedIterator for MemEntryIter<T, A, E> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            unsafe {
                Some(MemEntry::from_ptr(
                    self.next.as_ptr().wrapping_add(self.remaining),
                ))
            }
        }
    }
}

impl<T: PrimInt, A: Access, E: Endian> ExactSizeIterator for MemEntryIter<T, A, E> {}
impl<T: PrimInt, A: Access, E: Endian> FusedIterator for MemEntryIter<T, A, E> {}
