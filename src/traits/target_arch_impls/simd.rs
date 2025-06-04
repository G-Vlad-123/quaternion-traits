
use super::*;
use crate::core::simd::{
    Simd,
    SimdElement,
};

impl<Num> Quaternion<Num> for Simd<Num, 4>
where
    Num: SimdElement + Axis,
{
    #[inline] fn r(&self) -> Num { self[0] }
    #[inline] fn i(&self) -> Num { self[1] }
    #[inline] fn j(&self) -> Num { self[2] }
    #[inline] fn k(&self) -> Num { self[3] }
}

impl<Num> QuaternionConstructor<Num> for Simd<Num, 4>
where
    Num: SimdElement + Axis,
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> Self {
        Simd::from_array([r, i, j, k])
    }
}

impl<Num> QuaternionConsts<Num> for Simd<Num, 4>
where
    Num: SimdElement + Axis,
{
    const ORIGIN: Self = Simd::from_array([Num::ZERO, Num::ZERO, Num::ZERO, Num::ZERO]);
    const IDENTITY: Self = Simd::from_array([Num::ONE, Num::ZERO, Num::ZERO, Num::ZERO]);
    const NAN: Self = Simd::from_array([Num::NAN, Num::NAN, Num::NAN, Num::NAN]);
    const UNIT_I: Self = Simd::from_array([Num::ZERO, Num::ONE, Num::ZERO, Num::ZERO]);
    const UNIT_J: Self = Simd::from_array([Num::ZERO, Num::ZERO, Num::ONE, Num::ZERO]);
    const UNIT_K: Self = Simd::from_array([Num::ZERO, Num::ZERO, Num::ZERO, Num::ONE]);
}

impl<Num> QuaternionMethods<Num> for Simd<Num, 4>
where
    Num: SimdElement + Axis,
    Simd<Num, 4>
        : crate::core::ops::Add<Output = Self>
        + crate::core::ops::Sub<Output = Self>
        + crate::core::ops::Mul<Output = Self>
        + crate::core::ops::Div<Output = Self>
        + crate::core::ops::Neg<Output = Self>
{
    #[inline] fn add(self, other: impl Quaternion<Num>) -> Self {
        self + Simd::from_quat(other)
    }

    #[inline] fn sub(self, other: impl Quaternion<Num>) -> Self {
        self - Simd::from_quat(other)
    }

    fn mul(self, other: impl Quaternion<Num>) -> Self {
        let mut other = Simd::from_quat(other);
        let mut quat = Simd::splat(self[0]);
        quat *= other;
        other = Simd::from_array([other[1], other[0], other[3], other[2]]);
        quat += Simd::from_array([-Num::ONE, Num::ONE, -Num::ONE, Num::ONE]) * Simd::splat(self[1]) * other;
        other = other.reverse();
        quat += Simd::from_array([-Num::ONE, Num::ONE, Num::ONE, -Num::ONE]) * Simd::splat(self[2]) * other;
        other = Simd::from_array([other[1], other[0], other[3], other[2]]);
        quat += Simd::from_array([-Num::ONE, -Num::ONE, Num::ONE, Num::ONE]) * Simd::splat(self[3]) * other;
        quat
    }

    fn div(self, other: impl Quaternion<Num>) -> Self {
        let mut other = Simd::from_quat(&other) / Simd::splat(crate::quat::abs_squared(&other));
        let mut quat = Simd::splat(self[0]);
        quat *= other * Simd::from_array([Num::ONE, -Num::ONE, -Num::ONE, -Num::ONE]);
        other = Simd::from_array([other[1], other[0], other[3], other[2]]);
        quat += Simd::from_array([-Num::ONE, -Num::ONE, Num::ONE, -Num::ONE]) * Simd::splat(self[1]) * other;
        other = other.reverse();
        quat += Simd::from_array([-Num::ONE, -Num::ONE, -Num::ONE, Num::ONE]) * Simd::splat(self[2]) * other;
        other = Simd::from_array([other[1], other[0], other[3], other[2]]);
        quat += Simd::from_array([-Num::ONE, Num::ONE, -Num::ONE, -Num::ONE]) * Simd::splat(self[3]) * other;
        quat
    }

    #[inline] fn neg(self) -> Self { -self }

    #[inline] fn inv(self) -> Self {
        self * Simd::from_array([Num::ONE, -Num::ONE, -Num::ONE, -Num::ONE]) * Simd::splat(Num::ONE / self.abs_squared())
    }

    #[inline] fn conj(self) -> Self {
        self * Simd::from_array([Num::ONE, -Num::ONE, -Num::ONE, -Num::ONE])
    }
}

impl<Num> Vector<Num> for Simd<Num, 3>
where
    Num: SimdElement + Axis,
{
    #[inline] fn x(&self) -> Num { self[0] }
    #[inline] fn y(&self) -> Num { self[1] }
    #[inline] fn z(&self) -> Num { self[2] }
}

impl<Num> VectorConstructor<Num> for Simd<Num, 3>
where
    Num: SimdElement + Axis,
{
    #[inline] fn new_vector(x: Num, y: Num, z: Num) -> Self {
        Simd::from_array([x, y, z])
    }
}

