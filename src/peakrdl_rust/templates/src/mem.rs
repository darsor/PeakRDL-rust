pub trait Memory {
    /// Primitive integer type used to represented a memory entry
    type Memwidth;

    fn first_entry_ptr(&self) -> *mut Self::Memwidth;

    /// Number of memory entries
    fn len(&self) -> usize;

    /// Bit width of each memory entry
    fn width(&self) -> usize;

    /// Access the memory entry at a specific index. Panics if out of bounds.
    fn index(&mut self, idx: usize) -> MemEntry<Self::Memwidth> {
        if idx < self.len() {
            MemEntry {
                ptr: unsafe { self.first_entry_ptr().add(idx) },
                width: self.width(),
            }
        } else {
            panic!(
                "Tried to index {} in a memory with only {} entries",
                idx,
                self.len()
            );
        }
    }

    /// Get an iterator over a range of memory entries
    fn slice<'a>(
        &'a mut self,
        range: impl core::ops::RangeBounds<usize>,
    ) -> MemEntryIter<'a, Self> {
        let low_idx = match range.start_bound() {
            core::ops::Bound::Included(idx) => *idx,
            core::ops::Bound::Excluded(idx) => *idx + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let high_idx = match range.end_bound() {
            core::ops::Bound::Included(idx) => *idx,
            core::ops::Bound::Excluded(idx) => *idx - 1,
            core::ops::Bound::Unbounded => self.len() - 1,
        };
        MemEntryIter {
            mem: self,
            low_idx,
            high_idx,
        }
    }

    /// Get an iterator over all memory entries
    fn iter<'a>(&'a mut self) -> MemEntryIter<'a, Self> {
        self.slice(..)
    }
}

/// Representation of a single memory entry
pub struct MemEntry<T> {
    ptr: *mut T,
    width: usize,
}

impl<T> MemEntry<T>
where
    T: num_traits::PrimInt,
{
    pub const unsafe fn from_ptr(ptr: *mut T, width: usize) -> Self {
        Self { ptr, width }
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Bit width of the entry
    pub const fn width(&self) -> usize {
        self.width
    }

    pub fn mask(&self) -> T {
        (T::one() << self.width) - T::one()
    }

    pub fn read(&self) -> T {
        let value = unsafe { self.ptr.read_volatile() };
        value & self.mask()
    }

    pub fn write(&mut self, value: T) {
        let value = value & self.mask();
        unsafe { self.ptr.write_volatile(value) }
    }
}

/// Iterator over memory entries
pub struct MemEntryIter<'a, M>
where
    M: ?Sized,
{
    mem: &'a mut M,
    low_idx: usize,
    high_idx: usize,
}

impl<'a, M> Iterator for MemEntryIter<'a, M>
where
    M: Memory,
{
    type Item = MemEntry<M::Memwidth>;

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

impl<'a, M> DoubleEndedIterator for MemEntryIter<'a, M>
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

impl<'a, M> core::iter::ExactSizeIterator for MemEntryIter<'a, M> where M: Memory {}
impl<'a, M> core::iter::FusedIterator for MemEntryIter<'a, M> where M: Memory {}
