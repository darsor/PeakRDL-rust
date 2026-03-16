//! Memory abstraction used to read, write, and iterate over memory entries

use crate::{
    access::{Access, Read, Write},
    endian::Endian,
    reg::RegInt,
};
use core::{
    fmt::Debug,
    iter::{ExactSizeIterator, FusedIterator},
    ops::{Bound, RangeBounds},
};

/// Behaviors common to all SystemRDL memories
pub trait Memory: Sized {
    /// Primitive integer type used to represented a memory entry
    type Memwidth: RegInt;
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
    fn index(&self, idx: usize) -> MemEntry<Self> {
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
    fn slice(&self, range: impl RangeBounds<usize>) -> MemEntryIter<Self> {
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
    fn iter(&self) -> MemEntryIter<Self> {
        self.slice(..)
    }
}

/// Representation of a single memory entry
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemEntry<M: Memory> {
    ptr: *mut M::Memwidth,
}

impl<M: Memory> MemEntry<M> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware memory entry of size `T` with access `A` and endianness `E`.
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut M::Memwidth) -> Self {
        Self { ptr }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut M::Memwidth {
        self.ptr
    }
}

impl<M: Memory> MemEntry<M>
where
    M::Access: Read,
{
    /// Read the value of the hardware memory entry.
    #[must_use]
    pub fn read(&self) -> M::Memwidth {
        // SAFETY: MemEntry can only be constructed through from_ptr(),
        // which means the user has guaranteed the address points to
        // a suitable hardware memory.
        M::Endian::from_register_endian(unsafe { self.ptr.read_volatile() })
    }
}

impl<M: Memory> MemEntry<M>
where
    M::Access: Write,
{
    /// Write the provided value to the hardware memory entry.
    pub fn write(&mut self, value: M::Memwidth) {
        // SAFETY: MemEntry can only be constructed through from_ptr(),
        // which means the user has guaranteed the address points to
        // a suitable hardware memory.
        unsafe {
            self.ptr
                .write_volatile(M::Endian::to_register_endian(value));
        }
    }
}

/// Iterator over memory entries
#[derive(Debug)]
pub struct MemEntryIter<M: Memory> {
    next: MemEntry<M>,
    remaining: usize,
}

impl<M: Memory> Iterator for MemEntryIter<M> {
    type Item = MemEntry<M>;

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

impl<M: Memory> DoubleEndedIterator for MemEntryIter<M> {
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

impl<M: Memory> ExactSizeIterator for MemEntryIter<M> {}
impl<M: Memory> FusedIterator for MemEntryIter<M> {}
