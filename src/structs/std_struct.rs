
use crate::quat;
use crate::Axis;
use crate::Quaternion;
use crate::QuaternionConstructor;
use crate::Scalar;
use crate::ScalarConstructor;
use crate::core::num::{
    NonZero,
    Wrapping,
    Saturating,
};

use crate::core::ops::{Add, Sub, Mul, Div, Neg, Rem};

use crate::std;

/**
Wrapper that changes the [`Axis`] methods from the [libm](https://docs.rs/libm/latest/libm/)
ones to the [std](https://doc.rust-lang.org/std/index.html) ones.
*/
#[repr(transparent)]
#[allow(private_bounds)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Std<T>(pub T);

/// A type alias for [`Std<f32>`](Std).
pub type Std32 = Std<f32>;
/// A type alias for [`Std<f64>`](Std).
pub type Std64 = Std<f64>;

#[allow(private_bounds)]
impl<Num: Axis> Std<Num> {
    /// Creates a new `Std<Num>`
    #[inline]
    pub const fn new(number: Num) -> Self {
        Std(number)
    }

    /// Gets the nubmer value.
    #[inline]
    pub const fn get(self) -> Num {
        self.0
    }

    /// Maps the number value of self into another one.
    #[inline]
    pub fn map(&mut self, change: impl crate::core::ops::FnOnce(Num) -> Num) {
        self.0 = change(self.0);
    }
}

impl<Num: Axis> crate::core::convert::From<Num> for Std<Num> {
    fn from(origin: Num) -> Std<Num> {
        Std(origin)
    }
}

