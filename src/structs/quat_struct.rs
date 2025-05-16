
use crate::Axis;
use crate::Quaternion;
use crate::QuaternionConsts;
use crate::QuaternionConstructor;
use crate::QuaternionMethods;
use crate::quat;
#[cfg(feature = "std")]
use crate::structs::Std;

use crate::core::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Neg, Not,
};

/**
The struct representation of the [`Quaternion`] trait.

Try not to use this quaternion struct if:
- If you don't mind using the a tuple or an array.
- You don't plan on using any operators like `+` or `*` and just functions/traits.
- You already have a quaternion type that implements the
[`Quaternion`], [`QuaternionConstructor`], [`QuaternionConsts`]
and [`QuaternionMethods`] traits.

Reasoning: This struct exists just as a ease of use if you need
a quaternion struct and do not want to make your own or get one from another crate.

If the `std` feature is enabled the default is set to `Std<f32>`, otherwise it's set to `f32`
 */
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
#[cfg(not(feature = "std"))]
pub struct Quat<Num: Axis = f32, T = (Num, [Num; 3])> {
    /// The quaternion held by this struct.
    pub quat: T,
    _num: crate::core::marker::PhantomData<Num>,
}
/**
The struct representation of the [`Quaternion`] trait.

Try not to use this quaternion struct if:
- If you don't mind using the a tuple or an array.
- You don't plan on using any operators like `+` or `*` and just functions/traits.
- You already have a quaternion type that implements the
[`Quaternion`], [`QuaternionConstructor`], [`QuaternionConsts`]
and [`QuaternionMethods`] traits.

Reasoning: This struct exists just as a ease of use if you need
a quaternion struct and do not want to make your own or get one from another crate.

If the `std` feature is enabled the default is set to `Std<f32>`, otherwise it's set to `f32`
 */
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
#[cfg(feature = "std")]
pub struct Quat<Num: Axis = crate::structs::Std<f32>, T = (Num, [Num; 3])> {
    /// The quaternion held by this struct.
    pub quat: T,
    _num: crate::core::marker::PhantomData<Num>,
}

/// Type alias for [`Quat`] or [`Quat<f32>`].
/// 
/// If the `std` feature is enabled thn this uses [`Std<f32>`](crate::structs::Std) instead of [`f32`].
#[cfg(not(feature = "std"))]
pub type Quat32<T = (f32, [f32; 3])> = Quat<f32, T>;
/// Type alias for [`Quat`] or [`Quat<f32>`].
/// 
/// If the `std` feature is enabled thn this uses [`Std<f32>`](crate::structs::Std) instead of [`f32`].
#[cfg(feature = "std")]
pub type Quat32<T = (Std<f32>, [Std<f32>; 3])> = Quat<Std<f32>, T>;
/// Type alias for [`Quat<f64>`].
/// 
/// If the `std` feature is enabled thn this uses [`Std<f64>`](crate::structs::Std) instead of [`f64`].
#[cfg(not(feature = "std"))]
pub type Quat64<T = (f64, [f64; 3])> = Quat<f64, T>;
/// Type alias for [`Quat<f64>`].
/// 
/// If the `std` feature is enabled thn this uses [`Std<f64>`](crate::structs::Std) instead of [`f64`].
#[cfg(feature = "std")]
pub type Quat64<T = (Std<f64>, [Std<f64>; 3])> = Quat<Std<f64>, T>;

impl<Num: Axis, T> Quat<Num, T> {
    #[inline]
    /// Creates a new instince of this struct.
    pub const fn new(quat: T) -> Self {
        Quat {
            quat, _num: crate::core::marker::PhantomData
        }
    }

    #[inline]
    /// Gets the wrapped value.
    pub fn get(self) -> T {
        self.quat
    }

    #[inline]
    /// Gets a refrence to the wrapped value.
    pub fn get_ref(&self) -> &T {
        &self.quat
    }

