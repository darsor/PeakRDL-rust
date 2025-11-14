//! Types for numeric fixed-point field representations

/// A fixed-point number implementation using an underlying primitive integer type.
///
/// # Type Parameters
///
/// * `P` - The primitive integer type (signed or unsigned) used to store the fixed-point value
/// * `I` - The number of integer bits (can be negative for sub-integer representations)
/// * `F` - The number of fractional bits (can be negative for super-integer representations)
///
/// # Examples
///
/// Basic usage with different bit configurations:
///
/// ```
/// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
/// // 8-bit unsigned with 4 integer and 4 fractional bits
/// let fp = FixedPoint::<u8, 4, 4>::from_f64(2.25);
/// assert_eq!(fp.to_f64(), 2.25);
///
/// // 16-bit signed with 8 integer and 4 fractional bits
/// let fp = FixedPoint::<i16, 8, 4>::from_f64(-1.5);
/// assert_eq!(fp.to_bits(), -24);
/// ```
///
/// The total width is calculated as I + F:
///
/// ```
/// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
/// assert_eq!(FixedPoint::<u8, 8, 0>::width(), 8);
/// assert_eq!(FixedPoint::<i8, 7, -3>::width(), 4);
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FixedPoint<P, const I: isize, const F: isize> {
    val: P,
}

impl<P, const I: isize, const F: isize> core::fmt::Debug for FixedPoint<P, I, F>
where
    P: num_traits::PrimInt
        + num_traits::AsPrimitive<f64>
        + num_traits::WrappingSub
        + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write as _;
        let mut name: heapless::String<32> = heapless::String::new();
        write!(&mut name, "FixedPoint<{I},{F}>")
            .expect("Fixedpoint type name should fit in small buffer");
        f.debug_struct(&name)
            .field("int", &self.val)
            .field("real", &self.to_f64())
            .finish()
    }
}

