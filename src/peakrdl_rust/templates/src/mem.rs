//! Traits and types for memory components

use crate::access::{self, Access};
use core::marker::PhantomData;

pub trait Memory: Sized {
    /// Primitive integer type used to represented a memory entry
    type Memwidth: num_traits::PrimInt;
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
    fn index(&mut self, idx: usize) -> MemEntry<Self::Memwidth, Self::Access> {
        if idx < self.num_entries() {
            unsafe { MemEntry::from_ptr(self.first_entry_ptr().add(idx), self.width()) }
        } else {
            panic!(
                "Tried to index {} in a memory with only {} entries",
                idx,
                self.num_entries()
            );
        }
    }

    /// Get an iterator over a range of memory entries
    #[must_use]
    fn slice(
        &mut self,
        range: impl core::ops::RangeBounds<usize>,
    ) -> MemEntryIter<'_, Self> {
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
        MemEntryIter {
            mem: self,
            low_idx,
            high_idx,
        }
    }

    /// Get an iterator over all memory entries
    #[must_use]
    fn iter(&mut self) -> MemEntryIter<'_, Self> {
        self.slice(..)
    }
}

/// Representation of a single memory entry
pub struct MemEntry<T, A: Access> {
    ptr: *mut T,
    width: usize,
    phantom: PhantomData<A>,
}

impl<T, A: Access> MemEntry<T, A>
where
    T: num_traits::PrimInt,
{
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware memory entry of size `T` with access `A`.
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut T, width: usize) -> Self {
        Self {
            ptr,
            width,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Bit width of the entry
    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn mask(&self) -> T {
        (T::one() << self.width) - T::one()
    }
}

impl<T, A: access::Read> MemEntry<T, A>
where
    T: num_traits::PrimInt,
{
    #[must_use]
    pub fn read(&self) -> T {
        let value = unsafe { self.ptr.read_volatile() };
        value & self.mask()
    }
}

impl<T, A: access::Write> MemEntry<T, A>
where
    T: num_traits::PrimInt,
{
    pub fn write(&mut self, value: T) {
        let value = value & self.mask();
        unsafe { self.ptr.write_volatile(value) }
    }
}

/// Iterator over memory entries
pub struct MemEntryIter<'a, M: Memory> {
    mem: &'a mut M,
    low_idx: usize,
    high_idx: usize,
}

impl<M: Memory> Iterator for MemEntryIter<'_, M> {
    type Item = MemEntry<M::Memwidth, M::Access>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.low_idx > self.high_idx {
            None
        } else {
            let entry = self.mem.index(self.low_idx);
            self.low_idx += 1;
            Some(entry)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.low_idx > self.high_idx {
            (0, Some(0))
        } else {
            let len = self.high_idx - self.low_idx + 1;
            (len, Some(len))
        }
    }
}

impl<M> DoubleEndedIterator for MemEntryIter<'_, M>
where
    M: Memory,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.low_idx > self.high_idx {
            None
        } else {
            let entry = self.mem.index(self.high_idx);
            if self.high_idx > 0 {
                self.high_idx -= 1;
            } else {
                self.low_idx += 1;
            }
            Some(entry)
        }
    }
}

impl<M> core::iter::ExactSizeIterator for MemEntryIter<'_, M> where M: Memory {}
impl<M> core::iter::FusedIterator for MemEntryIter<'_, M> where M: Memory {}