    #[inline]
    /// Gets a mutable refrence to the wrapped value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.quat
    }

    #[inline]
    /// Gets a cloned version of the wrapped value.
    pub fn get_cloned(&self) -> T
    where T: crate::core::clone::Clone
    {
        self.quat.clone()
    }
}

impl<Num: Axis, T: Quaternion<Num>> Quaternion<Num> for Quat<Num, T> {
    #[inline(always)] fn r(&self) -> Num { self.quat.r() }
    #[inline(always)] fn i(&self) -> Num { self.quat.i() }
    #[inline(always)] fn j(&self) -> Num { self.quat.j() }
    #[inline(always)] fn k(&self) -> Num { self.quat.k() }
}

impl<Num: Axis, T: QuaternionConstructor<Num>> QuaternionConstructor<Num> for Quat<Num, T> {
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> Self { Quat::new(QuaternionConstructor::new_quat(r, i, j, k)) }
    #[inline] fn from_quat(quat: impl Quaternion<Num>) -> Self { Quat::new(T::from_quat(quat)) }
}

impl<Num: Axis, T: QuaternionConsts<Num>> QuaternionConsts<Num> for Quat<Num, T> {
    const ORIGIN: Self = Quat::new(T::ORIGIN);
    const IDENTITY: Self = Quat::new(T::IDENTITY);
    const NAN: Self = Quat::new(T::NAN);

    const UNIT_R: Self = Quat::new(T::UNIT_R);
    const UNIT_I: Self = Quat::new(T::UNIT_I);
    const UNIT_J: Self = Quat::new(T::UNIT_J);
    const UNIT_K: Self = Quat::new(T::UNIT_K);
}

impl<Num: Axis, T> crate::core::ops::Deref for Quat<Num, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.quat
    }
}

impl<Num: Axis, T: crate::core::default::Default> crate::core::default::Default for Quat<Num, T> {
    #[inline] fn default() -> Self {
        Quat::new(T::default())
    }
}

#[cfg(feature = "display")] 
impl<Num: Axis + crate::core::fmt::Display, T: Quaternion<Num>> crate::core::fmt::Display for Quat<Num, T> {
    #[inline] fn fmt(&self, f: &mut crate::core::fmt::Formatter<'_>) -> crate::core::fmt::Result {
        quat::display(f, self, crate::structs::QuaternionFormat::DEFAULT)
    }
}

impl<Num: Axis, T: Quaternion<Num>, Other: Quaternion<Num>> crate::core::cmp::PartialEq<Other> for Quat<Num, T> {
    #[inline] fn eq(&self, other: &Other) -> bool {
        quat::eq(self, other)
    }
}
impl<Num: Axis + crate::core::cmp::Eq, T: Quaternion<Num> + crate::core::cmp::Eq> crate::core::cmp::Eq for Quat<Num, T> { }

impl<Num: Axis, T: QuaternionConstructor<Num>, Q: Quaternion<Num>> crate::core::iter::Sum<Q> for Quat<Num, T> {
    fn sum<I: crate::core::iter::Iterator<Item = Q>>(iter: I) -> Self {
        quat::sum(iter)
    }
}

impl<Num: Axis, T: QuaternionConstructor<Num>, Q: Quaternion<Num>> crate::core::iter::Product<Q> for Quat<Num, T> {
    fn product<I: crate::core::iter::Iterator<Item = Q>>(iter: I) -> Self {
        quat::product(iter)
    }
}

impl<Num: Axis, T: QuaternionMethods<Num>> Neg for Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn neg(self) -> Self::Output {
        quat::neg(self)
    }
}

impl<Num: Axis, T: QuaternionMethods<Num>> Neg for &Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn neg(self) -> Self::Output {
        quat::neg(self)
    }
}

impl<Num: Axis, T: QuaternionMethods<Num>> Neg for &mut Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn neg(self) -> Self::Output {
        quat::neg(self)
    }
}

/// Calculates the conjugate of the quat using `!`.
impl<Num: Axis, T: QuaternionMethods<Num>> Not for Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn not(self) -> Self::Output {
        quat::conj(self)
    }
}

