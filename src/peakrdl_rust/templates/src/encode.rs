//! Types for enum-encoded field representations

use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

/// A bit pattern representing an unencoded enum variant.
///
/// This is typically obtained as an error value when reading an enum-encoded
/// field, but the bit value of the field doesn't match any known enum variant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UnknownVariant<T> {
    /// The bit value of the unknown variant
    pub value: T,
    _private: (),
}

impl<T: Copy> UnknownVariant<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            _private: (),
        }
    }
}

impl<T: Copy + Display> Display for UnknownVariant<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Unknown enum variant: {}", self.value)
    }
}

impl<T: Copy + Debug + Display> Error for UnknownVariant<T> {}