macro_rules! impl_scalar_for_std {
    ( $for:ty : $( $impl:ty ),+ $(,)? ) => {
        impl crate::core::convert::From<Std<$for>> for $for {
            #[inline] fn from(origin: Std<$for>) -> $for {
                origin.0
            }
        }

        impl Add for Std<$for> {
            type Output = Self;
            #[inline] fn add(self, other: Self) -> Self {
                Std(self.0 + other.0)
            }
        }
    
        impl Sub for Std<$for> {
            type Output = Self;
            #[inline] fn sub(self, other: Self) -> Self {
                Std(self.0 - other.0)
            }
        }
    
        impl Mul for Std<$for> {
            type Output = Self;
            #[inline] fn mul(self, other: Self) -> Self {
                Std(self.0 * other.0)
            }
        }
    
        impl Div for Std<$for> {
            type Output = Self;
            #[inline] fn div(self, other: Self) -> Self {
                Std(self.0 / other.0)
            }
        }
    
        impl Rem for Std<$for> {
            type Output = Self;
            #[inline] fn rem(self, other: Self) -> Self {
                Std(self.0 % other.0)
            }
        }
    
        impl Neg for Std<$for> {
            type Output = Self;
            #[inline] fn neg(self) -> Self {
                Std(-self.0)
            }
        }
    
        impl crate::core::str::FromStr for Std<$for> {
            type Err = <$for as crate::core::str::FromStr>::Err;

            #[inline] fn from_str(s: &str) -> crate::core::result::Result<Self, Self::Err> {
                crate::core::result::Result::Ok(Std(<$for as crate::core::str::FromStr>::from_str(s)?))
            }
        }

        impl Scalar<Std<$for>> for f32 {
            #[inline]fn scalar( &self ) -> Std<$for> {
                Std(self.scalar())
            }
        }

        impl Scalar<Std<$for>> for f64 {
            #[inline] fn scalar( &self ) -> Std<$for> {
                Std(self.scalar())
            }
        }

        impl Scalar<f32> for Std<$for> {
            #[inline]fn scalar( &self ) -> f32 {
                self.0 as f32
            }
        }

        impl Scalar<f64> for Std<$for> {
            #[inline] fn scalar( &self ) -> f64 {
                self.0 as f64
            }
        }

        impl Scalar<Std<$for>> for &f32 {
            #[inline]fn scalar( &self ) -> Std<$for> {
                Std(self.scalar())
            }
        }

        impl Scalar<Std<$for>> for &f64 {
            #[inline] fn scalar( &self ) -> Std<$for> {
                Std(self.scalar())
            }
        }

        impl Scalar<f32> for &Std<$for> {
            #[inline]fn scalar( &self ) -> f32 {
                self.0 as f32
            }
        }

        impl Scalar<f64> for &Std<$for> {
            #[inline] fn scalar( &self ) -> f64 {
                self.0 as f64
            }
        }

        $(
            impl Scalar<Std<$for>> for $impl {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }

            impl Scalar<Std<$for>> for NonZero<$impl> {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }

            impl Scalar<Std<$for>> for Saturating<$impl> {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }

            impl Scalar<Std<$for>> for Wrapping<$impl> {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }
            
            impl Scalar<Std<$for>> for &$impl {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }

            impl Scalar<Std<$for>> for &NonZero<$impl> {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }

            impl Scalar<Std<$for>> for &Saturating<$impl> {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }

            impl Scalar<Std<$for>> for &Wrapping<$impl> {
                #[inline] fn scalar( &self ) -> Std<$for> {
                    Std(self.scalar())
                }
            }

            impl ScalarConstructor<Std<$for>> for crate::core::option::Option<NonZero<$impl>> {
                #[inline] fn new_scalar( axis: Std<$for> ) -> Self {
                    Self::new_scalar(axis.0)
                }
            }

            impl ScalarConstructor<Std<$for>> for Saturating<$impl> {
                #[inline] fn new_scalar( axis: Std<$for> ) -> Self {
                    Self::new_scalar(axis.0)
                }
            }

            impl ScalarConstructor<Std<$for>> for Wrapping<$impl> {
                #[inline] fn new_scalar( axis: Std<$for> ) -> Self {
                    Self::new_scalar(axis.0)
                }
            }
        )+

        #[cfg(feature = "num-traits")]
        impl crate::num_traits::ToPrimitive for Std<$for> {
            fn to_i64(&self) -> crate::core::option::Option<i64> { crate::core::option::Option::Some(self.0 as i64) }

            fn to_u64(&self) -> crate::core::option::Option<u64> { crate::core::option::Option::Some(self.0 as u64) }

            fn to_i128(&self) -> crate::core::option::Option<i128> { crate::core::option::Option::Some(self.0 as i128) }

            fn to_u128(&self) -> crate::core::option::Option<u128> { crate::core::option::Option::Some(self.0 as u128) }

            fn to_f32(&self) -> crate::core::option::Option<f32> { crate::core::option::Option::Some(self.0 as f32) }

            fn to_f64(&self) -> crate::core::option::Option<f64> { crate::core::option::Option::Some(self.0 as f64) }
        }

        #[cfg(feature = "num-traits")]
        impl crate::num_traits::Num for Std<$for> {
            type FromStrRadixErr = <$for as crate::num_traits::Num>::FromStrRadixErr;
            #[inline]
            fn from_str_radix(
                str: &str,
                radix: u32,
            ) -> crate::core::result::Result<Self, Self::FromStrRadixErr> {
                crate::core::result::Result::Ok(Std(<$for>::from_str_radix(str, radix)?))
            }
        }

        #[cfg(feature = "num-traits")]
        impl crate::num_traits::Zero for Std<$for> {
            #[inline]
            fn zero() -> Self {
                Std(0.0)
            }
            #[inline]
            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }

        #[cfg(feature = "num-traits")]
        impl crate::num_traits::One for Std<$for> {
            #[inline]
            fn one() -> Self {
                Std(1.0)
            }
            #[inline]
            fn is_one(&self) -> bool {
                self.0.is_one()
            }
        }

        #[cfg(feature = "num-traits")]
        impl crate::num_traits::Float for Std<$for> {
            #[inline] fn nan() -> Self { Std(<$for as crate::num_traits::Float>::nan()) }
            #[inline] fn infinity() -> Self { Std(<$for as crate::num_traits::Float>::infinity()) }
            #[inline] fn neg_infinity() -> Self { Std(<$for as crate::num_traits::Float>::neg_infinity()) }
            #[inline] fn neg_zero() -> Self { Std(<$for as crate::num_traits::Float>::neg_zero()) }
            #[inline] fn min_value() -> Self { Std(<$for as crate::num_traits::Float>::min_value()) }
            #[inline] fn min_positive_value() -> Self { Std(<$for as crate::num_traits::Float>::min_positive_value()) }
            #[inline] fn max_value() -> Self { Std(<$for as crate::num_traits::Float>::max_value()) }
            #[inline] fn is_nan(self) -> bool { <$for as crate::num_traits::Float>::is_nan(self.0) }
            #[inline] fn is_infinite(self) -> bool { <$for as crate::num_traits::Float>::is_infinite(self.0) }
            #[inline] fn is_finite(self) -> bool { <$for as crate::num_traits::Float>::is_finite(self.0) }
            #[inline] fn is_normal(self) -> bool { <$for as crate::num_traits::Float>::is_normal(self.0) }
            #[inline] fn classify(self) -> crate::core::num::FpCategory { <$for as crate::num_traits::Float>::classify(self.0) }
            #[inline] fn floor(self) -> Self { Std(<$for as crate::num_traits::Float>::floor(self.0)) }
            #[inline] fn ceil(self) -> Self { Std(<$for as crate::num_traits::Float>::ceil(self.0)) }
            #[inline] fn round(self) -> Self { Std(<$for as crate::num_traits::Float>::round(self.0)) }
            #[inline] fn trunc(self) -> Self { Std(<$for as crate::num_traits::Float>::trunc(self.0)) }
            #[inline] fn fract(self) -> Self { Std(<$for as crate::num_traits::Float>::fract(self.0)) }
            #[inline] fn abs(self) -> Self { Std(<$for as crate::num_traits::Float>::abs(self.0)) }
            #[inline] fn signum(self) -> Self { Std(<$for as crate::num_traits::Float>::signum(self.0)) }
            #[inline] fn is_sign_positive(self) -> bool { <$for as crate::num_traits::Float>::is_sign_positive(self.0) }
            #[inline] fn is_sign_negative(self) -> bool { <$for as crate::num_traits::Float>::is_sign_negative(self.0) }
            #[inline] fn mul_add(self, factor: Self, addend: Self) -> Self { Std(<$for as crate::num_traits::Float>::mul_add(self.0, factor.0, addend.0)) }
            #[inline] fn recip(self) -> Self { Std(<$for as crate::num_traits::Float>::recip(self.0)) }
            #[inline] fn powi(self, exp: i32) -> Self { Std(<$for as crate::num_traits::Float>::powi(self.0, exp)) }
            #[inline] fn powf(self, exp: Self) -> Self { Std(<$for as crate::num_traits::Float>::powf(self.0, exp.0)) }
            #[inline] fn sqrt(self) -> Self { Std(<$for as crate::num_traits::Float>::sqrt(self.0)) }
            #[inline] fn exp(self) -> Self { Std(<$for as crate::num_traits::Float>::exp(self.0)) }
            #[inline] fn exp2(self) -> Self { Std(<$for as crate::num_traits::Float>::exp2(self.0)) }
            #[inline] fn ln(self) -> Self { Std(<$for as crate::num_traits::Float>::ln(self.0)) }
            #[inline] fn log(self, base: Self) -> Self { Std(<$for as crate::num_traits::Float>::log(self.0, base.0)) }
            #[inline] fn log2(self) -> Self { Std(<$for as crate::num_traits::Float>::log2(self.0)) }
            #[inline] fn log10(self) -> Self { Std(<$for as crate::num_traits::Float>::log10(self.0)) }
            #[inline] fn max(self, other: Self) -> Self { Std(<$for as crate::num_traits::Float>::max(self.0, other.0)) }
            #[inline] fn min(self, other: Self) -> Self { Std(<$for as crate::num_traits::Float>::min(self.0, other.0)) }
            #[inline] fn abs_sub(self, other: Self) -> Self { Std(<$for as crate::num_traits::Float>::abs_sub(self.0, other.0)) }
            #[inline] fn cbrt(self) -> Self { Std(<$for as crate::num_traits::Float>::cbrt(self.0)) }
            #[inline] fn hypot(self, other: Self) -> Self { Std(<$for as crate::num_traits::Float>::hypot(self.0, other.0)) }
            #[inline] fn sin(self) -> Self { Std(<$for as crate::num_traits::Float>::sin(self.0)) }
            #[inline] fn cos(self) -> Self { Std(<$for as crate::num_traits::Float>::cos(self.0)) }
            #[inline] fn tan(self) -> Self { Std(<$for as crate::num_traits::Float>::tan(self.0)) }
            #[inline] fn asin(self) -> Self { Std(<$for as crate::num_traits::Float>::asin(self.0)) }
            #[inline] fn acos(self) -> Self { Std(<$for as crate::num_traits::Float>::acos(self.0)) }
            #[inline] fn atan(self) -> Self { Std(<$for as crate::num_traits::Float>::atan(self.0)) }
            #[inline] fn atan2(self, other: Self) -> Self { Std(<$for as crate::num_traits::Float>::atan2(self.0, other.0)) }
            #[inline] fn sin_cos(self) -> (Self, Self) {
                let (sin, cos) = <$for as crate::num_traits::Float>::sin_cos(self.0);
                (Std(sin), Std(cos))
            }
            #[inline] fn exp_m1(self) -> Self { Std(<$for as crate::num_traits::Float>::exp_m1(self.0)) }
            #[inline] fn ln_1p(self) -> Self { Std(<$for as crate::num_traits::Float>::ln_1p(self.0)) }
            #[inline] fn sinh(self) -> Self { Std(<$for as crate::num_traits::Float>::sinh(self.0)) }
            #[inline] fn cosh(self) -> Self { Std(<$for as crate::num_traits::Float>::cosh(self.0)) }
            #[inline] fn tanh(self) -> Self { Std(<$for as crate::num_traits::Float>::tanh(self.0)) }
            #[inline] fn asinh(self) -> Self { Std(<$for as crate::num_traits::Float>::asinh(self.0)) }
            #[inline] fn acosh(self) -> Self { Std(<$for as crate::num_traits::Float>::acosh(self.0)) }
            #[inline] fn atanh(self) -> Self { Std(<$for as crate::num_traits::Float>::atanh(self.0)) }
            #[inline] fn integer_decode(self) -> (u64, i16, i8) { <$for as crate::num_traits::Float>::integer_decode(self.0) }
        }

    };
}