impl<Num> VectorConsts<Num> for Simd<Num, 3>
where
    Num: SimdElement + Axis,
{
    const ORIGIN: Self = Simd::from_array([Num::ZERO, Num::ZERO, Num::ZERO]);
    const NAN: Self = Simd::from_array([Num::NAN, Num::NAN, Num::NAN]);
    const UNIT_X: Self = Simd::from_array([Num::ONE, Num::ZERO, Num::ZERO]);
    const UNIT_Y: Self = Simd::from_array([Num::ZERO, Num::ONE, Num::ZERO]);
    const UNIT_Z: Self = Simd::from_array([Num::ZERO, Num::ZERO, Num::ONE]);
}

impl<Num> Complex<Num> for Simd<Num, 2>
where
    Num: SimdElement + Axis,
{
    #[inline] fn real(&self) -> Num { self[0] }
    #[inline] fn imaginary(&self) -> Num { self[1] }
}

impl<Num> ComplexConstructor<Num> for Simd<Num, 2>
where
    Num: SimdElement + Axis,
{
    #[inline] fn new_complex(real: Num, imaginary: Num) -> Self {
        Simd::from_array([real, imaginary])
    }
}

impl<Num> ComplexConsts<Num> for Simd<Num, 2>
where
    Num: SimdElement + Axis,
{
    const ORIGIN: Self = Simd::from_array([Num::ZERO, Num::ZERO]);
    const IDENTITY: Self = Simd::from_array([Num::ONE, Num::ZERO]);
    const NAN: Self = Simd::from_array([Num::NAN, Num::NAN]);
    const UNIT_IMAGINARY: Self = Simd::from_array([Num::ZERO, Num::ONE]);
}

impl<Num> Axis for Simd<Num, 1>
where
    Num: SimdElement + Axis,
    Simd<Num, 1>: crate::core::marker::Sized
        + crate::core::ops::Add<Output = Self>
        + crate::core::ops::Sub<Output = Self>
        + crate::core::ops::Mul<Output = Self>
        + crate::core::ops::Div<Output = Self>
        + crate::core::ops::Rem<Output = Self>
        + crate::core::ops::Neg<Output = Self>
        + crate::core::cmp::PartialOrd
        + crate::core::marker::Copy
{
    const ONE: Self = Simd::from_array([Num::ONE]);
    const ZERO: Self = Simd::from_array([Num::ZERO]);
    const TAU: Self = Simd::from_array([Num::TAU]);
    const NAN: Self = Simd::from_array([Num::NAN]);
    const ERROR: Self = Simd::from_array([Num::ERROR]);

    #[inline]
    fn is_nan( &self ) -> bool { self[0].is_nan() }

    #[inline]
    fn mul_add( self, factor: Self, addend: Self ) -> Self { self * factor + addend }

    #[inline]
    fn sqrt( self ) -> Self { Simd::from_array([self[0].sqrt()]) }

    #[inline]
    fn pow( self, exp: Self ) -> Self { Simd::from_array([self[0].pow(exp[0])]) }
    
    #[inline]
    fn sin_cos( self ) -> (Self, Self) {
        let (sin, cos) = self[0].sin_cos();
        (Simd::from_array([sin]), Simd::from_array([cos]))
    }

    #[inline]
    fn sin( self ) -> Self { Simd::from_array([self[0].sin()]) }

    #[inline]
    fn asin( self ) -> Self { Simd::from_array([self[0].asin()]) }

    #[inline]
    fn sinh( self ) -> Self { Simd::from_array([self[0].sinh()]) }

    #[inline]
    fn cos( self ) -> Self { Simd::from_array([self[0].cos()]) }

    #[inline]
    fn acos( self ) -> Self { Simd::from_array([self[0].acos()]) }

    #[inline]
    fn cosh( self ) -> Self { Simd::from_array([self[0].cosh()]) }

    #[inline]
    fn exp( self ) -> Self { Simd::from_array([self[0].exp()]) }

    #[inline]
    fn ln( self ) -> Self { Simd::from_array([self[0].ln()]) }
    
    #[inline]
    fn atan2( self, bottom: Self ) -> Self { Simd::from_array([self[0].atan2(bottom[0])]) }

    #[inline]
    fn from_u8( uint: u8 ) -> Self { Simd::from_array([Num::from_u8(uint)]) }

    #[inline]
    fn from_f64( float: f64 ) -> Self { Simd::from_array([Num::from_f64(float)]) }
}

impl<Num> Scalar<Num> for Simd<Num, 1>
where
    Num: SimdElement + Axis,
{
    #[inline] fn scalar(&self) -> Num { self[0] }
}

impl<Num> ScalarConsts<Num> for Simd<Num, 1>
where
    Num: SimdElement + Axis,
{
    const ZERO: Self = Simd::from_array([Num::ZERO]);
    const ONE: Self = Simd::from_array([Num::ONE]);
    const NAN: Self = Simd::from_array([Num::NAN]);
}
