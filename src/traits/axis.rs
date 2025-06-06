
use crate::libm;
use crate::core::{
    ops::{Add, Sub, Mul, Div, Rem, Neg},
    cmp::{PartialEq, PartialOrd},
    marker::{Copy, Sized}
};

/**
A representation of the real number line.

If the type can aproximate real numbers (eg: floating point) then it qualifyes.
If it's limited to a surtun number type (eg: integers) then it does not.
For a type to qualify it must represent one dimension line that is as contineous as possible.
It does not need to 

This is manualy implemented for [f32] and [f64] by default.

# Important

Depeanding on how this crate evolves and on what it needs, this trait will change and added.

# Implementation

If you want to implement this trait for a custom type `T` make sure the following holds:

`T::ZERO == -T::ZERO`

`T::ZERO < T::ERROR < T::ONE`

For any `a: T` (optionaly for `T::NAN`) -> `a == a`

For any `a: T` -> `a + T::ZERO == a`

For any `a: T` -> `a * T::ONE == a`

For any `a: T` -> `a * T::ZERO == T::ZERO`

For any `a: T` -> `T::ZERO - a == -a`

For any `a: T, b: T` -> `a + b == b + a`

For any `a: T, b: T` -> `a * b == b * a`

For any `a: T, b: T` -> `a / b == a * (T::ONE / b)`

For any `a: T, b: T` -> `(a + b) - b == a`
*/
pub trait Axis: Sized
              + Add<Output = Self>
              + Sub<Output = Self>
              + Mul<Output = Self>
              + Div<Output = Self>
              + Rem<Output = Self>
              + Neg<Output = Self>
              + PartialOrd
              + PartialEq
              + Copy
{
    /// The multiplicative identity.
    const ONE: Self;
    /// The additive identity.
    const ZERO: Self;
    /// An aproximation to the circumfrince of a circle divided by it's radius.
    /// 
    /// `TAU = 2 * PI`
    const TAU: Self;
    /// The representation of a `Not a Number` value.
    const NAN: Self;
    /// Used as the aporximative precision error for flaoting point arithmatic.
    const ERROR: Self;
    // /// The representation of the ∞ value.
    // const INF: Self;
    // /// The representation of the -∞ value.
    // const NEG_INF: Self;

    /// Checks to see if `self` is NAN. (`x == Self::NAN` is not guaranteed to work)
    fn is_nan(&self) -> bool;

    /// Calculates `(self * factor) + addend`.
    fn mul_add(self, factor: Self, addend: Self) -> Self;

    /// Calculates the square root of `self`.
    fn sqrt(self) -> Self;
    /// Calculates the `self` raised to the `exp` power.
    fn pow(self, exp: Self) -> Self;
    /// Calculates the sine and cosine of `self` at once.
    fn sin_cos(self) -> (Self, Self);
    /// Calculates the sine of `self`.
    #[inline]
    fn sin(self) -> Self { self.sin_cos().0 }
    /// Calculates the arcsine of `self`.
    fn asin(self) -> Self;
    /// Calculates the arcsine of `self`.
    fn sinh(self) -> Self {
        let exp = self.exp();
        (exp - Self::ONE / exp) / (Self::ONE + Self::ONE)
    }
    /// Calculates the cosine of `self`.
    #[inline]
    fn cos(self) -> Self { self.sin_cos().0 }
    /// Calculates the arccosine of `self`.
    fn acos(self) -> Self;
    /// Calculates the arccosine of `self`.
    fn cosh(self) -> Self {
        let exp = self.exp();
        (exp + Self::ONE / exp) / (Self::ONE + Self::ONE)
    }
    /// Calculates the arctangent of `self / bottom`.
    fn atan2( self, bottom: Self ) -> Self;
    /// Calculates [`e`](https://en.wikipedia.org/wiki/E_(mathematical_constant)) raised to the power of `self`.
    /// 
    /// `e ≈ 2.71828...`
    fn exp(self) -> Self;
    /// Calculates natural logarithm `self`.
    fn ln(self) -> Self;
    /// Calculates the absolute value of `self`.
    #[inline]
    fn abs(self) -> Self {
        if self < Self::ZERO { -self }
        else {self}
    }
    /// Gets the larget value between `self` and `other`.
    #[inline]
    fn max( self, other: Self ) -> Self {
        if self > other { self }
        else { other }
    }
    /// Gets the smaller value between `self` and `other`.
    #[inline]
    fn min( self, other: Self ) -> Self {
        if self < other { self }
        else { other }
    }
    
    /// Turns a [`f64`] into `Self`
    fn from_f64( float: f64 ) -> Self;
    
    // #[deprecated(note = "Use `from_f64` instead.")]
    /// Turns a [`u8`] into `Self` (Note: this could be decapricated)
    fn from_u8( uint: u8 ) -> Self {
        let mut out: Self = Self::ZERO;
        for _ in 0..uint {
            out = out + Self::ONE;
        }
        out
    }
}

