
use crate::Axis;
use crate::Quaternion;
use crate::QuaternionConsts;
use crate::QuaternionConstructor;
use crate::QuaternionMethods;
use crate::quat;
#[cfg(feature = "std")]
use crate::structs::Std;
#[allow(unused_imports)]
use crate::core::option::Option;

#[allow(unused_imports)]
use crate::core::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Rem, RemAssign,
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

#[cfg(feature = "unstable")]
impl<Num: Axis + Rem<Num, Output = Num>, T: Quaternion<Num> + QuaternionConstructor<Num>> Rem for Quat<Num, T> {
    type Output = Self;

    fn rem(self, modulus: Self) -> Self {
        quat::rem(self, modulus)
    }
}

#[cfg(feature = "unstable")]
impl<Num: Axis + Rem<Num, Output = Num>, T: Quaternion<Num> + QuaternionConstructor<Num>> RemAssign for Quat<Num, T> {
    fn rem_assign(&mut self, modulus: Self) {
        *self = quat::rem(&*self, modulus)
    }
}

#[cfg(feature = "display")] 
impl<Num: Axis + crate::core::str::FromStr, T: QuaternionConstructor<Num>> crate::core::str::FromStr for Quat<Num, T> {
    type Err = Num::Err;

    fn from_str(s: &str) -> crate::core::result::Result<Self, Self::Err> {
        quat::from_str(s)
    }
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
#[cfg(feature = "math_fns")]
use crate::num_traits::pow::Pow;

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
#[cfg(feature = "math_fns")]
impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> Pow<Other> for Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn pow(self, other: Other) -> Quat<Num, T> {
        quat::pow_q(&self, &other)
    }
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
#[cfg(feature = "math_fns")]
impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> Pow<Other> for &Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn pow(self, other: Other) -> Quat<Num, T> {
        quat::pow_q(&self, &other)
    }
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
#[cfg(feature = "math_fns")]
impl<Num: Axis, T: QuaternionMethods<Num>, Other: Quaternion<Num>> Pow<Other> for &mut Quat<Num, T> {
    type Output = Quat<Num, T>;
    #[inline] fn pow(self, other: Other) -> Quat<Num, T> {
        quat::pow_q(&self, &other)
    }
}


#[cfg(feature = "num-traits")]
use crate::num_traits::{
    Num as Number,
    One, ConstOne,
    Zero, ConstZero,
};

