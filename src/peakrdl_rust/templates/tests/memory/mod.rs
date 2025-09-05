use core::ops::{Bound, Range, RangeBounds};

/// A block of memory used for simulating hardware registers.
///
/// A forced alignment of 16 bytes allows access to internal registers as
/// primitive types.
#[repr(align(16))]
pub(crate) struct Memory<const N: usize>([u8; N]);

impl<const N: usize> Memory<N> {
    pub fn new_zeroed() -> Self {
        Memory([0; N])
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    pub fn at(&self, address: usize) -> &[u8] {
        &self.0[address..]
    }

    pub fn at_mut(&mut self, address: usize) -> &mut [u8] {
        &mut self.0[address..]
    }
}

/// Most generated register files are lsb0, but the BitfieldAccess crate uses
/// the msb0 convention. Use the register width in bits to translate a bit range
/// into msb0 for field accesses.
///
/// # Example
///
/// ```
/// let lsb0_field_range = 7..=0;
/// let msb0_field_range = lsb0_to_msb0(lsb0_field_range, 32);
/// assert_eq!(msb0_field_range, 24..32);
/// ```
pub fn lsb0_to_msb0(bit_range: impl RangeBounds<usize>, reg_width: usize) -> Range<usize> {
    // convert to an inclusive range
    let start = match bit_range.start_bound() {
        Bound::Included(idx) => *idx,
        Bound::Excluded(idx) => *idx - 1,
        Bound::Unbounded => reg_width - 1,
    };
    let end = match bit_range.end_bound() {
        Bound::Included(idx) => *idx,
        Bound::Excluded(idx) => *idx + 1,
        Bound::Unbounded => 0,
    };
    (reg_width - 1 - start)..(reg_width - end)
}