impl_scalar_for_std!{
    f32:
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl_scalar_for_std!{
    f64:
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(feature = "num-traits")]
impl crate::num_traits::NumCast for Std<f32> {
    #[inline]
    fn from<T>(num: T) -> crate::core::option::Option<Self>
    where T: crate::num_traits::ToPrimitive
    {
        crate::core::option::Option::Some(Std(num.to_f32()?))
    }
}

#[cfg(feature = "num-traits")]
impl crate::num_traits::NumCast for Std<f64> {
    #[inline]
    fn from<T>(num: T) -> crate::core::option::Option<Self>
    where T: crate::num_traits::ToPrimitive
    {
        crate::core::option::Option::Some(Std(num.to_f64()?))
    }
}

#[cfg(feature = "serde")]
use crate::serde::{
    Serialize,
    Serializer,
    Deserialize,
    Deserializer,
};

#[cfg(feature = "serde")]
impl<Num: Axis + Serialize> Serialize for Std<Num> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> crate::core::result::Result<S::Ok, S::Error>
    where S: Serializer
    { self.0.serialize(serializer) }
}

#[cfg(feature = "serde")]
impl<'de, Num: Axis + Deserialize<'de>> Deserialize<'de> for Std<Num> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> crate::core::result::Result<Self, D::Error>
    where D: Deserializer<'de>
    { crate::core::result::Result::Ok(Std(Num::deserialize(deserializer)?)) }
}