impl Axis for f32 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
    const TAU: Self = crate::core::f32::consts::TAU;
    const NAN: Self = f32::NAN;
    const ERROR: Self = 0.00001525878; // 2 ^ -16

    #[inline]
    fn is_nan( &self ) -> bool { f32::is_nan(*self) }

    #[inline(always)]
    fn mul_add( self, factor: Self, addend: Self ) -> Self { self * factor + addend }

    #[inline(always)]
    fn sqrt( self ) -> Self { libm::sqrtf(self) }

    #[inline(always)]
    fn pow( self, exp: Self ) -> Self { libm::powf(self, exp) }
    
    #[inline(always)]
    fn sin_cos( self ) -> (Self, Self) { libm::sincosf(self) }

    #[inline(always)]
    fn sin( self ) -> Self { libm::sinf(self) }

    #[inline(always)]
    fn asin( self ) -> Self { libm::asinf(self) }

    #[inline(always)]
    fn sinh( self ) -> Self { libm::sinhf(self) }

    #[inline(always)]
    fn cos( self ) -> Self { libm::cosf(self) }

    #[inline(always)]
    fn acos( self ) -> Self { libm::acosf(self) }

    #[inline(always)]
    fn cosh( self ) -> Self { libm::coshf(self) }

    #[inline(always)]
    fn exp( self ) -> Self { libm::expf(self) }

    #[inline(always)]
    fn ln( self ) -> Self { libm::logf(self) }
    
    #[inline(always)]
    fn atan2( self, bottom: Self ) -> Self { libm::atan2f(self, bottom) }

    #[inline(always)]
    fn from_u8( uint: u8 ) -> Self { uint as Self }

    #[inline(always)]
    fn from_f64( float: f64 ) -> Self { float as Self }
}

impl Axis for f64 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
    const TAU: Self = crate::core::f64::consts::TAU;
    const NAN: Self = f64::NAN;
    const ERROR: Self = 0.00001525878; // 2 ^ -16

    #[inline]
    fn is_nan( &self ) -> bool { f64::is_nan(*self) }

    #[inline(always)]
    fn mul_add( self, factor: Self, addend: Self ) -> Self { self * factor + addend }

    #[inline(always)]
    fn sqrt( self ) -> Self { libm::sqrt(self) }

    #[inline(always)]
    fn pow( self, exp: Self ) -> Self { libm::pow(self, exp) }
    
    #[inline(always)]
    fn sin_cos( self ) -> (Self, Self) { libm::sincos(self) }
    
    #[inline(always)]
    fn sin( self ) -> Self { libm::sin(self) }

    #[inline(always)]
    fn asin( self ) -> Self { libm::asin(self) }
    
    #[inline(always)]
    fn sinh( self ) -> Self { libm::sinh(self) }
    
    #[inline(always)]
    fn cos( self ) -> Self { libm::cos(self) }
    
    #[inline(always)]
    fn acos( self ) -> Self { libm::acos(self) }
    
    #[inline(always)]
    fn cosh( self ) -> Self { libm::cosh(self) }
    
    #[inline(always)]
    fn exp( self ) -> Self { libm::exp(self) }
    
    #[inline(always)]
    fn ln( self ) -> Self { libm::log(self) }
    
    #[inline(always)]
    fn atan2( self, bottom: Self ) -> Self { libm::atan2(self, bottom) }

    #[inline(always)]
    fn from_u8( uint: u8 ) -> Self { uint as Self }

    #[inline(always)]
    fn from_f64( float: f64 ) -> Self { float }
}