impl<P, const I: isize, const F: isize> FixedPoint<P, I, F>
where
    P: num_traits::PrimInt + num_traits::WrappingSub,
{
    /// Creates a fixed-point number from its raw bit representation.
    ///
    /// # Panics
    ///
    /// - (At compile time) If the primitive type P is not wide enough for the specified I + F bit width
    /// - (At compile time) If I + F is not positive
    /// - If the provided bits would overflow the fixed-point representation
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// let fp = FixedPoint::<u8, 4, 4>::from_bits(16); // represents 1.0
    /// assert_eq!(fp.to_f64(), 1.0);
    /// ```
    ///
    /// The following should not compile:
    ///
    /// ```compile_fail
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// FixedPoint::<u8, 5, 4>::from_bits(0); // u8 not large enough
    /// ```
    ///
    /// ```compile_fail
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// FixedPoint::<i8, -5, 4>::from_bits(0); // invalid negative width
    /// ```
    #[must_use]
    pub fn from_bits(bits: P) -> Self {
        const {
            assert!(
                I + F <= 8 * (core::mem::size_of::<P>() as isize),
                "The primitive integer type is not wide enough for this fixed-point representation"
            );
            assert!(I + F > 0, "The fixed-point bit width must be positive");
        }
        assert!(
            (bits <= Self::max_bits()) && (bits >= Self::min_bits()),
            "The provided bits overflow this fixed-point representation"
        );
        Self { val: bits }
    }

    /// Returns the raw bit representation of the fixed-point number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// let fp = FixedPoint::<u16, 8, 2>::from_f64(2.25);
    /// assert_eq!(fp.to_bits(), 9);
    /// ```
    #[must_use]
    pub const fn to_bits(self) -> P {
        self.val
    }

    /// Returns the number of integer bits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u8, 10, -4>::intwidth(), 10);
    /// ```
    #[must_use]
    pub const fn intwidth() -> isize {
        I
    }

    /// Returns the number of fractional bits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u8, 10, -4>::fracwidth(), -4);
    /// ```
    #[must_use]
    pub const fn fracwidth() -> isize {
        F
    }

    /// Returns the total bit width (I + F) of the fixed-point representation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u8, 8, 0>::width(), 8);
    /// assert_eq!(FixedPoint::<i8, 7, -3>::width(), 4);
    /// ```
    #[must_use]
    pub const fn width() -> usize {
        (I + F) as usize
    }

    /// Returns true if the fixedpoint representation (underlying primitive type) is signed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u16, 8, 2>::is_signed(), false);
    /// assert_eq!(FixedPoint::<i16, 8, 2>::is_signed(), true);
    /// ```
    #[must_use]
    pub fn is_signed() -> bool {
        P::min_value() < P::zero()
    }

    /// Returns a fixed-point representation of zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// let zero = FixedPoint::<u8, 4, 4>::zero();
    /// assert_eq!(zero.to_f64(), 0.0);
    /// ```
    #[must_use]
    pub fn zero() -> Self {
        Self::from_bits(P::zero())
    }

    #[must_use]
    fn max_bits() -> P {
        let unused_bits = core::mem::size_of::<P>() * 8 - Self::width();
        P::max_value().shr(unused_bits)
    }

    #[must_use]
    fn min_bits() -> P {
        let unused_bits = core::mem::size_of::<P>() * 8 - Self::width();
        P::min_value().shr(unused_bits)
    }

    /// Returns the maximum representable value for this fixed-point type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u8, 2, 6>::max_value().to_f32(), 3.984375);
    /// assert_eq!(FixedPoint::<i8, 3, 4>::max_value().to_f32(), 3.9375);
    /// ```
    #[must_use]
    pub fn max_value() -> Self {
        Self::from_bits(Self::max_bits())
    }

    /// Returns the minimum representable value for this fixed-point type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u8, 2, 6>::min_value().to_f32(), 0.0);
    /// assert_eq!(FixedPoint::<i8, 3, 4>::min_value().to_f32(), -4.0);
    /// ```
    #[must_use]
    pub fn min_value() -> Self {
        Self::from_bits(Self::min_bits())
    }

    /// Returns the smallest representable positive value (the resolution).
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// let res = FixedPoint::<u8, 4, 4>::resolution();
    /// assert_eq!(res.to_f64(), 0.0625); // 2^(-4)
    /// ```
    #[must_use]
    pub fn resolution() -> Self {
        Self::from_bits(P::one())
    }

    /// Quantizes a floating-point value to the resolution of this fixed-point type
    /// and returns it as a floating-point value.
    ///
    /// This is equivalent to converting to fixed-point and back to floating-point.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// // 2.3 gets quantized to the nearest representable value
    /// let quantized = FixedPoint::<u8, 4, 4>::quantize(2.3);
    /// assert_eq!(quantized, 2.3125);
    /// ```
    pub fn quantize<T>(value: T) -> T
    where
        T: num_traits::Float + 'static,
        P: num_traits::AsPrimitive<T>,
    {
        Self::from_float(value).to_float()
    }

    /// Creates a fixed-point number from a 32-bit floating-point value.
    ///
    /// Values are rounded to the nearest representable fixed-point value.
    /// Ties are rounded away from 0.
    /// Out-of-range values are saturated to the min/max representable values.
    ///
    /// # Panics
    ///
    /// Panics if the input is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// let fp = FixedPoint::<u8, 4, 4>::from_f32(1.5);
    /// assert_eq!(fp.to_bits(), 24);
    ///
    /// // saturation behavior
    /// let min_fp = FixedPoint::<i8, 4, 4>::from_f64(-100.0);
    /// assert_eq!(min_fp, FixedPoint::<i8, 4, 4>::min_value());
    /// ```
    #[must_use]
    pub fn from_f32(value: f32) -> Self
    where
        P: num_traits::AsPrimitive<f32>,
    {
        Self::from_float(value)
    }

    /// Creates a fixed-point number from a 64-bit floating-point value.
    ///
    /// Values are rounded to the nearest representable fixed-point value.
    /// Ties are rounded away from 0.
    /// Out-of-range values are saturated to the min/max representable values.
    ///
    /// # Panics
    ///
    /// Panics if the input is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// let fp = FixedPoint::<u16, 8, 2>::from_f64(2.25);
    /// assert_eq!(fp.to_bits(), 9);
    ///
    /// // Saturation behavior
    /// let max_fp = FixedPoint::<u8, 4, 4>::from_f64(100.0);
    /// assert_eq!(max_fp, FixedPoint::<u8, 4, 4>::max_value());
    /// ```
    #[must_use]
    pub fn from_f64(value: f64) -> Self
    where
        P: num_traits::AsPrimitive<f64>,
    {
        Self::from_float(value)
    }

    #[must_use]
    fn from_float<T>(value: T) -> Self
    where
        T: num_traits::Float + 'static,
        P: num_traits::AsPrimitive<T>,
    {
        assert!(!value.is_nan(), "Can't convert NaN to FixedPoint");

        // scale
        #[allow(clippy::cast_possible_truncation)]
        let scale = T::from(2)
            .expect("two can be represented by any float type")
            .powi(F as i32);
        let scaled_value = value * scale;

        // saturate
        if scaled_value >= P::max_value().as_() {
            Self::from_bits(P::max_value())
        } else if scaled_value <= P::min_value().as_() {
            Self::from_bits(P::min_value())
        } else {
            // round
            Self::from_bits(
                P::from(scaled_value.round()).expect("shouldn't be NaN or out of range"),
            )
        }
    }

    /// Converts the fixed-point number to a 32-bit floating-point value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u16, 8, 2>::from_bits(8).to_f32(), 2.0);
    /// assert_eq!(FixedPoint::<u16, 8, 2>::from_bits(9).to_f32(), 2.25);
    /// assert_eq!(FixedPoint::<i16, 8, 4>::from_bits(-24).to_f32(), -1.5);
    /// assert_eq!(FixedPoint::<u8, 4, 4>::from_bits(1).to_f32(), 0.0625);
    /// assert_eq!(FixedPoint::<i8, 4, 4>::from_bits(-1).to_f32(), -0.0625);
    /// assert_eq!(FixedPoint::<u8, 4, 4>::from_bits(0).to_f32(), 0.0);
    /// ```
    #[must_use]
    pub fn to_f32(self) -> f32
    where
        P: num_traits::AsPrimitive<f32>,
    {
        self.to_float()
    }

    /// Converts the fixed-point number to a 64-bit floating-point value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
    /// assert_eq!(FixedPoint::<u16, 8, 2>::from_bits(8).to_f64(), 2.0);
    /// assert_eq!(FixedPoint::<u16, 8, 2>::from_bits(9).to_f64(), 2.25);
    /// assert_eq!(FixedPoint::<i16, 8, 4>::from_bits(-24).to_f64(), -1.5);
    /// assert_eq!(FixedPoint::<u8, 4, 4>::from_bits(1).to_f64(), 0.0625);
    /// assert_eq!(FixedPoint::<i8, 4, 4>::from_bits(-1).to_f64(), -0.0625);
    /// assert_eq!(FixedPoint::<u8, 4, 4>::from_bits(0).to_f64(), 0.0);
    /// ```
    #[must_use]
    pub fn to_f64(self) -> f64
    where
        P: num_traits::AsPrimitive<f64>,
    {
        self.to_float()
    }

    #[must_use]
    fn to_float<T>(self) -> T
    where
        T: num_traits::Float + 'static,
        P: num_traits::AsPrimitive<T>,
    {
        #[allow(clippy::cast_possible_truncation)]
        let scale = T::from(2)
            .expect("two can be represented by any float type")
            .powi(-F as i32);
        self.val.as_() * scale
    }
}