#[inline]
/// Turns a quaternion that returns std values into a quaternion that returns normal ones.
pub fn from_quat_std<Num, Out>(quaternion: impl Quaternion<Std<Num>>) -> Out
where 
    Num: Axis,
    Std<Num>: Axis,
    Out: QuaternionConstructor<Num>
{
    let [Std(r), Std(i), Std(j), Std(k)] = quat::convert_quat(quaternion);
    Out::new_quat(r, i, j, k)
}

#[inline]
/// Turns a vector that returns std values into a vector that returns normal ones.
pub fn from_vector_std<Num, Out>(vector: impl crate::Vector<Std<Num>>) -> Out
where 
    Num: Axis,
    Std<Num>: Axis,
    Out: crate::VectorConstructor<Num>
{
    let [Std(x), Std(y), Std(z)] = <[Std<Num>; 3] as crate::VectorConstructor<Std<Num>>>::from_vector(vector);
    Out::new_vector(x, y, z)
}

#[inline]
/// Turns a vector that returns std values into a vector that returns normal ones.
pub fn from_complex_std<Num, Out>(vector: impl crate::Complex<Std<Num>>) -> Out
where 
    Num: Axis,
    Std<Num>: Axis,
    Out: crate::ComplexConstructor<Num>
{
    let [Std(real), Std(imaginary)] = <[Std<Num>; 2] as crate::ComplexConstructor<Std<Num>>>::from_complex(vector);
    Out::new_complex(real, imaginary)
}