/// Calculates the conjugate of the quat using `!`.
impl<Num: Axis, T: QuaternionMethods<Num>> Not for &Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn not(self) -> Self::Output {
        quat::conj(self)
    }
}

/// Calculates the conjugate of the quat using `!`.
impl<Num: Axis, T: QuaternionMethods<Num>> Not for &mut Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn not(self) -> Self::Output {
        quat::conj(self)
    }
}

macro_rules! impl_basic_ops_for_quat {
    (
        impl = $trait:ident;
        func = $trait_func:ident;
        assign = $assign:ident;
        assign_func = $assign_func:ident;
        using = $func:ident $(;)?
    ) => {
        impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> $trait<Other> for Quat<Num, T> {
            type Output = Quat<Num, T>;
            #[inline] fn $trait_func(self, other: Other) -> Quat<Num, T> {
                quat::$func(&self, &other)
            }
        }

        impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> $trait<Other> for &Quat<Num, T> {
            type Output = Quat<Num, T>;
            #[inline] fn $trait_func(self, other: Other) -> Quat<Num, T> {
                quat::$func(&self, &other)
            }
        }

        impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> $trait<Other> for &mut Quat<Num, T> {
            type Output = Quat<Num, T>;
            #[inline] fn $trait_func(self, other: Other) -> Quat<Num, T> {
                quat::$func(&self, &other)
            }
        }

        impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> $assign<Other> for Quat<Num, T> {
            #[inline] fn $assign_func(&mut self, other: Other) {
                *self = quat::$func(&self, &other);
            }
        }
    };
}

impl_basic_ops_for_quat!{
    impl = Add;
    func = add;
    assign = AddAssign;
    assign_func = add_assign;
    using = add;
}

impl_basic_ops_for_quat!{
    impl = Sub;
    func = sub;
    assign = SubAssign;
    assign_func = sub_assign;
    using = sub;
}

impl_basic_ops_for_quat!{
    impl = Mul;
    func = mul;
    assign = MulAssign;
    assign_func = mul_assign;
    using = mul;
}

impl_basic_ops_for_quat!{
    impl = Div;
    func = div;
    assign = DivAssign;
    assign_func = div_assign;
    using = div;
}

#[cfg(feature = "display")] 
impl<Num: Axis + crate::core::str::FromStr, T: QuaternionConstructor<Num>> crate::core::str::FromStr for Quat<Num, T> {
    type Err = crate::structs::QuaternionParseError<Num>;

    fn from_str(s: &str) -> crate::core::result::Result<Self, Self::Err> {
        quat::from_str(s)
    }
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
use crate::num_traits::pow::Pow;

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> Pow<Other> for Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn pow(self, other: Other) -> Quat<Num, T> {
        quat::pow_q(&self, &other)
    }
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> Pow<Other> for &Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn pow(self, other: Other) -> Quat<Num, T> {
        quat::pow_q(&self, &other)
    }
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> Pow<Other> for &mut Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn pow(self, other: Other) -> Quat<Num, T> {
        quat::pow_q(&self, &other)
    }
}

/// Constructs a [`Quat`] for any `Num` that implements axis and of `T = (Num, [Num; 3]`).
pub const fn q<Num: Axis>(r: Num, i: Num, j: Num, k: Num) -> Quat<Num> {
    Quat::new((r, [i, j, k]))
}

trait Take<To> {
    fn take(self) -> To;
}
impl Take<f32> for f32 { fn take(self) -> f32 { self } }
#[cfg(feature = "std")] impl Take<Std<f32>> for f32 { fn take(self) -> Std<f32> { Std(self) } }
impl Take<f64> for f64 { fn take(self) -> f64 { self } }
#[cfg(feature = "std")] impl Take<Std<f64>> for f64 { fn take(self) -> Std<f64> { Std(self) } }

