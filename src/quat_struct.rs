
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
    Neg,
};

/**
The struct representation of the [`Quaternion`] trait.

Try not to use this quaternion struct if:
- If you don't mind using the a tuple or an array.
- You don't plan on using any operators like `+` or `*` and just functions/traits.
- You already have a quaternion type that implements the [`Quaternion`], [`QuaternionConstructor`], [`QuaternionConsts`] and [`QuaternionMethods`] traits.

Reasoning: This struct exists just as a ease of use if you need
a quaternion struct and do not want to make your own or get one from another crate.
 */
#[derive(Debug, Clone, Copy)]
pub struct Quat<Num: Axis = f32, T = (Num, [Num; 3])> {
    /// The quaternion held by this struct.
    pub quat: T,
    _num: crate::core::marker::PhantomData<Num>,
}

impl<Num: Axis, T> Quat<Num, T> {
    #[inline]
    /// Creates a new instince of this struct.
    pub const fn new(quat: T) -> Self {
        Quat {
            quat, _num: crate::core::marker::PhantomData
        }
    }
}

impl<Num: Axis, T: Quaternion<Num>> Quaternion<Num> for Quat<Num, T> {
    #[inline] fn r(&self) -> Num { self.quat.r() }
    #[inline] fn i(&self) -> Num { self.quat.i() }
    #[inline] fn j(&self) -> Num { self.quat.j() }
    #[inline] fn k(&self) -> Num { self.quat.k() }
}

impl<Num: Axis, T: QuaternionConstructor<Num>> QuaternionConstructor<Num> for Quat<Num, T> {
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> Self { Quat::new(QuaternionConstructor::new_quat(r, i, j, k)) }
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

impl<Num: Axis, T: QuaternionMethods<Num>> QuaternionMethods<Num> for crate::Quat<Num, T> {}

impl<Num: Axis, T: crate::core::default::Default> crate::core::default::Default for Quat<Num, T> {
    #[inline] fn default() -> Self {
        Quat::new(T::default())
    }
}

impl<Num: Axis + crate::core::fmt::Display, T: Quaternion<Num>> crate::core::fmt::Display for Quat<Num, T> {
    #[inline] fn fmt(&self, f: &mut crate::core::fmt::Formatter<'_>) -> crate::core::fmt::Result {
        quat::display(self, f)
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
        quat::neg(&self)
    }
}

impl<Num: Axis, T: QuaternionMethods<Num>> Neg for &Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn neg(self) -> Self::Output {
        quat::neg(&self)
    }
}

impl<Num: Axis, T: QuaternionMethods<Num>> Neg for &mut Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn neg(self) -> Self::Output {
        quat::neg(&self)
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
