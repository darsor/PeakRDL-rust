/// A block of memory used for simulating hardware registers.
///
/// A forced alignment of 16 bytes allows access to internal registers as
/// primitive types.
#[repr(align(16))]
pub(crate) struct Memory<const N: usize>([u8; N]);

impl<const N: usize> Memory<N> {
    pub(crate) fn new_zeroed() -> Self {
        Memory([0; N])
    }

    pub(crate) fn at(&self, byte_idx: usize) -> &[u8] {
        &self.0[byte_idx..]
    }

    pub(crate) fn at_mut(&mut self, byte_idx: usize) -> &mut [u8] {
        &mut self.0[byte_idx..]
    }
}

impl<const N: usize> AsRef<[u8]> for Memory<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8]> for Memory<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