/// Constructs a [`Quat`] with `Num = f32` and `T = (f32, [f32; 3])`.
/// 
/// Or uses [`Std<f32>`] instead of [`f32`] if the feature `std` is enabled.
pub fn q32<Num: crate::core::convert::Into<f32>>(r: Num, i: Num, j: Num, k: Num) -> Quat32 {
    Quat::new((r.into().take(), [i.into().take(), j.into().take(), k.into().take()]))
}

/// Constructs a [`Quat`] with `Num = f64` and `T = (f64, [f64; 3])`.
/// 
/// Or uses [`Std<f64>`] instead of [`f64`] if the feature `std` is enabled.
pub fn q64<Num: crate::core::convert::Into<f64>>(r: Num, i: Num, j: Num, k: Num) -> Quat64 {
    Quat::new((r.into().take(), [i.into().take(), j.into().take(), k.into().take()]))
}

mod quat_struct_methods_impl {
    #[allow(unused_imports)]
    use crate::core::option::Option;
    use super::Quat;
    use crate::*;

    macro_rules! impl_method_on_quaternion_methods {
        (
            $(
                $( #[$($attrib:meta),+] )?
                fn $func:ident // function name
                $( < $( $generic:ident $( : $trait:ty $( | $extra:ty )* )? ),+ > )? // generics
                ( self $ ( , $( $param:ident : $type:ty ),* $(,)? )? ) // parameters
                -> Self // return type
                $( where $( $where_generic:ident : $where_trait:ty $( | $where_extra:ty)* ),+ )? // where clause
            );+
            $(;)?
        ) => {
            $(
                #[inline]
                $( $(#[$attrib])+ )?
                fn $func $(<$($generic$(: $trait $(+ $extra)*)?)+>)?
                ( self, $( $($param: $type),* )? )
                -> Self
                $( where $( $where_generic : $where_trait $(+ $where_extra)* )+ )?
                { Quat::new( <T as QuaternionMethods<Num>>::$func( self.quat, $( $($param, )* )? ) ) }
            )+
        };

        (
            $(
                $( #[$($attrib:meta),+] )?
                fn $func:ident // function name
                $( < $( $generic:ident $( : $trait:ident $( < $( $trait_extra:ident ),+ > )? $( | $extra:ident )* )? ),+ > )? // generics
                ( self $ ( , $( $param:ident : $type:ty ),* $(,)? )? ) // parameters
                -> $return:ty // return type
                $( where $( $where_generic:ident : $where_trait:ty $( | $where_extra:ty)* ),+ )? // where clause
            );+
            $(;)?
        ) => {
            $(
                #[inline]
                $( $(#[$attrib])+ )?
                fn $func $(<$($generic$(: $trait$(<$( $trait_extra, )+ > )? $(+ $extra)*)?)+>)?
                ( self, $( $($param: $type),* )? )
                -> $return
                $( where $( $where_generic : $where_trait $(+ $where_extra)* )+ )?
                { <T as QuaternionMethods<Num>>::$func( self.quat, $( $($param, )* )? ) }
            )+
        };

        (
            $(
                $( #[$($attrib:meta),+] )?
                fn $func:ident // function name
                $( < $( $generic:ident $( : $trait:ident $( | $extra:ident )* )? ),+ > )? // generics
                ( $( $param:ident : $type:ty ),* $(,)? ) // parameters
                -> Self // return type
                $( where $( $where_generic:ident : $where_trait:ty $( | $where_extra:ty)* ),+ )? // where clause
            );+
            $(;)?
        ) => {
            $(
                #[inline]
                $( $(#[$attrib])+ )?
                fn $func $(<$($generic$(: $trait $(+ $extra)*)?)+>)?
                ( $($param: $type),* )
                -> Self
                $( where $( $where_generic : $where_trait $(+ $where_extra)* )+ )?
                { Quat::new( <T as QuaternionMethods<Num>>::$func( $($param, )* ) ) }
            )+
        };

        (
            $(
                fn $func:ident // function name
                $( < $( $generic:ident $( : $trait:ident $( | $extra:ident )* )? ),+ > )? // generics
                ( $( $param:ident : $type:ty ),* $(,)? ) // parameters
                -> Option<Self> // return type
                $( where $( $where_generic:ident : $where_trait:ty $( | $where_extra:ty)* ),+ )? // where clause
            );+
            $(;)?
        ) => {
            $(
                #[inline]
                fn $func $(<$($generic$(: $trait $(+ $extra)*)?)+>)?
                ( $($param: $type),* )
                -> Option<Self>
                $( where $( $where_generic : $where_trait $(+ $where_extra)* )+ )?
                { Option::Some(Quat::new( <T as QuaternionMethods<Num>>::$func( $($param, )* )? )) }
            )+
        };
    }

    impl<Num: Axis, T: QuaternionMethods<Num>> QuaternionMethods<Num> for Quat<Num, T> {
        // isntead of `+` use `|`
        impl_method_on_quaternion_methods!{
            fn add(self, other: impl Quaternion<Num>) -> Self;
            fn sub(self, other: impl Quaternion<Num>) -> Self;
            fn mul(self, other: impl Quaternion<Num>) -> Self;
            fn mul_reversed(self, other: impl Quaternion<Num>) -> Self;
            fn div(self, other: impl Quaternion<Num>) -> Self;
            fn div_reversed(self, other: impl Quaternion<Num>) -> Self;

            fn neg(self) -> Self;
            fn conj(self) -> Self;
            fn inv(self) -> Self;
            fn norm(self) -> Self;
            #[cfg(feature = "math_fns")] fn sqrt(self) -> Self;
            #[cfg(any(feature = "math_fns", feature = "qol_fns"))] fn square(self) -> Self;
            #[cfg(feature = "math_fns")] fn exp(self) -> Self;
            #[cfg(feature = "math_fns")] fn ln(self) -> Self;
            #[cfg(feature = "unstable")] fn log(self, base: impl Quaternion<Num>) -> Self;
            #[cfg(feature = "trigonometry")] fn sin(self) -> Self;
            #[cfg(feature = "trigonometry")] fn sinh(self) -> Self;
            #[cfg(feature = "trigonometry")] fn sec(self) -> Self;
            #[cfg(feature = "trigonometry")] fn cos(self) -> Self;
            #[cfg(feature = "trigonometry")] fn cosh(self) -> Self;
            #[cfg(feature = "trigonometry")] fn csc(self) -> Self;
            #[cfg(feature = "trigonometry")] fn tan(self) -> Self;
            #[cfg(feature = "trigonometry")] fn tanh(self) -> Self;
            #[cfg(feature = "trigonometry")] fn cot(self) -> Self;
            #[cfg(feature = "trigonometry")] fn coth(self) -> Self;
            #[cfg(feature = "trigonometry")] fn asin(self) -> Self;
            #[cfg(feature = "trigonometry")] fn asinh(self) -> Self;
            #[cfg(feature = "trigonometry")] fn asec(self) -> Self;
            #[cfg(feature = "trigonometry")] fn acos(self) -> Self;
            #[cfg(feature = "trigonometry")] fn acosh(self) -> Self;
            #[cfg(feature = "trigonometry")] fn acsc(self) -> Self;
            #[cfg(feature = "trigonometry")] fn atan(self) -> Self;
            #[cfg(feature = "trigonometry")] fn atanh(self) -> Self;
            #[cfg(feature = "trigonometry")] fn acot(self) -> Self;
            #[cfg(feature = "trigonometry")] fn acoth(self) -> Self;

            fn scale(self, scalar: impl Scalar<Num>) -> Self;
            fn unscale(self, scalar: impl Scalar<Num>) -> Self;

            #[cfg(feature = "math_fns")] fn pow_i(self, exp: i32) -> Self;
            #[cfg(feature = "math_fns")] fn pow_u(self, exp: u32) -> Self;
            #[cfg(feature = "math_fns")] fn pow_f(self, exp: impl Scalar<Num>) -> Self;
            #[cfg(feature = "unstable"), cfg(feature = "math_fns")]
            fn pow_q(self, exp: impl Quaternion<Num>) -> Self;

            fn vector_part(self) -> Self;
            fn complex_part(self) -> Self;
            fn scalar_part(self) -> Self;
        }

        impl_method_on_quaternion_methods!{
            fn eq(self, other: impl Quaternion<Num>) -> bool;

            fn abs(self) -> Num;
            fn abs_squared(self) -> Num;
            fn angle(self) -> Num;
            fn angle_cos(self) -> Num;
            fn dot(self, other: impl Quaternion<Num>) -> Num;

            fn is_scalar(self) -> bool;
            fn is_vector(self) -> bool;
            fn is_complex(self) -> bool;
            #[cfg(feature = "qol_fns")] 
            fn is_on_axis_plane(self) -> bool;

            fn is_near(self, other: impl Quaternion<Num>) -> bool;
            fn is_near_by(self, other: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool;
            fn is_close(self, other: impl Quaternion<Num>) -> bool;
            fn is_close_by(self, other: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool;
            
            fn dist_euclid(self, other: impl Quaternion<Num>) -> Num;
            fn dist_cosine(self, other: impl Quaternion<Num>) -> Num;

            fn to_vector<V: VectorConstructor<Num> >(self) -> V;
            fn to_complex<C: ComplexConstructor<Num> >(self) -> C;
            fn to_scalar<S: ScalarConstructor<Num> >(self) -> S;
            #[cfg(feature = "rotation")]
            fn to_rotation<R: RotationConstructor<Num> >(self) -> R;
        }

        #[inline]
        #[cfg(feature = "math_fns")]
        fn to_polar_form<Abs, Angle, UnitVec>(self) -> (Abs, Angle, UnitVec)
            where 
                Abs: ScalarConstructor<Num>,
                Angle: ScalarConstructor<Num>,
                UnitVec: VectorConstructor<Num>,
        { T::to_polar_form(self.quat) }

        impl_method_on_quaternion_methods!{
            fn from_vector(vector: impl Vector<Num>) -> Self;
            fn from_complex(complex: impl Complex<Num>) -> Self;
            fn from_scalar(scalar: impl Scalar<Num>) -> Self;
            #[cfg(feature = "rotation")]
            fn from_rotation(rotation: impl Rotation<Num>) -> Self;
        }

        #[cfg(feature = "math_fns")]
        impl_method_on_quaternion_methods!{
            fn from_polar_form(
                abs: impl Scalar<Num>,
                angle: impl Scalar<Num>,
                unit_vec: impl Vector<Num>,
            ) -> Option<Self>;
        }

        #[cfg(feature = "matrix")] #[inline] fn to_matrix_2<C: ComplexConstructor<Num>, M: MatrixConstructor<C, 2>>(self) -> M {T::to_matrix_2(self.quat)}
        #[cfg(feature = "matrix")] #[inline] fn to_matrix_3<S: ScalarConstructor<Num>, M: MatrixConstructor<Num, 3>>(self) -> M {T::to_matrix_3::<S, M>(self.quat)}
        #[cfg(feature = "matrix")] #[inline] fn to_matrix_4<S: ScalarConstructor<Num>, M: MatrixConstructor<Num, 4>>(self) -> M {T::to_matrix_4::<S, M>(self.quat)}

        #[cfg(feature = "matrix")] #[inline] fn from_matrix_2<Elem: Complex<Num>>(matrix: impl Matrix<Elem, 2>) -> Option<Self> {Option::Some(Quat::new(T::from_matrix_2(matrix)?))}
        #[cfg(feature = "matrix")] #[inline] fn from_matrix_3<Elem: Scalar<Num>>(matrix: impl Matrix<Elem, 3>) -> Self {Quat::new(T::from_matrix_3::<Elem>(matrix))}
        #[cfg(feature = "matrix")] #[inline] fn from_matrix_4<Elem: Scalar<Num>>(matrix: impl Matrix<Elem, 4>) -> Self {Quat::new(T::from_matrix_4::<Elem>(matrix))}
    }
}