#[cfg(feature = "num-traits")]
impl<Num: Axis, T: QuaternionMethods<Num>> One for Quat<Num, T>
where Quat<Num, T>: Add<Self, Output = Self>
{
    fn one() -> Self {
        quat::unit_r()
    }

    fn is_one(&self) -> bool {
        quat::eq(&self.quat, quat::identity::<Num, T>())
    }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis, T: QuaternionMethods<Num>> Zero for Quat<Num, T> {
    fn zero() -> Self {
        quat::origin()
    }

    fn is_zero(&self) -> bool {
        quat::eq(&self.quat, ())
    }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis, T: QuaternionConsts<Num> + QuaternionMethods<Num>> ConstZero for Quat<Num, T> {
    const ZERO: Self = Self::ORIGIN;
}

#[cfg(feature = "num-traits")]
impl<Num: Axis, T: QuaternionConsts<Num> + QuaternionMethods<Num>> ConstOne for Quat<Num, T> {
    const ONE: Self = Self::IDENTITY;
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
impl<Num: Axis + Number, T: QuaternionMethods<Num>> Number for Quat<Num, T> {
    type FromStrRadixErr = <Num as Number>::FromStrRadixErr;

    fn from_str_radix(
        s: &str,
        radix: u32,
    ) -> crate::core::result::Result<Self, Self::FromStrRadixErr> {
        use crate::core::result::Result;
        use crate::core::option::Option::{*, self};
    
        let mut quat: [Num; 4] = [Num::ZERO; 4];
        let mut sign: Num = Num::ONE;
        let mut num: Option<(usize, usize)> = None;

        #[inline] fn read<Num: Number>(s: &str, radix: u32) -> Result<Num, <Num as Number>::FromStrRadixErr> {
            Num::from_str_radix(s, radix)
        }

        for (index, c) in s.char_indices() {
            match c {
                ' ' | '\t' | '\n' | '-' | '+' => {
                    if let Some(n) = num {
                        quat[0] = quat[0] + sign * read(&s[n.0..=(n.0 + n.1)], radix)?;
                        num = None;
                        sign = Num::ONE;
                    }
                    if c == '-' {sign = -sign}
                },
                'r' | 'R' => {
                    if let Some(n) = num {
                        quat[0] = quat[0] + sign * read(&s[n.0..=(n.0 + n.1)], radix)?;
                        num = None;
                        sign = Num::ONE;
                    } else {
                        quat[0] = quat[0] + Num::ONE;
                    }
                },
                'i' | 'I' => {
                    if let Some(n) = num {
                        quat[1] = quat[1] + sign * read(&s[n.0..=(n.0 + n.1)], radix)?;
                        num = None;
                        sign = Num::ONE;
                    } else {
                        quat[1] = quat[1] + Num::ONE;
                    }
                },
                'j' | 'J' => {
                    if let Some(n) = num {
                        quat[2] = quat[2] + sign * read(&s[n.0..=(n.0 + n.1)], radix)?;
                        num = None;
                        sign = Num::ONE;
                    } else {
                        quat[2] = quat[2] + Num::ONE;
                    }
                },
                'k' | 'K' => {
                    if let Some(n) = num {
                        quat[3] = quat[3] + sign * read(&s[n.0..=(n.0 + n.1)], radix)?;
                        num = None;
                        sign = Num::ONE;
                    } else {
                        quat[3] = quat[3] + Num::ONE;
                    }
                },
                _ => match num {
                    Some((_, ref mut len)) => *len = *len + 1,
                    None => num = Some((index, 0))
                },
            }
        }

        if let Some(n) = num {
            quat[0] = quat[0] + sign * read(&s[n.0..], radix)?;
        }

        Result::Ok(Self::from_quat(quat))
    }
}

#[cfg(feature = "num-traits")]
#[cfg(feature = "unstable")]
impl<Num: Axis + 'static, T: Quaternion<Num> + crate::core::marker::Copy + 'static> crate::num_traits::AsPrimitive<Num> for Quat<Num, T> {
    #[inline] fn as_(self) -> Num {
        self.r() as Num
    }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis + crate::num_traits::ToPrimitive, T: Quaternion<Num>> crate::num_traits::ToPrimitive for Quat<Num, T> {
    #[inline] fn to_i8(&self) -> Option<i8> { self.r().to_i8() }
    #[inline] fn to_u8(&self) -> Option<u8> { self.r().to_u8() }
    
    #[inline] fn to_i16(&self) -> Option<i16> { self.r().to_i16() }
    #[inline] fn to_u16(&self) -> Option<u16> { self.r().to_u16() }
    
    #[inline] fn to_i32(&self) -> Option<i32> { self.r().to_i32() }
    #[inline] fn to_u32(&self) -> Option<u32> { self.r().to_u32() }
    #[inline] fn to_f32(&self) -> Option<f32> { self.r().to_f32() }

    #[inline] fn to_i64(&self) -> Option<i64> { self.r().to_i64() }
    #[inline] fn to_u64(&self) -> Option<u64> { self.r().to_u64() }
    #[inline] fn to_f64(&self) -> Option<f64> { self.r().to_f64() }
    
    #[inline] fn to_i128(&self) -> Option<i128> { self.r().to_i128() }
    #[inline] fn to_u128(&self) -> Option<u128> { self.r().to_u128() }

    #[inline] fn to_isize(&self) -> Option<isize> { self.r().to_isize() }
    #[inline] fn to_usize(&self) -> Option<usize> { self.r().to_usize() }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis + crate::num_traits::FromPrimitive + Number, T: QuaternionConstructor<Num>> crate::num_traits::FromPrimitive for Quat<Num, T> {
    #[inline] fn from_i8(primitive: i8) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_i8(primitive)?)) }
    #[inline] fn from_u8(primitive: u8) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_u8(primitive)?)) }
    
    #[inline] fn from_i16(primitive: i16) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_i16(primitive)?)) }
    #[inline] fn from_u16(primitive: u16) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_u16(primitive)?)) }
    
    #[inline] fn from_i32(primitive: i32) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_i32(primitive)?)) }
    #[inline] fn from_u32(primitive: u32) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_u32(primitive)?)) }
    #[inline] fn from_f32(primitive: f32) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_f32(primitive)?)) }

    #[inline] fn from_i64(primitive: i64) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_i64(primitive)?)) }
    #[inline] fn from_u64(primitive: u64) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_u64(primitive)?)) }
    #[inline] fn from_f64(primitive: f64) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_f64(primitive)?)) }
    
    #[inline] fn from_i128(primitive: i128) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_i128(primitive)?)) }
    #[inline] fn from_u128(primitive: u128) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_u128(primitive)?)) }

    #[inline] fn from_isize(primitive: isize) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_isize(primitive)?)) }
    #[inline] fn from_usize(primitive: usize) -> Option<Self> { Option::Some(quat::from_scalar(<Num as crate::num_traits::FromPrimitive>::from_usize(primitive)?)) }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis + crate::num_traits::NumCast + crate::num_traits::Num, T: QuaternionConstructor<Num> + Quaternion<Num>> crate::num_traits::NumCast for Quat<Num, T> {
    #[inline]
    fn from<O>(origin: O) -> Option<Self>
    where O: crate::num_traits::ToPrimitive
    {
        Option::Some(
            quat::from_scalar(Num::from(origin)?)
        )
    }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis + crate::num_traits::Bounded, T: QuaternionConstructor<Num>> crate::num_traits::Bounded for Quat<Num, T> {
    #[inline] fn min_value() -> Self { Quat::new_quat(Num::min_value(), Num::min_value(), Num::min_value(), Num::min_value())}
    #[inline] fn max_value() -> Self { Quat::new_quat(Num::max_value(), Num::max_value(), Num::max_value(), Num::max_value())}
}

#[cfg(feature = "num-traits")]
impl<Num: Axis, T: QuaternionMethods<Num>> crate::num_traits::Inv for Quat<Num, T> {
    type Output = Self;

    fn inv(self) -> Self {
        quat::inv(self)
    }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis, T: QuaternionMethods<Num>> crate::num_traits::Inv for &Quat<Num, T> {
    type Output = Quat<Num, T>;

    fn inv(self) -> Quat<Num, T> {
        quat::inv(self)
    }
}

#[cfg(feature = "num-traits")]
impl<Num: Axis, T: QuaternionMethods<Num>> crate::num_traits::Inv for &mut Quat<Num, T> {
    type Output = Quat<Num, T>;

    fn inv(self) -> Quat<Num, T> {
        quat::inv(self)
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
            #[cfg(feature = "unstable"), cfg(feature = "math_fns")] fn log(self, base: impl Quaternion<Num>) -> Self;
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
