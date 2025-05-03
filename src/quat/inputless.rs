// This module is for functions that will always return the same thing and that do not take an input

use super::*;

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs the origin quaternion. (Aditive identity)
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::origin;
/// 
/// let quat: [f32; 4] = origin::<f32, [f32; 4]>();
/// 
/// assert_eq!( quat, [0.0, 0.0, 0.0, 0.0] );
/// ```
pub fn origin<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat(())
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs the positive real unit quaternion. (Multiplicative identity)
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::identity;
/// 
/// let quat: [f32; 4] = identity::<f32, [f32; 4]>();
/// 
/// assert_eq!( quat, [1.0, 0.0, 0.0, 0.0] );
/// ```
pub fn identity<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat((Num::ONE, ()))
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion that has all axies set to [`Num::NAN`s](Axis::NAN).
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::nan;
/// 
/// let quat: [f32; 4] = nan::<f32, [f32; 4]>();
/// 
/// assert!( quat[0].is_nan() );
/// assert!( quat[1].is_nan() );
/// assert!( quat[2].is_nan() );
/// assert!( quat[3].is_nan() );
/// ```
pub fn nan<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat([Num::NAN; 4])
}

#[inline(always)]
/// Returns the unit quaternion on the real axis.
pub fn unit_r<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    identity()
}

#[inline]
/// Returns the unit quaternion on the first imaginary axis.
pub fn unit_i<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat([Num::ZERO, Num::ONE, Num::ZERO, Num::ZERO])
}

#[inline]
/// Returns the unit quaternion on the second imaginary axis.
pub fn unit_j<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat([Num::ZERO, Num::ZERO, Num::ONE, Num::ZERO])
}

#[inline]
/// Returns the unit quaternion on the third imaginary axis.
pub fn unit_k<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat([Num::ZERO, Num::ZERO, Num::ZERO, Num::ONE])
}
