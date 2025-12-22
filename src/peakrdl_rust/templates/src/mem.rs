//! Memory abstraction used to read, write, and iterate over memory entries

use crate::access::{self, Access};
use core::marker::PhantomData;
use num_traits::PrimInt;

/// Behaviors common to all SystemRDL memories
pub trait Memory: Sized {
    /// Primitive integer type used to represented a memory entry
    type Memwidth: PrimInt;
    type Access: Access;

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
    fn index(&self, idx: usize) -> MemEntry<Self::Memwidth, Self::Access> {
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
        range: impl core::ops::RangeBounds<usize>,
    ) -> MemEntryIter<Self::Memwidth, Self::Access> {
        let low_idx = match range.start_bound() {
            core::ops::Bound::Included(idx) => *idx,
            core::ops::Bound::Excluded(idx) => *idx + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let high_idx = match range.end_bound() {
            core::ops::Bound::Included(idx) => *idx,
            core::ops::Bound::Excluded(idx) => *idx - 1,
            core::ops::Bound::Unbounded => self.num_entries() - 1,
        };
        let num_entries = high_idx - low_idx + 1;
        MemEntryIter {
            next: self.index(low_idx),
            remaining: num_entries,
        }
    }

    /// Iterate over all memory entries
    #[must_use]
    fn iter(&self) -> MemEntryIter<Self::Memwidth, Self::Access> {
        self.slice(..)
    }
}

/// Representation of a single memory entry
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemEntry<T: PrimInt, A: Access> {
    ptr: *mut T,
    phantom: PhantomData<A>,
}

impl<T: PrimInt, A: Access> MemEntry<T, A> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware memory entry of size `T` with access `A`.
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T: PrimInt, A: access::Read> MemEntry<T, A> {
    /// Read the value of the hardware memory entry.
    #[must_use]
    pub fn read(&self) -> T {
        // SAFETY: MemEntry can only be constructed through from_ptr(),
        // which means the user has guaranteed the address points to
        // a suitable hardware memory.
        T::from_{{ctx.byte_endian}}(unsafe { self.ptr.read_volatile() })
    }
}

impl<T: PrimInt, A: access::Write> MemEntry<T, A> {
    /// Write the provided value to the hardware memory entry.
    pub fn write(&mut self, value: T) {
        // SAFETY: MemEntry can only be constructed through from_ptr(),
        // which means the user has guaranteed the address points to
        // a suitable hardware memory.
        unsafe { self.ptr.write_volatile(value.to_{{ctx.byte_endian}}()) }
    }
}

/// Iterator over memory entries
pub struct MemEntryIter<T: PrimInt, A: Access> {
    next: MemEntry<T, A>,
    remaining: usize,
}

impl<T: PrimInt, A: Access> Iterator for MemEntryIter<T, A> {
    type Item = MemEntry<T, A>;

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

impl<T: PrimInt, A: Access> DoubleEndedIterator for MemEntryIter<T, A> {
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

impl<T: PrimInt, A: Access> core::iter::ExactSizeIterator for MemEntryIter<T, A> {}
impl<T: PrimInt, A: Access> core::iter::FusedIterator for MemEntryIter<T, A> {}
