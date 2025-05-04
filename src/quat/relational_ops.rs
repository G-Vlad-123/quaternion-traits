
use super::*;

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if two types represent the same quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::eq;
/// 
/// assert!( eq::<f32>(&[1.0, 2.0, 3.0, 4.0], &(1.0, 2.0, 3.0, 4.0)) );
/// ```
pub fn eq<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> bool
where 
    Num: Axis,
{
        left.r() == right.r()
     && left.i() == right.i()
     && left.j() == right.j()
     && left.k() == right.k()
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks to see if this quaternion is only a scalar value.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::is_scalar;
/// 
/// let yes_scalar = [3.14, 0.0, 0.0, 0.0];
/// let no_scalar = [1.2, 3.4, 5.6, 7.8];
/// 
/// assert!( is_scalar::<f32>(&yes_scalar) );
/// assert!( !is_scalar::<f32>(&no_scalar) );
/// ```
pub fn is_scalar<Num>(quaternion: impl Quaternion<Num>) -> bool
where 
    Num: Axis,
{
        quaternion.i() == Num::ZERO
     && quaternion.j() == Num::ZERO
     && quaternion.k() == Num::ZERO
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks to see if this quaternion is a complex value.
/// 
/// Note: this checks if [`q.j()`](Quaternion::j) and
/// [`q.k()`](Quaternion::k) return both [`Num::ZERO`](Axis::ZERO).
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::is_complex;
/// 
/// let yes_complex = [0.0, 1.0, 0.0, 0.0];
/// let no_complex = [1.2, 3.4, 5.6, 7.8];
/// 
/// assert!( is_complex::<f32>(&yes_complex) );
/// assert!( !is_complex::<f32>(&no_complex) );
/// ```
pub fn is_complex<Num>(quaternion: impl Quaternion<Num>) -> bool
where 
    Num: Axis,
{
        quaternion.j() == Num::ZERO
     && quaternion.k() == Num::ZERO
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks to see if this quaternion is a vector value. (or aure quaternion)
/// 
/// Note: this checks if [`q.r()`](Quaternion::r) returns [`Num::ZERO`](Axis::ZERO).
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::is_vector;
/// 
/// let yes_vector = [0.0, 1.0, 2.0, 3.0];
/// let no_vector = [1.2, 3.4, 5.6, 7.8];
/// 
/// assert!( is_vector::<f32>(&yes_vector) );
/// assert!( !is_vector::<f32>(&no_vector) );
/// ```
pub fn is_vector<Num>(quaternion: impl Quaternion<Num>) -> bool
where 
    Num: Axis,
{
    quaternion.r() == Num::ZERO
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks to see if this quaternion is on a axis plane.
/// 
/// Checks if at most 2 of the quaternion feilds are non-zero.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::is_on_axis_plane;
/// 
/// let yes_planar = [0.0, 3.14, 0.0, 2.71];
/// let no_planar = [1.0, 2.0, 3.0, 0.0];
/// 
/// assert!( is_on_axis_plane::<f32>(&yes_planar) );
/// assert!( !is_on_axis_plane::<f32>(&no_planar) );
/// ```
pub fn is_on_axis_plane<Num>(quaternion: impl Quaternion<Num>) -> bool
where 
    Num: Axis,
{
    1 < (quaternion.r() == Num::ZERO) as u8
      + (quaternion.i() == Num::ZERO) as u8
      + (quaternion.j() == Num::ZERO) as u8
      + (quaternion.k() == Num::ZERO) as u8
}

// #[inline]
// #[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
// /// Checks if two quaternions are on the same paralel to an axis plane plane.
// /// 
// /// Equivalent to checking if the two vectors subtracted 
// /// 
// /// # Example
// /// ```
// /// use quaternion_traits::quat::is_on_same_plane;
// /// 
// /// ass
// /// ```
// pub fn is_on_same_plane<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> bool
// where 
//     Num: Axis,
// {
//     1 < (left.r() == right.r()) as u8
//       + (left.i() == right.i()) as u8
//       + (left.j() == right.j()) as u8
//       + (left.k() == right.k()) as u8
// }

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the distance between two quaternions is less then [`Num::ERROR`](Axis::ERROR).
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::is_near;
/// 
/// let a: [f32; 4] = [0.0; 4];
/// let b: [f32; 4] = [0.00001, 0.0, 0.0, 0.0];
/// 
/// assert!( is_near::<f32>(&a, &b) );
/// ```
pub fn is_near<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> bool
where
    Num: Axis,
{
    abs_squared::<Num, Num>(&sub::<Num, Q<Num>>(left, right)) < Num::ERROR * Num::ERROR
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the distance between two quaternions is less then `error`.
/// 
/// If [`error.scalar()`](Scalar::scalar) evaluates to a non_
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::is_near_by;
/// 
/// let a: [f32; 4] = [0.0; 4];
/// let b: [f32; 4] = [0.5, 0.0, 0.0, 0.0];
/// 
/// assert!( is_near_by::<f32>(a, b, 1) );
/// ```
pub fn is_near_by<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool
where
    Num: Axis,
{
    abs_squared::<Num, Num>(&sub::<Num, Q<Num>>(left, right)) < error.scalar() * error.scalar()
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the ratio inbetween the abs of two quaternions is small enough
/// 
/// Checks if the ratio inbetween the absolute values of two quaternions
/// is strictly inbetween `Num::ONE - Num::ERROR` and `Num::ONE + Num::ERROR`
/// AND that the distance inbetween the angle
/// 
/// If eather quaternion is the origin, then [`is_near`] is used because
/// otherwise the algorithm used always give out false, even if they are equal.
/// 
/// [`is_close`] and [`is_near`] will not always give the same results.
/// 
/// # Example
/// 
/// They are close but not near.
/// ```
/// use quaternion_traits::quat::is_close;
/// use quaternion_traits::quat::is_near;
/// 
/// let a: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
/// let b: [f32; 4] = [1.0, 0.0001, -0.0001, 0.0001];
/// 
/// assert!( is_close::<f32>(a, b) );
/// assert!( !is_near::<f32>(a, b) );
/// ```
/// 
/// They are near but not close.
/// ```
/// use quaternion_traits::quat::is_close;
/// use quaternion_traits::quat::is_near;
/// 
/// let a: [f32; 4] = [1e+32, 0.0, 0.0, 0.0];
/// let b: [f32; 4] = [1e+32, 1e-16, -1e-16, 1e-16];
/// 
/// assert!( !is_close::<f32>(a, b) );
/// assert!( is_near::<f32>(a, b) );
/// ```
pub fn is_close<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> bool
where
    Num: Axis
{
    if eq(&left, ()) {
        return is_near(right, ());
    }
    if eq(&right, ()) {
        return is_near(left, ());
    }
    ( ( abs_squared::<Num, Num>(&left) / abs_squared::<Num, Num>(&right) ).sqrt() - Num::ONE ).abs()
    < Num::ERROR
    &&
    ((angle::<Num, Num>(left) - angle::<Num, Num>(right)).abs() < Num::ERROR)
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the ratio inbetween the abs of two quaternions is small enough
/// 
/// Checks if the ratio inbetween the absolute values of two quaternions
/// is strictly inbetween `Num::ONE - Num::ERROR` and `Num::ONE + Num::ERROR`
/// AND that the ratio inbetween the angle
/// 
/// [`is_close_by`] and [`is_near_by`] will not always give the same results.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::is_close_by;
/// 
/// let a: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
/// let b: [f32; 4] = [1.5, 0.5, 0.0, 0.0];
/// 
/// assert!( is_close_by::<f32>(a, b, 1) );
/// ```
pub fn is_close_by<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool
where
    Num: Axis
{
    if eq(&left, ()) {
        return is_near_by(right, (), error);
    }
    if eq(&right, ()) {
        return is_near_by(left, (), error);
    }
    ( ( abs_squared::<Num, Num>(&left) / abs_squared::<Num, Num>(&right) ).sqrt() - Num::ONE ).abs()
    < error.scalar()
    &&
    (angle::<Num, Num>(left) - angle::<Num, Num>(right)).abs() < error.scalar()
}