#[inline]
/// Turns a vector that returns std values into a vector that returns normal ones.
pub fn from_scalar_std<Num, Out>(vector: impl crate::Scalar<Std<Num>>) -> Out
where 
    Num: Axis,
    Std<Num>: Axis,
    Out: crate::ScalarConstructor<Num>
{
    let Std(scalar) = <Std<Num> as crate::ScalarConstructor<Std<Num>>>::from_scalar(vector);
    Out::new_scalar(scalar)
}

impl Axis for Std<f32> {
    const ONE: Self = Std(1.0);
    const ZERO: Self = Std(0.0);
    const TAU: Self = Std(f32::TAU);
    const NAN: Self = Std(f32::NAN);
    const ERROR: Self = Std(f32::EPSILON);
    #[inline] fn is_nan( &self ) -> bool { std::primitive::f32::is_nan(self.0) }
    #[inline] fn mul_add( self, factor: Self, addend: Self ) -> Self { Std(std::primitive::f32::mul_add(self.0, factor.0, addend.0)) }
    #[inline] fn sqrt( self ) -> Self { Std(std::primitive::f32::sqrt(self.0)) }
    #[inline] fn pow( self, exp: Self ) -> Self { Std(std::primitive::f32::pow(self.0, exp.0)) }
    #[inline] fn sin_cos( self ) -> (Self, Self) {
        let (sin, cos) = std::primitive::f32::sin_cos(self.0);
        ( Std(sin), Std(cos) )
    }
    #[inline] fn sin( self ) -> Self { Std(std::primitive::f32::sin(self.0)) }
    #[inline] fn asin( self ) -> Self { Std(std::primitive::f32::asin(self.0)) }
    #[inline] fn sinh( self ) -> Self { Std(std::primitive::f32::sinh(self.0)) }
    #[inline] fn cos( self ) -> Self { Std(std::primitive::f32::cos(self.0)) }
    #[inline] fn acos( self ) -> Self { Std(std::primitive::f32::acos(self.0)) }
    #[inline] fn cosh( self ) -> Self { Std(std::primitive::f32::cosh(self.0)) }
    #[inline] fn atan2( self, other: Self ) -> Self { Std(std::primitive::f32::atan2(self.0, other.0)) }
    #[inline] fn exp( self ) -> Self { Std(std::primitive::f32::exp(self.0)) }
    #[inline] fn ln( self ) -> Self { Std(std::primitive::f32::ln(self.0)) }
    #[inline] fn from_u8( uint: u8 ) -> Self { Std( uint as f32) }
    #[inline(always)] fn from_f64( float: f64 ) -> Self { Std(float as f32) }
}
    
