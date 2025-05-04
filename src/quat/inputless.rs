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
/// let quat: [f32; 4] = origin::<f32, _>();
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
/// let quat: [f32; 4] = identity::<f32, _>();
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
/// let quat: [f32; 4] = nan::<f32, _>();
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
/// Returns a unit quaternion on the real axis.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::unit_r;
/// 
/// let quat: [f32; 4] = unit_r::<f32, _>();
/// 
/// assert_eq!( quat, [1.0, 0.0, 0.0, 0.0] );
/// ```
pub fn unit_r<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    identity()
}

#[inline]
/// Returns a unit quaternion on the first imaginary axis.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::unit_i;
/// 
/// let quat: [f32; 4] = unit_i::<f32, _>();
/// 
/// assert_eq!( quat, [0.0, 1.0, 0.0, 0.0] );
/// ```
pub fn unit_i<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat([Num::ZERO, Num::ONE, Num::ZERO, Num::ZERO])
}

#[inline]
/// Returns a unit quaternion on the second imaginary axis.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::unit_j;
/// 
/// let quat: [f32; 4] = unit_j::<f32, _>();
/// 
/// assert_eq!( quat, [0.0, 0.0, 1.0, 0.0] );
/// ```
pub fn unit_j<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat([Num::ZERO, Num::ZERO, Num::ONE, Num::ZERO])
}

#[inline]
/// Returns a unit quaternion on the third imaginary axis.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::unit_k;
/// 
/// let quat: [f32; 4] = unit_k::<f32, _>();
/// 
/// assert_eq!( quat, [0.0, 0.0, 0.0, 1.0] );
/// ```
pub fn unit_k<Num, Out>() -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::from_quat([Num::ZERO, Num::ZERO, Num::ZERO, Num::ONE])
}
