
use crate::libm;
use crate::core::{
    ops::{Add, Sub, Mul, Div, Neg},
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
              + Neg<Output = Self>
              + PartialOrd
              + PartialEq
              + Copy
              + 'static
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

    /// Calculates the square root of `self`.
    fn sqrt(self) -> Self;
    /// Calculates the sine of `self`.
    fn sin(self) -> Self;
    /// Calculates the arcsine of `self`.
    fn asin(self) -> Self;
    /// Calculates the arcsine of `self`.
    fn sinh(self) -> Self;
    /// Calculates the cosine of `self`.
    fn cos(self) -> Self;
    /// Calculates the arccosine of `self`.
    fn acos(self) -> Self;
    /// Calculates the arccosine of `self`.
    fn cosh(self) -> Self;
    /// Calculates the arctangent of `self / bottom`.
    fn atan2( self, bottom: Self ) -> Self;
    /// Calculates [`e`](https://en.wikipedia.org/wiki/E_(mathematical_constant)) raised to the power of `self`.
    /// 
    /// `e ≈ 2.71828...`
    fn exp(self) -> Self;
    /// Calculates natural logarithm `self`.
    fn ln(self) -> Self;
}

impl Axis for f32 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
    const TAU: Self = crate::core::f32::consts::TAU;
    const NAN: Self = f32::NAN;
    const ERROR: Self = 0.00001525878; // 2 ^ -16

    #[inline]
    fn is_nan( &self ) -> bool { self != self }

    #[inline(always)]
    fn sqrt( self ) -> Self { libm::sqrtf(self) }

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
}

impl Axis for f64 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
    const TAU: Self = crate::core::f64::consts::TAU;
    const NAN: Self = f64::NAN;
    const ERROR: Self = 0.00001525878; // 2 ^ -16

    #[inline]
    fn is_nan( &self ) -> bool { self != self }

    #[inline(always)]
    fn sqrt( self ) -> Self { libm::sqrt(self) }
    
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
}