impl Axis for Std<f64> {
    const ONE: Self = Std(1.0);
    const ZERO: Self = Std(0.0);
    const TAU: Self = Std(f64::TAU);
    const NAN: Self = Std(f64::NAN);
    const ERROR: Self = Std(f64::EPSILON);
    #[inline] fn is_nan( &self ) -> bool { std::primitive::f64::is_nan(self.0) }
    #[inline] fn mul_add( self, factor: Self, addend: Self ) -> Self { Std(std::primitive::f64::mul_add(self.0, factor.0, addend.0)) }
    #[inline] fn sqrt( self ) -> Self { Std(std::primitive::f64::sqrt(self.0)) }
    #[inline] fn pow( self, exp: Self ) -> Self { Std(std::primitive::f64::pow(self.0, exp.0)) }
    #[inline] fn sin_cos( self ) -> (Self, Self) {
        let (sin, cos) = std::primitive::f64::sin_cos(self.0);
        ( Std(sin), Std(cos) )
    }
    #[inline] fn sin( self ) -> Self { Std(std::primitive::f64::sin(self.0)) }
    #[inline] fn asin( self ) -> Self { Std(std::primitive::f64::asin(self.0)) }
    #[inline] fn sinh( self ) -> Self { Std(std::primitive::f64::sinh(self.0)) }
    #[inline] fn cos( self ) -> Self { Std(std::primitive::f64::cos(self.0)) }
    #[inline] fn acos( self ) -> Self { Std(std::primitive::f64::acos(self.0)) }
    #[inline] fn cosh( self ) -> Self { Std(std::primitive::f64::cosh(self.0)) }
    #[inline] fn atan2( self, other: Self ) -> Self { Std(std::primitive::f64::atan2(self.0, other.0)) }
    #[inline] fn exp( self ) -> Self { Std(std::primitive::f64::exp(self.0)) }
    #[inline] fn ln( self ) -> Self { Std(std::primitive::f64::ln(self.0)) }
    #[inline] fn from_u8( uint: u8 ) -> Self { Std( uint as f64) }
    #[inline(always)] fn from_f64( float: f64 ) -> Self { Std(float) }
}

impl<Num: Axis, Q> crate::Quaternion<Std<Num>> for Std<Q>
where
    Q: crate::Quaternion<Num>,
    Std<Num>: Axis,
{
    #[inline] fn r(&self) -> Std<Num> { Std(self.0.r()) }
    #[inline] fn i(&self) -> Std<Num> { Std(self.0.i()) }
    #[inline] fn j(&self) -> Std<Num> { Std(self.0.j()) }
    #[inline] fn k(&self) -> Std<Num> { Std(self.0.k()) }
}

impl<Num: Axis, Q> crate::QuaternionConstructor<Std<Num>> for Std<Q>
where
    Q: crate::QuaternionConstructor<Num>,
    Std<Num>: Axis,
{
    #[inline] fn new_quat(r: Std<Num>, i: Std<Num>, j: Std<Num>, k: Std<Num>) -> Self {
        Std(Q::new_quat(r.0, i.0, j.0, k.0))
    }

    #[inline] fn from_quat(quaternion: impl crate::Quaternion<Std<Num>>) -> Self {
        let [Std(r), Std(i), Std(j), Std(k)] = quat::convert_quat(quaternion);
        Std(Q::from_quat([r, i, j, k]))
    }

    #[inline] fn origin() -> Self { Std(Q::origin()) }
    #[inline] fn identity() -> Self { Std(Q::identity()) }
    #[inline] fn nan() -> Self { Std(Q::nan()) }
    #[inline] fn unit_r() -> Self { Std(Q::unit_r()) }
    #[inline] fn unit_i() -> Self { Std(Q::unit_i()) }
    #[inline] fn unit_j() -> Self { Std(Q::unit_j()) }
    #[inline] fn unit_k() -> Self { Std(Q::unit_k()) }
}

