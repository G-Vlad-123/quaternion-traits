
use crate::Axis;
use crate::Quaternion;
use crate::QuaternionConsts;
use crate::QuaternionConstructor;
use crate::QuaternionMethods;
use crate::quat;

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
#[cfg(not(feature = "std"))]
pub struct Quat<Num: Axis = f32, T = (Num, [Num; 3])> {
    /// The quaternion held by this struct.
    pub quat: T,
    _num: crate::core::marker::PhantomData<Num>,
}
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
#[cfg(feature = "std")]
pub type Quat32<T = (Std<f32>, [Std<f32>; 3])> = Quat<Std<f32>, T>;
/// Type alias for [`Quat<f64>`].
/// 
/// If the `std` feature is enabled thn this uses [`Std<f64>`](crate::structs::Std) instead of [`f64`].
#[cfg(not(feature = "std"))]
pub type Quat64<T = (f64, [f64; 3])> = Quat<f64, T>;
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
    /// Gets a cloned version of the wrapped value.
    pub fn get_cloned(&self) -> T
    where T: crate::core::clone::Clone
    {
        <Self as crate::core::clone::Clone>::clone(self).quat
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

/// Constructs a [`Quat`] with `Num = f32` and `T = (f32, [f32; 3])`.
pub fn q32<Num: crate::core::convert::Into<f32>>(r: Num, i: Num, j: Num, k: Num) -> Quat<f32> {
    Quat::new((r.into(), [i.into(), j.into(), k.into()]))
}

/// Constructs a [`Quat`] with `Num = f64` and `T = (f64, [f64; 3])`.
pub fn q64<Num: crate::core::convert::Into<f64>>(r: Num, i: Num, j: Num, k: Num) -> Quat<f64> {
    Quat::new((r.into(), [i.into(), j.into(), k.into()]))
}

impl<Num: Axis, T: QuaternionMethods<Num>> QuaternionMethods<Num> for Quat<Num, T> { }
