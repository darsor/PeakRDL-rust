//! Register endianness implementations

use num_traits::PrimInt;

/// Endianness of a register
#[allow(private_bounds)]
pub trait Endian: Sealed + Copy {
    /// Convert from native endianness to register endianness.
    fn to_register_endian<T: PrimInt>(value: T) -> T;

    /// Convert from register endianness to native endianness.
    fn from_register_endian<T: PrimInt>(value: T) -> T;

    /// Given the address order of a subword, return the sigificance order
    /// of the subword.
    ///
    /// For example, if `address_order_to_significance(1, 2) == 0`, it indicates
    /// that the subword at the second-to-lowest address (`address_order == 1`)
    /// is the least significant subword in the register (`== 0`).
    fn address_order_to_significance(address_order: usize, num_subwords: usize) -> usize;
}

/// Big endian byte and word ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BigEndian;
/// Little endian byte and word ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LittleEndian;
/// Little endian byte ordering with big endian word ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordBigByteLittleEndian;
/// Big endian byte ordering with little endian word ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordLittleByteBigEndian;

trait Sealed {}
impl Sealed for BigEndian {}
impl Sealed for LittleEndian {}
impl Sealed for WordBigByteLittleEndian {}
impl Sealed for WordLittleByteBigEndian {}

impl Endian for BigEndian {
    fn to_register_endian<T: PrimInt>(value: T) -> T {
        value.to_be()
    }

    fn from_register_endian<T: PrimInt>(value: T) -> T {
        T::from_be(value)
    }

    fn address_order_to_significance(address_order: usize, num_subwords: usize) -> usize {
        // The lowest address is the most significant word
        num_subwords - 1 - address_order
    }
}

impl Endian for LittleEndian {
    fn to_register_endian<T: PrimInt>(value: T) -> T {
        value.to_le()
    }

    fn from_register_endian<T: PrimInt>(value: T) -> T {
        T::from_le(value)
    }

    fn address_order_to_significance(address_order: usize, _num_subwords: usize) -> usize {
        // The lowest address is the least significant word
        address_order
    }
}

impl Endian for WordBigByteLittleEndian {
    fn to_register_endian<T: PrimInt>(value: T) -> T {
        value.to_le()
    }

    fn from_register_endian<T: PrimInt>(value: T) -> T {
        T::from_le(value)
    }

    fn address_order_to_significance(address_order: usize, num_subwords: usize) -> usize {
        // The lowest address is the most significant word
        num_subwords - 1 - address_order
    }
}

impl Endian for WordLittleByteBigEndian {
    fn to_register_endian<T: PrimInt>(value: T) -> T {
        value.to_be()
    }

    fn from_register_endian<T: PrimInt>(value: T) -> T {
        T::from_be(value)
    }

    fn address_order_to_significance(address_order: usize, _num_subwords: usize) -> usize {
        // The lowest address is the least significant word
        address_order
    }
}