impl<Num: Axis, Q> crate::QuaternionConsts<Std<Num>> for Std<Q>
where
    Q: crate::QuaternionConsts<Num>,
    Std<Num>: Axis,
{
    const ORIGIN: Self = Std(Q::ORIGIN);
    const IDENTITY: Self = Std(Q::IDENTITY);
    const NAN: Self = Std(Q::NAN);

    const UNIT_R: Self = Std(Q::UNIT_R);
    const UNIT_I: Self = Std(Q::UNIT_I);
    const UNIT_J: Self = Std(Q::UNIT_J);
    const UNIT_K: Self = Std(Q::UNIT_K);
}

impl<Num: Axis, Q> crate::QuaternionMethods<Std<Num>> for Std<Q>
where
    Q: crate::QuaternionMethods<Num>,
    Std<Num>: Axis,
{ }

impl<Num: Axis, V> crate::Vector<Std<Num>> for Std<V>
where
    V: crate::Vector<Num>,
    Std<Num>: Axis,
{
    #[inline] fn x(&self) -> Std<Num> { Std(self.0.x()) }
    #[inline] fn y(&self) -> Std<Num> { Std(self.0.y()) }
    #[inline] fn z(&self) -> Std<Num> { Std(self.0.z()) }
}

impl<Num: Axis, Q> crate::VectorConstructor<Std<Num>> for Std<Q>
where
    Q: crate::VectorConstructor<Num>,
    Std<Num>: Axis,
{
    #[inline] fn new_vector(x: Std<Num>, y: Std<Num>, z: Std<Num>) -> Self {
        Std(Q::new_vector(x.0, y.0, z.0))
    }

    #[inline] fn from_vector(vector: impl crate::Vector<Std<Num>>) -> Self {
        Std(Q::from_vector(from_vector_std::<Num, [Num; 3]>(vector)))
    }
}

impl<Num: Axis, V> crate::VectorConsts<Std<Num>> for Std<V>
where
    V: crate::VectorConsts<Num>,
    Std<Num>: Axis,
{
    const ORIGIN: Self = Std(V::ORIGIN);
    const NAN: Self = Std(V::NAN);

    const UNIT_X: Self = Std(V::UNIT_X);
    const UNIT_Y: Self = Std(V::UNIT_Y);
    const UNIT_Z: Self = Std(V::UNIT_Z);
}

impl<Num: Axis, C> crate::Complex<Std<Num>> for Std<C>
where
    C: crate::Complex<Num>,
    Std<Num>: Axis,
{
    #[inline] fn real(&self) -> Std<Num> { Std(self.0.real()) }
    #[inline] fn imaginary(&self) -> Std<Num> { Std(self.0.imaginary()) }
}

impl<Num: Axis, C> crate::ComplexConstructor<Std<Num>> for Std<C>
where
    C: crate::ComplexConstructor<Num>,
    Std<Num>: Axis,
{
    #[inline] fn new_complex(real: Std<Num>, imaginary: Std<Num>) -> Self {
        Std(C::new_complex(real.0, imaginary.0))
    }

    #[inline] fn from_complex(complex: impl crate::Complex<Std<Num>>) -> Self {
        Std(C::from_complex(from_complex_std::<Num, [Num; 2]>(complex)))
    }
}

impl<Num: Axis, C> crate::ComplexConsts<Std<Num>> for Std<C>
where
    C: crate::ComplexConsts<Num>,
    Std<Num>: Axis,
{
    const ORIGIN: Self = Std(C::ORIGIN);
    const IDENTITY: Self = Std(C::IDENTITY);
    const NAN: Self = Std(C::NAN);

    const UNIT_REAL: Self = Std(C::UNIT_REAL);
    const UNIT_IMAGINARY: Self = Std(C::UNIT_IMAGINARY);
}
