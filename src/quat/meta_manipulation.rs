// for functions that manipulate the quaternion as an arbritary struct/enum/union/object.

use super::*;

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a new general quaternion representation.
/// 
/// ```
/// use quaternion_traits::quat::new_quat;
/// 
/// let q: [f32; 4] = new_quat(1.0, 2.0, 3.0, 4.0);
/// assert_eq!( q, [1.0, 2.0, 3.0, 4.0] );
/// 
/// let p = new_quat::<f64, (u8, [u8; 3])>(1.0, 2.0, 3.0, 4.0);
/// assert_eq!( p, (1, [2, 3, 4]) ); 
/// ```
pub fn new_quat<Num, Out>(r: Num, i: Num, j: Num, k: Num) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(r, i, j, k)
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the vector part of a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::vector_part;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// 
/// assert_eq!(
///     vector_part::<f32, [f32; 4]>(&quat),
///     [0.0, 3.4, 5.6, 7.8]
/// )
/// ```
pub fn vector_part<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        Num::ZERO,
        quaternion.i(),
        quaternion.j(),
        quaternion.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the complex part of a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::complex_part;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// 
/// assert_eq!(
///     complex_part::<f32, [f32; 4]>(&quat),
///     [1.2, 3.4, 0.0, 0.0]
/// )
/// ```
pub fn complex_part<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r(),
        quaternion.i(),
        Num::ZERO,
        Num::ZERO,
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the scalar part of a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::scalar_part;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// 
/// assert_eq!(
///     scalar_part::<f32, [f32; 4]>(&quat),
///     [1.2, 0.0, 0.0, 0.0]
/// )
/// ```
pub fn scalar_part<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r(),
        Num::ZERO,
        Num::ZERO,
        Num::ZERO,
    )
}