/// Automatic conversion from floating-point types to fixed-point.
///
/// This provides convenient syntax for creating fixed-point numbers from floats.
/// Note that this is a lossy conversion that will never fail (unless NaN). Saturation
/// and rounding are applied.
///
/// # Panics
///
/// Panics if the value is NaN.
///
/// # Examples
///
/// ```
/// # use {{ctx.crate_name}}::fixedpoint::FixedPoint;
/// let fp: FixedPoint<u8, 4, 4> = 2.5.into();
/// assert_eq!(fp.to_f64(), 2.5);
/// ```
impl<T, P, const I: isize, const F: isize> From<T> for FixedPoint<P, I, F>
where
    T: num_traits::Float + 'static,
    P: num_traits::PrimInt + num_traits::AsPrimitive<T> + num_traits::WrappingSub,
{
    fn from(value: T) -> Self {
        Self::from_float(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_float() {
        assert_eq!(FixedPoint::<u16, 8, 2>::from_float(2.25).to_bits(), 9);
        assert_eq!(FixedPoint::<i16, 8, 4>::from_float(-1.5).to_bits(), -24);
        assert_eq!(FixedPoint::<u8, 4, 4>::from_float(0.0625).to_bits(), 1);

        // Test rounding
        assert_eq!(FixedPoint::<u8, 4, 4>::from_float(0.03124).to_bits(), 0); // rounds down
        assert_eq!(FixedPoint::<u8, 4, 4>::from_float(0.03125).to_bits(), 1); // rounds ties away from 0
        assert_eq!(FixedPoint::<u8, 4, 4>::from_float(0.03126).to_bits(), 1); // rounds up
        assert_eq!(FixedPoint::<i8, 4, 4>::from_float(-0.03124).to_bits(), 0); // rounds up
        assert_eq!(FixedPoint::<i8, 4, 4>::from_float(-0.03125).to_bits(), -1); // rounds ties away from 0
        assert_eq!(FixedPoint::<i8, 4, 4>::from_float(-0.03126).to_bits(), -1); // rounds down

        // Test saturation - positive overflow
        assert_eq!(
            FixedPoint::<u8, 4, 4>::from_float(100.0),
            FixedPoint::<u8, 4, 4>::max_value()
        );
        assert_eq!(
            FixedPoint::<i8, 4, 4>::from_float(100.0),
            FixedPoint::<i8, 4, 4>::max_value()
        );

        // Test saturation - negative overflow
        assert_eq!(
            FixedPoint::<u8, 4, 4>::from_float(-100.0),
            FixedPoint::<u8, 4, 4>::min_value()
        );
        assert_eq!(
            FixedPoint::<i8, 4, 4>::from_float(-100.0),
            FixedPoint::<i8, 4, 4>::min_value()
        );
    }

    #[test]
    #[should_panic(expected = "Can't convert NaN to FixedPoint")]
    fn test_from_float_nan_panic() {
        let _ = FixedPoint::<u8, 4, 4>::from_float(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "The provided bits overflow this fixed-point representation")]
    fn test_positive_overflow1() {
        let _ = FixedPoint::<u8, 2, 4>::from_bits(64);
    }

    #[test]
    #[should_panic(expected = "The provided bits overflow this fixed-point representation")]
    fn test_negative_overflow() {
        let _ = FixedPoint::<i8, 2, 4>::from_bits(-33);
    }
}
