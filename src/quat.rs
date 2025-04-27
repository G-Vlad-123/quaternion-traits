
use crate::core::option::Option;
use crate::{
    Axis,
    Quaternion, QuaternionConstructor,
    Vector, VectorConstructor,
    Complex, ComplexConstructor,
    Scalar, ScalarConstructor,
    Rotation, RotationConstructor,
};

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs the origin quaternion. (Aditive identity)
/// 
/// # Example
/// ```
/// use quaternion_traits::origin;
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
/// use quaternion_traits::identity;
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
/// use quaternion_traits::nan;
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

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if two types represent the same quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::eq;
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
/// Adds two quaternions.
/// 
/// # Example
/// ```
/// use quaternion_traits::add;
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [4.0, 3.0, 2.0, -4.0];
/// let c: [f32; 4] = add::<f32, [f32; 4]>(&a, &b);
/// 
/// assert_eq!( c, [5.0, 5.0, 5.0, 0.0] );
/// ```
pub fn add<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        left.r() + right.r(), 
        left.i() + right.i(), 
        left.j() + right.j(), 
        left.k() + right.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Subtracts a quaternion from another one.
/// 
/// # Example
/// ```rust
/// use quaternion_traits::sub;
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [4.0, 3.0, 2.0, -4.0];
/// let c: [f32; 4] = sub::<f32, [f32; 4]>(&a, &b);
/// 
/// assert_eq!( c, [-3.0, -1.0, 1.0, 8.0] );
/// ```
pub fn sub<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        left.r() - right.r(), 
        left.i() - right.i(), 
        left.j() - right.j(), 
        left.k() - right.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Multiplies a quaternion to another one.
/// 
/// Quaternion multiplication formula:
///     
///     # "
///     let w1, w2, x1, x2, y1, y2, z1, z2 be Real numbers
///     let q1 = w1 + x1*i + y1*j + z1*k
///     let q2 = w2 + x2*i + y2*j + z2*k
/// 
///        =>
/// 
///     q1 * q2 = w1*w2 - x1*x2 - y1*y2 - z1*z2       <---- scalar part
///           + ( w1*x2 - x1*w2 - y1*z2 - z1*y2 ) * i | <-- vector part
///           + ( w1*y2 - x1*z2 - y1*w2 - z1*x2 ) * j |
///           + ( w1*z2 - x1*y2 - y1*x2 - z1*w2 ) * k |
///     # ";
/// 
/// Since quaternion multiplication is acctualy neather comutative nor anti-comutative,
/// therefor `mul(q1, q2) == mul(q2, q1)` is NOT guaranteed for any q1 and q2.
pub fn mul<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        left.r() * right.r() - left.i() * right.i() - left.j() * right.j() - left.k() * right.k(),
        left.r() * right.i() + left.i() * right.r() + left.j() * right.k() - left.k() * right.j(),
        left.r() * right.j() - left.i() * right.k() + left.j() * right.r() + left.k() * right.i(),
        left.r() * right.k() + left.i() * right.j() - left.j() * right.i() + left.k() * right.r(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Multiplies two quaternions in reversed order.
/// 
/// Quaternions are neather comutative nor anti-comutative for multiplication,
/// aka neather `mul(q1, q2) == mul(q2, q1)` nor `mul(q1, q2) == neg(mul(q2, q1))`
/// are not guaranteed. So `mul(q1, q2)` and `mul(q2, q1)` give diferent results.
/// For the conveniance in some cases this function is given.
/// 
/// # Example
/// ```
/// use quaternion_traits::{mul, mul_reversed};
/// 
/// let a: [f32; 4] = // quaternion
/// # [1.0, 2.0, 0.0, 3.0];
/// let b: [f32; 4] = // quaternion
/// # [3.0, 1.0, 4.0, 0.0];
/// 
/// assert_eq!( mul::<f32, [f32; 4]>(&a, &b), mul_reversed::<f32, [f32; 4]>(&b, &a) );
/// ```
pub fn mul_reversed<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{ mul(right, left) }

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Divides a quaternion by another one.
/// 
/// Is equivalent to multiplying a quaternion with
/// another one's inverse.
/// 
/// ```
/// use quaternion_traits::{mul, div, inv};
/// 
/// let a: [f32; 4] = // quaternion
/// # [1.0, 2.0, 0.0, 3.0];
/// let b: [f32; 4] = // quaternion
/// # [3.0, 1.0, 4.0, 0.0];
/// 
/// assert_eq!(
///     mul::<f32, [f32; 4]>(&a, &inv::<f32, [f32; 4]>(&b)),
///     div::<f32, [f32; 4]>(&a, &b)
/// );
/// ```
pub fn div<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    mul::<Num, Out>(left, &inv::<Num, [Num; 4]>(right))
}

/// Divides a quaternion by another one in reversed order.
/// 
/// Since quaternion multiplication is neather commutative nor
/// anti-commutative and since division is just multiplying by the inverse
/// 
/// `div(q1, q2) = q1 * inv(q2)` 
/// `div_reversed(q1, q2) = inv(q2) * q1` 
pub fn div_reversed<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    mul::<Num, Out>(&inv::<Num, [Num; 4]>(left), &right)
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the negative of this quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::neg;
/// 
/// let quat = [1.0, 2.0, 3.0, 4.0];
/// let neg_quat = [-1.0, -2.0, -3.0, -4.0];
/// 
/// assert_eq!( neg::<f32, [f32; 4]>(&quat), neg_quat );
/// ```
pub fn neg<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        -quaternion.r(), 
        -quaternion.i(), 
        -quaternion.j(), 
        -quaternion.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the conjugate of this quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::conj;
/// 
/// let quat = [1.0, 2.0, 3.0, 4.0];
/// let conj_quat = [1.0, -2.0, -3.0, -4.0];
/// 
/// assert_eq!( conj::<f32, [f32; 4]>(&quat), conj_quat );
/// ```
pub fn conj<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r(),
        - quaternion.i(),
        - quaternion.j(),
        - quaternion.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks to see if this quaternion is only a scalar value.
/// 
/// # Example
/// ```
/// use quaternion_traits::is_scalar;
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
/// use quaternion_traits::is_complex;
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
/// use quaternion_traits::is_vector;
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
/// use quaternion_traits::is_on_axis_plane;
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
// /// use quaternion_traits::is_on_same_plane;
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
/// Scales a quaternion.
/// 
/// Equivalent to multiplying a quaternion by a scalar quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::scale;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, 2.0, 3.0];
/// let scaled: [f32; 4] = scale::<f32, [f32; 4]>(&quat, &2);
/// 
/// assert_eq!( scaled, [0.0, 2.0, 4.0, 6.0] );
/// ```
pub fn scale<Num, Out>(quaternion: impl Quaternion<Num>, scalar: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r() * scalar.scalar(),
        quaternion.i() * scalar.scalar(),
        quaternion.j() * scalar.scalar(),
        quaternion.k() * scalar.scalar(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Scales a quaternion by the inverse of the scalar.
/// 
/// Equivalent to dividing a quaternion by a scalar quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::unscale;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, 2.0, 3.0];
/// let unscaled: [f32; 4] = unscale::<f32, [f32; 4]>(&quat, &2);
/// 
/// assert_eq!( unscaled, [0.0, 0.5, 1.0, 1.5] );
/// ```
pub fn unscale<Num, Out>(quaternion: impl Quaternion<Num>, scalar: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r() / scalar.scalar(),
        quaternion.i() / scalar.scalar(),
        quaternion.j() / scalar.scalar(),
        quaternion.k() / scalar.scalar(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the distance between two quaternions is less then [`Num::ERROR`](Axis::ERROR).
/// 
/// ```
/// use quaternion_traits::{is_near, Axis};
/// 
/// let a: [f32; 4] = [0.0; 4];
/// let b: [f32; 4] = [<f32 as Axis>::ERROR / 2.0, 0.0, 0.0, 0.0];
/// 
/// assert!( is_near::<f32>(&a, &b) );
/// ```
pub fn is_near<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> bool
where
    Num: Axis,
{
    abs_squared::<Num, Num>(&sub::<Num, [Num; 4]>(left, right)) < Num::ERROR * Num::ERROR
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the distance between two quaternions is less then `error`.
/// 
/// If [`error.scalar()`](Scalar::scalar) evaluates to a non_
/// 
/// ```
/// use quaternion_traits::{is_near, Axis};
/// 
/// let a: [f32; 4] = [0.0; 4];
/// let b: [f32; 4] = [<f32 as Axis>::ERROR / 2.0, 0.0, 0.0, 0.0];
/// 
/// assert!( is_near::<f32>(&a, &b) );
/// ```
pub fn is_near_by<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool
where
    Num: Axis,
{
    abs_squared::<Num, Num>(&sub::<Num, [Num; 4]>(left, right)) < error.scalar() * error.scalar()
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the ratio inbetween the absolute values of two quaternions
/// is strictly inbetween `Num::ONE - Num::ERROR` and `Num::ONE + Num::ERROR`
/// AND that the distance inbetween the angle
/// 
/// Note: `is_close` and `is_near` will not always give the same results.
pub fn is_close<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> bool
where
    Num: Axis
{
    ( ( abs_squared::<Num, Num>(&left) / abs_squared::<Num, Num>(&right) ).sqrt() - Num::ONE ).abs()
    < Num::ERROR
    &&
    (angle::<Num, Num>(left) - angle::<Num, Num>(right)).abs() < Num::ERROR
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Checks if the ratio inbetween the absolute values of two quaternions
/// is strictly inbetween `Num::ONE - Num::ERROR` and `Num::ONE + Num::ERROR`
/// AND that the ratio inbetween the angle
/// 
/// Note: `is_close` and `is_near` will not always give the same results.
pub fn is_close_by<Num>(left: impl Quaternion<Num>, right: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool
where
    Num: Axis
{
    ( ( abs_squared::<Num, Num>(&left) / abs_squared::<Num, Num>(&right) ).sqrt() - Num::ONE ).abs()
    < error.scalar()
    &&
    (angle::<Num, Num>(left) - angle::<Num, Num>(right)).abs() < error.scalar()
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the distance inbetween the coordenates of two quaternions.
/// 
/// Equivalent to getting the absolute value of 
/// 
/// ```
/// use quaternion_traits::dist;
/// 
/// let a: [f32; 4] = [5.0, 0.0, 1.0, 3.0];
/// let b: [f32; 4] = [2.0, 0.0, 5.0, 3.0];
/// 
/// assert_eq!( dist::<f32, f32>(&a, &b), 5.0 );
/// assert_eq!( dist::<f32, f32>(&a, &a), 0.0 );
/// ```
pub fn dist<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    abs(&sub::<Num, [Num; 4]>(from, to))
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the normal of a quaternion.
/// 
/// The normal of a quaternion always has the same "direction"
/// but with an absolute value of [`Num::ONE`](Axis::ONE).
/// 
/// If the quaternion is the origin it returns the origin.
/// 
/// # Example
/// ```
/// use quaternion_traits::norm;
/// 
/// let quat: [f32; 4] = [0.0, 3.25, 0.0, 0.0];
/// let normal: [f32; 4] = norm::<f32, [f32; 4]>(&quat);
/// 
/// assert_eq!( normal, [0.0, 1.0, 0.0, 0.0] );
/// ```
pub fn norm<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if eq(&quaternion, &()) { return origin() }
    let length: Num = Num::ONE / abs(&quaternion);
    Out::new_quat(
        quaternion.r() * length,
        quaternion.i() * length,
        quaternion.j() * length,
        quaternion.k() * length,
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the absolute value of a quaternion. (Also knows as it's "length")
/// 
/// # Example
/// ```
/// use quaternion_traits::abs;
/// 
/// let quat: [f32; 4] = [1.0, 3.0, 9.0, 3.0];
/// 
/// assert_eq!( abs::<f32, f32>(&quat), 10.0 );
/// ```
pub fn abs<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar( Num::sqrt(
        quaternion.r() * quaternion.r()
        + quaternion.i() * quaternion.i()
        + quaternion.j() * quaternion.j()
        + quaternion.k() * quaternion.k()
    ) )
}

// TODO test this
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the absolute value of a quaternion close to the origin.
/// 
/// Calculates the sqared absolute value of the quaternion multiplied by [`Num::ERROR`](Axis::ERROR) to the -2 power.
/// Then it takes the square root.
/// The it multiplies by [`Num::ERROR`](Axis::ERROR) to the first power.
/// 
/// Note: The operations above are the rough order in which they are done.
pub fn small_abs<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    let factor = Num::ONE / Num::ERROR / Num::ERROR;
    Out::new_scalar( Num::sqrt(
          quaternion.r() * factor * quaternion.r()
        + quaternion.i() * factor * quaternion.i()
        + quaternion.j() * factor * quaternion.j()
        + quaternion.k() * factor * quaternion.k()
    ) * Num::ERROR )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the squared absolute value of a quaternion. (Also knows as it's squared "length")
/// 
/// # Example
/// ```
/// use quaternion_traits::abs_squared;
/// 
/// let quat: [f32; 4] = [1.0, 3.0, 9.0, 3.0];
/// 
/// assert_eq!( abs_squared::<f32, u32>(&quat), 100 );
/// ```
pub fn abs_squared<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(
        quaternion.r() * quaternion.r()
      + quaternion.i() * quaternion.i()
      + quaternion.j() * quaternion.j()
      + quaternion.k() * quaternion.k()
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the angle of a quaternion's polar form.
pub fn angle<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(Num::acos(quaternion.r() / abs::<Num, Num>(quaternion)))
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the cosine of the angle of a quaternion's polar form.
pub fn angle_cos<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(quaternion.r() / abs::<Num, Num>(quaternion))
}

// use `is_near` instead
// --- vvvvvvvvv -------

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the inverse quaternion of a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::{inv, mul, identity, is_near};
/// 
/// let quat: [f32; 4] = [1.0, 3.0, 9.0, 3.0];
/// let inv_quat: [f32; 4] = inv::<f32, [f32; 4]>(&quat);
/// 
/// assert!( is_near::<f32>(
///     &mul::<f32, [f32; 4]>(&quat, &inv_quat),
///     &identity::<f32, [f32; 4]>()
/// ) );
/// ```
/// The function [`is_near`] is used here because of finite floating point precision.
pub fn inv<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if eq(&quaternion, &()) {
        return Out::from_quat([Num::NAN; 4]);
    }
    let inv: Num = Num::ONE / abs_squared(&quaternion);
    Out::new_quat(
         quaternion.r() * inv,
        -quaternion.i() * inv,
        -quaternion.j() * inv,
        -quaternion.k() * inv,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the natural logarithm of a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::{ln, exp, is_near};
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let ln_quat: [f32; 4] = ln::<f32, [f32; 4]>(&quat);
/// 
/// assert!( is_near::<f32>(&exp::<f32, [f32; 4]>(&ln_quat), &quat) );
/// ```
/// The function [`is_near`] is used here because of finite floating point precision.
pub fn ln<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let absolute: Num = abs(&quaternion);
    add(
        &scale::<Num, [Num; 4]>(
            &norm::<Num, [Num; 4]>(
                &vector_part::<Num, [Num; 4]>(&quaternion),
            ),
            (quaternion.r() / absolute).acos()
        ), 
        &(absolute.ln(), ())
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Raises the number e to a quaternion power.
/// 
/// e ≈ 2.71828...
/// 
/// ```
/// use quaternion_traits::{exp, ln, is_near};
/// 
/// let quat: [f32; 4] = [1.0, 3.14, 0.0, 0.0];
/// let exp_quat: [f32; 4] = exp::<f32, [f32; 4]>(&quat);
/// 
/// println!("{:?}", exp_quat);
/// println!("{:?}", ln::<f32, [f32; 4]>(&exp_quat));
/// println!("{:?}", quat);
/// assert!( is_near::<f32>(&ln::<f32, [f32; 4]>(&exp_quat), &quat) );
/// ```
/// The function [`is_near`] is used here because of finite floating point precision.
pub fn exp<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let vec: [Num; 4] = vector_part(&quaternion);
    let (sin, cos) = abs::<Num, Num>(&vec).sin_cos();
    scale::<Num, Out>(
        &add::<Num, [Num; 4]>(
            &scale::<Num, [Num; 4]>(
                &norm::<Num, [Num; 4]>(&vec),
                sin
            ),
            &(cos, ())
        ),
        quaternion.r().exp(),
    )
}

#[inline]
#[cfg(feature = "unstable")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the logarithm of a quaternion with a quaternion base.
/// 
/// # Example
/// ```
/// use quaternion_traits::{log, pow_i};
/// 
/// let base: [f32; 4] = [0.0, 2.0, 1.0, 0.0];
/// let quat: [f32; 4] = pow_i::<f32, [f32; 4]>(&base, 3);
/// let log_quat: [f32; 4] = log::<f32, [f32; 4]>(&base, &quat);
/// 
/// assert_eq!( log_quat, [3.0, 0.0, 0.0, 0.0] );
/// ```
pub fn log<Num, Out>(base: impl Quaternion<Num>, num: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    div::<Num, Out>(&ln::<Num, [Num; 4]>(num), &ln::<Num, [Num; 4]>(base))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the sqaure root of a quaternion.
/// 
/// This uses a diferent algorthm from [`pow_f`].
pub fn sqrt<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if is_scalar(&quaternion) {
        use crate::core::cmp::Ordering;
        use crate::core::option::Option::Some;
        return match quaternion.r().partial_cmp(&Num::ZERO) {
            Some(Ordering::Greater) => Out::from_quat((quaternion.r().sqrt(), ())),
            Some(Ordering::Less) => Out::from_quat((Num::ZERO, (-quaternion.r()).sqrt(), Num::ZERO, Num::ZERO)),
            _ => nan(),
        }
    }
    let r: Num = quaternion.r();
    let unit = norm::<Num, [Num; 4]>(&vector_part::<Num, [Num; 4]>(&quaternion));
    let abs: Num = abs::<Num, Num>(&quaternion);
    let unreal_part: Num = Num::sqrt( (abs - r) / (Num::ONE + Num::ONE) );
    Out::new_quat (
        Num::sqrt( (abs + r) / (Num::ONE + Num::ONE) ),
        unit[1] * unreal_part,
        unit[2] * unreal_part,
        unit[3] * unreal_part,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Raises a quaternion to an integer power.
/// 
/// This is evaluated by repeated multiplication.
/// For large (or small) values use [`pow_f`].
pub fn pow_i<Num, Out>(base: impl Quaternion<Num>, mut exp: i32) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if eq(&base, &()) {
        if exp > 0 { return origin(); }
        return nan()
    }
    if eq(&base, &identity::<Num, [Num; 4]>()) { return identity() }
    if exp == 0 { return identity(); }
    let mut out: [Num; 4] = identity::<Num, [Num; 4]>();
    let is_inverse = exp < 0;
    if is_inverse { exp = -exp }
    for _ in 0..exp {
        out = mul(&out, &base);
    }
    if is_inverse { inv(&out) } else { Out::from_quat(out) }
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Raises a quaternion to a positive integer power.
/// 
/// This is evaluated by repeated multiplication.
/// For larger values use [`pow_f`].
pub fn pow_u<Num, Out>(base: impl Quaternion<Num>, exp: u32) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if eq(&base, &()) { return origin(); }
    if eq(&base, &identity::<Num, [Num; 4]>()) { return identity() }
    if exp == 0 { return identity(); }
    let mut out = identity::<Num, [Num; 4]>();
    for _ in 0..exp {
        out = mul(&out, &base);
    }
    Out::from_quat(out)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Raises a quaternion to a scalar power.
/// 
/// Doesn't use eather `exp(ln(base) * exp)` or `exp(exp * ln(base))`.
pub fn pow_f<Num, Out>(base: impl Quaternion<Num>, exp: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let abs: Num = abs(&base);
    let angle = (base.r() / abs).acos();
    scale(
        &crate::exp::<Num, [Num; 4]>(
            &scale::<Num, [Num; 4]>(
                &vector_part::<Num, [Num; 4]>(base),
                exp.scalar() * angle
            )
        ),
        abs.pow(exp.scalar()) // replaces one use of sqrt with one div
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[cfg(feature = "unstable")]
/// Raises a quaternion to a quaternion power.
/// 
/// Used this paper as a refrence:
/// [link](https://web.archive.org/web/20170705123142/http://www.lce.hut.fi/~ssarkka/pub/quat.pdf)
/// 
/// Calculates `exp(ln(base) * exp)`, `exp(exp * ln(base))` may also be valid but it may give a diferent result.
pub fn pow_q<Num, Out>(base: impl Quaternion<Num>, exp: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if eq(&base, &()) {
        if is_scalar(&exp) && crate::core::matches!( exp.r().partial_cmp(&Num::ZERO), Option::Some(crate::core::cmp::Ordering::Greater) ) {
            return origin();
        }
        return nan();
    }
    // refrence: https://web.archive.org/web/20170705123142/http://www.lce.hut.fi/~ssarkka/pub/quat.pdf
    crate::quat::exp(&mul::<Num, [Num; 4]>(&ln::<Num, [Num; 4]>(base), &exp))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the dot product of two quaternions.
/// 
/// Fun fact: the dot product of a quaternion with it'self returns the squared absolute value :)
/// 
/// # Example
/// ```
/// use quaternion_traits::dot;
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [5.0, 2.0, 1.0, 2.0];
/// let dot_product: f32 = dot::<f32, f32>(&a, &b);
/// 
/// assert_eq!( dot_product, 20.0 );
/// ```
pub fn dot<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(
        left.r() * right.r()
      + left.i() * right.i()
      + left.j() * right.j()
      + left.k() * right.k()
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the sinus of a quaternion.
pub fn sin<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    // refrence: https://math.stackexchange.com/questions/1499095/how-to-calculate-sin-cos-tan-of-a-quaternion
    let abs_vec = Num::sqrt(quaternion.i()*quaternion.i() + quaternion.j()*quaternion.j() + quaternion.k()*quaternion.k());
    let vec_scalar = quaternion.r().cos() * abs_vec.sinh() / abs_vec;
    Out::new_quat(
        quaternion.r().sin() * abs_vec.cosh(), 
        quaternion.i() * vec_scalar, 
        quaternion.j() * vec_scalar, 
        quaternion.k() * vec_scalar,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the hyperbolic sinus of a quaternion.
pub fn sinh<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let exp = exp::<Num, [Num; 4]>(quaternion);
    unscale(&sub::<Num, [Num; 4]>(&exp, &inv::<Num, [Num; 4]>(&exp)), Num::ONE + Num::ONE)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the secant of a quaternion.
pub fn sec<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    inv(&cos::<Num, [Num; 4]>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the  cosinus of a quaternion.
pub fn cos<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    // refrence: https://math.stackexchange.com/questions/1499095/how-to-calculate-sin-cos-tan-of-a-quaternion
    let abs_vec = Num::sqrt(quaternion.i()*quaternion.i() + quaternion.j()*quaternion.j() + quaternion.k()*quaternion.k());
    let vec_scalar = - quaternion.r().sin() * abs_vec.sinh() / abs_vec;
    Out::new_quat(
        quaternion.r().cos() * abs_vec.cosh(), 
        quaternion.i() * vec_scalar, 
        quaternion.j() * vec_scalar, 
        quaternion.k() * vec_scalar,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the hyperbolic cosinus of a quaternion.
pub fn cosh<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let exp = exp::<Num, [Num; 4]>(quaternion);
    unscale(&add::<Num, [Num; 4]>(&exp, &inv::<Num, [Num; 4]>(&exp)), Num::ONE + Num::ONE)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the cosecant of a quaternion.
pub fn csc<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    inv(&sin::<Num, [Num; 4]>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the sinus and cosinus of a quaternion at once.
pub fn sin_cos<Num, Out>(quaternion: impl Quaternion<Num>) -> (Out, Out)
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    // refrence: https://math.stackexchange.com/questions/1499095/how-to-calculate-sin-cos-tan-of-a-quaternion
    let abs_vec = Num::sqrt(quaternion.i()*quaternion.i() + quaternion.j()*quaternion.j() + quaternion.k()*quaternion.k());
    let vec_scalar = abs_vec.sinh() / abs_vec;
    let vec_scalar_cos = quaternion.r().cos() * vec_scalar;
    let vec_scalar_sin = quaternion.r().sin() * - vec_scalar;
    let abs_vec_cosh = abs_vec.cosh();
    (
        Out::new_quat(
            quaternion.r().sin() * abs_vec_cosh, 
            quaternion.i() * vec_scalar_sin, 
            quaternion.j() * vec_scalar_sin, 
            quaternion.k() * vec_scalar_sin,
        ),
        Out::new_quat(
            quaternion.r().cos() * abs_vec_cosh, 
            quaternion.i() * vec_scalar_cos, 
            quaternion.j() * vec_scalar_cos, 
            quaternion.k() * vec_scalar_cos,
        ),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the tangent of a quaternion
pub fn tan<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let (sin, cos) = sin_cos::<Num, [Num; 4]>(quaternion);
    div(&sin, &cos)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the sinus of a quaternion.
pub fn tanh<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let exp = exp::<Num, [Num; 4]>(quaternion);
    let inv = inv::<Num, [Num; 4]>(&exp);
    div(
        &sub::<Num, [Num; 4]>(&exp, &inv),
        &add::<Num, [Num; 4]>(&exp, &inv),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the cotangent of a quaternion
pub fn cot<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let (sin, cos) = sin_cos::<Num, [Num; 4]>(quaternion);
    div(&cos, &sin)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the sinus of a quaternion.
pub fn coth<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let exp = exp::<Num, [Num; 4]>(quaternion);
    let inv = inv::<Num, [Num; 4]>(&exp);
    div(
        &add::<Num, [Num; 4]>(&exp, &inv),
        &sub::<Num, [Num; 4]>(&exp, &inv),
    )
}

// Note: You can add the hyperbolic and inverse functions too probably using the linked refrence

use crate::core::iter::Iterator;

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Adds all the quaternions in an iterator.
/// 
/// Returns the origin quaternion if the iterator is empty.
/// 
/// # Example
/// ```
/// use quaternion_traits::{sum, add};
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [3.0, -2.0, 1.0, -4.0];
/// let c: [f32; 4] = [1.0, 1.3, 2.2, 3.1];
/// 
/// let normal: [f32; 4] = add::<f32, [f32; 4]>(&add::<f32, [f32; 4]>(&a, &b), &c);
/// let iter: [f32; 4] = sum::<f32, [f32; 4]>( [a, b, c] );
/// 
/// assert_eq!(normal, iter);
/// ```
pub fn sum<Num, Out>(iter: impl crate::core::iter::IntoIterator<Item: Quaternion<Num>>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut sum = [Num::ZERO; 4];
    for quaternion in iter {
        sum = add(&sum, &quaternion);
    }
    Out::from_quat(sum)
}

const PRODUCT_MARGIN: usize = 0xFFFFFFF;
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Multiplies all the quaternions in an iterator.
/// 
/// Returns the identity quaternion if the iterator is empty.
/// 
/// # Example
/// ```
/// use quaternion_traits::{product, mul};
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [3.0, -2.0, 1.0, -4.0];
/// let c: [f32; 4] = [1.0, 1.3, 2.2, 3.1];
/// 
/// let normal: [f32; 4] = mul::<f32, [f32; 4]>(&mul::<f32, [f32; 4]>(&a, &b), &c);
/// let iter: [f32; 4] = product::<f32, [f32; 4]>( [a, b, c] );
/// 
/// assert_eq!(normal, iter);
/// ```
pub fn product<Num, Out>(iter: impl crate::core::iter::IntoIterator<Item: Quaternion<Num>>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut iter = iter.into_iter();
    let mut sum = match iter.next() {
        Option::Some(ok) => <[Num; 4]>::from_quat(ok),
        Option::None => return identity(),
    };
    if Iterator::size_hint(&iter).0 > PRODUCT_MARGIN
    || match Iterator::size_hint(&iter).1 {
        Option::Some(some) => some > PRODUCT_MARGIN << 1,
        Option::None => true,
    } {
        for quaternion in iter {
            sum = mul(&sum, &quaternion);
            if eq(&sum, &()) {
                return Out::from_quat(());
            }
        }
    } else {
        for quaternion in iter {
            sum = mul(&sum, &quaternion);
        }
    }
    Out::from_quat(sum)
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the vector part of a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::vector_part;
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
/// use quaternion_traits::complex_part;
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
/// use quaternion_traits::scalar_part;
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

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a vector representation.
/// 
/// # Example
/// ```
/// use quaternion_traits::from_vector;
/// 
/// let vector: [f32; 3] = [3.14, 2.71, 1.23];
/// let quat: [f32; 4] = from_vector::<f32, [f32; 4]>(&vector);
/// 
/// assert_eq!( quat, [0.0, 3.14, 2.71, 1.23] );
/// ```
pub fn from_vector<Num, Out>(vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    QuaternionConstructor::new_quat(
        Num::ZERO,
        vector.x(),
        vector.y(),
        vector.z(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a complex number representation.
/// 
/// # Example
/// ```
/// use quaternion_traits::from_complex;
/// 
/// let complex: [f32; 2] = [3.14, 2.71];
/// let quat: [f32; 4] = from_complex::<f32, [f32; 4]>(&complex);
/// 
/// assert_eq!( quat, [3.14, 2.71, 0.0, 0.0] );
/// ```
pub fn from_complex<Num, Out>(complex: impl Complex<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    QuaternionConstructor::new_quat(
        complex.real(),
        complex.imaginary(),
        Num::ZERO,
        Num::ZERO,
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a scalar value.
/// 
/// # Example
/// ```
/// use quaternion_traits::from_scalar;
/// 
/// let scalar: f32 = 3.14;
/// let quat: [f32; 4] = from_scalar::<f32, [f32; 4]>(&scalar);
/// 
/// assert_eq!( quat, [3.14, 0.0, 0.0, 0.0] );
/// ```
pub fn from_scalar<Num, Out>(complex: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    QuaternionConstructor::new_quat(
        complex.scalar(),
        Num::ZERO,
        Num::ZERO,
        Num::ZERO,
    )
}

// TODO add is_near

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a rotation.
/// 
/// # Example
/// ```
/// use quaternion_traits::from_rotation;
/// use core::f32::consts::PI;
/// 
/// let rotation: [f32; 3] = [PI, 0.0, 0.0];
/// let quat: [f32; 4] = from_rotation::<f32, [f32; 4]>(&rotation);
/// 
/// assert_eq!( quat, [-4.371139e-8, 1.0, 0.0, 0.0] );
/// ```
pub fn from_rotation<Num, Out>(rotation: impl Rotation<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let cos_r: Num = Num::cos(rotation.roll() / (Num::ONE + Num::ONE));
    let sin_r: Num = Num::sin(rotation.roll() / (Num::ONE + Num::ONE));
    let cos_p: Num = Num::cos(rotation.pitch() / (Num::ONE + Num::ONE));
    let sin_p: Num = Num::sin(rotation.pitch() / (Num::ONE + Num::ONE));
    let cos_y: Num = Num::cos(rotation.yaw() / (Num::ONE + Num::ONE));
    let sin_y: Num = Num::sin(rotation.yaw() / (Num::ONE + Num::ONE));
    QuaternionConstructor::new_quat(
        cos_r * cos_p * cos_y + sin_r * sin_p * sin_y,
        sin_r * cos_p * cos_y + cos_r * sin_p * sin_y,
        cos_r * sin_p * cos_y + sin_r * cos_p * sin_y,
        cos_r * cos_p * sin_y + sin_r * sin_p * cos_y,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates a quaternion using the given polar form.
/// 
/// Returns [`None`](Option::None) if the absolute value of `unit_vec`
/// is not near [`Num::ONE`](Axis::ONE).
pub fn from_polar_form<Num, Out>(abs: impl Scalar<Num>, angle: impl Scalar<Num>, unit_vec: impl Vector<Num>) -> Option<Out>
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if (unit_vec.x() * unit_vec.x() + unit_vec.y() * unit_vec.y() + unit_vec.z() * unit_vec.z() - Num::ONE).abs() >= Num::ERROR * Num::ERROR {
        return Option::None;
    }
    let (sin, cos) = angle.scalar().sin_cos();
    Option::Some( Out::new_quat(
        abs.scalar() * cos,
        abs.scalar() * sin * unit_vec.x(),
        abs.scalar() * sin * unit_vec.y(),
        abs.scalar() * sin * unit_vec.z(),
    ) )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates a quaternion using the given polar form.
/// 
/// # Safety
/// Make sure the absolute value of `unit_vec` is near [Num::ONE](Axis::ONE).
pub unsafe fn from_polar_form_unchecked<Num, Out>(abs: impl Scalar<Num>, angle: impl Scalar<Num>, unit_vec: impl Vector<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let (sin, cos) = angle.scalar().sin_cos();
    Out::new_quat(
        abs.scalar() * cos,
        abs.scalar() * sin * unit_vec.x(),
        abs.scalar() * sin * unit_vec.y(),
        abs.scalar() * sin * unit_vec.z(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a complex number representation from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::to_vector;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// let vector: [f32; 3] = to_vector::<f32, [f32; 3]>(&quat);
/// 
/// assert_eq!( vector, [3.4, 5.6, 7.8] );
/// ```
pub fn to_vector<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    Out::new_vector(
        quaternion.i(),
        quaternion.j(),
        quaternion.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a complex number representation from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::to_complex;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// let complex: [f32; 2] = to_complex::<f32, [f32; 2]>(&quat);
/// 
/// assert_eq!( complex, [1.2, 3.4] );
/// ```
pub fn to_complex<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ComplexConstructor<Num>,
{
    Out::new_complex(
        quaternion.r(),
        quaternion.i(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a scalar value from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::to_scalar;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// let scalar = to_scalar::<f32, f32>(&quat);
/// 
/// assert_eq!( scalar, 1.2 );
/// ```
pub fn to_scalar<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(
        quaternion.r(),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a rotation representation from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::to_rotation;
/// use core::f32::consts::PI;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, 0.0, 0.0];
/// let rotation: [f32; 3] = to_rotation::<f32, [f32; 3]>(&quat);
/// 
/// assert_eq!( rotation, [PI, 0.0, 0.0] );
/// ```
pub fn to_rotation<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: RotationConstructor<Num>
{
    let quat: [Num; 4] = norm(quaternion);

    let two = Num::ONE + Num::ONE;
    // here I misspelled 'pitch' but it's funny so I kept it
    let peach = two * (quat.r() * quat.j() - quat.i() * quat.k());

    if peach > Num::ONE - Num::ERROR {
        return RotationConstructor::new_rotation(
            two * Num::atan2(quat.i(), quat.r()),
            Num::TAU / (two + two),
            Num::ZERO,
        )
    }

    if peach < Num::ERROR - Num::ONE {
        return RotationConstructor::new_rotation(
            -two * Num::atan2(quat.i(), quat.r()),
            -Num::TAU / (two + two),
            Num::ZERO,
        )
    }

    let roll: Num = Num::atan2(
        two * (quat.r() * quat.i() + quat.j() * quat.k()),
        Num::ONE - two * ( quat.i() * quat.i() + quat.j() * quat.j())
    );
    let pitch: Num = Num::asin(peach);
    let yaw: Num = Num::atan2(
        two * (quat.r() * quat.k() + quat.i() * quat.j()),
        Num::ONE - two * ( quat.j() * quat.j() + quat.k() * quat.k() )
    );
    RotationConstructor::new_rotation(roll, pitch, yaw)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the polar form of a quaternion.
/// 
/// The values are given in this order (`abs`, `angle`, `unit_vec`).
/// Where:
///     - `abs` = `abs(q)`
///     - `angle` = `angle(q)`
///     - `unit_vec` = `norm(vector_part(q))`
/// 
/// The equasion used: `q == abs * exp(angle * unit_vec)`
pub fn to_polar_form<Num, Abs, Angle, UnitVec>(quaternion: impl Quaternion<Num>) -> (Abs, Angle, UnitVec)
where 
    Num: Axis,
    Abs: ScalarConstructor<Num>,
    Angle: ScalarConstructor<Num>,
    UnitVec: VectorConstructor<Num>,
{
    let abs = abs(&quaternion);
    let vec_inv_abs = Num::ONE / Num::sqrt(
          quaternion.i() * quaternion.i()
        + quaternion.j() * quaternion.j()
        + quaternion.k() * quaternion.k()
    );
    (
        Abs::new_scalar(abs),
        Angle::new_scalar(Num::acos(quaternion.r() / abs)),
        UnitVec::new_vector(
            quaternion.i() * vec_inv_abs,
            quaternion.j() * vec_inv_abs,
            quaternion.k() * vec_inv_abs,
        )
    )
}

// Thanks to quaternion crate for formula.
/// Gives the vector rotated by the given quaternion
pub fn rotate_vector<Num, Out>(vector: impl Vector<Num>, quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    let two = Num::ONE + Num::ONE;
    let cross: [Num; 3] = [
        two * (vector.y() * quaternion.k() - vector.z() * quaternion.j()),
        two * (vector.z() * quaternion.i() - vector.x() * quaternion.k()),
        two * (vector.x() * quaternion.j() - vector.y() * quaternion.i()),
    ];
    Out::new_vector(
        vector.x() + cross[0] * quaternion.r() + quaternion.j() * cross[2] - quaternion.k() * cross[1],
        vector.y() + cross[1] * quaternion.r() + quaternion.k() * cross[0] - quaternion.i() * cross[2],
        vector.z() + cross[2] * quaternion.r() + quaternion.i() * cross[1] - quaternion.j() * cross[0],
    )
}

// Thanks to quaternion crate for formula.
/// Constructs a quaternion representing the rotation inbetween two vectors.
pub fn rotation_from_to<Num, Out>(from: impl Vector<Num>, to: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut len: Num;

    let from: [Num; 3] = {
        len = Num::ONE / ( from.x() * from.x() + from.y() * from.y() + from.z() * from.z() ).sqrt();
        [
            from.x() * len,
            from.y() * len,
            from.z() * len,
        ]
    };

    let to: [Num; 3] = {
        len = Num::ONE / ( to.x() * to.x() + to.y() * to.y() + to.z() * to.z() ).sqrt();
        [
            to.x() * len,
            to.y() * len,
            to.z() * len,
        ]
    };

    let dot: Num = from.x() * to.x() + from.y() * to.y() + from.z() * to.z();

    // from and to are parallel
    if dot >= Num::ONE {
        return identity();
    }

    // from and to are anti-parallel
    if dot < Num::ERROR - Num::ONE {
        let mut axis: [Num; 3] = if from[2] != Num::ZERO && from[1] != Num::ZERO {
            [
                Num::ZERO,
                -from[2],
                from[1],
            ]
        } else {
            [
                Num::ZERO,
                Num::ZERO,
                -from[0],
            ]
        };
        len = Num::ONE / (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        axis = [axis[0] * len, axis[1] * len, axis[2] * len];
        unsafe {
            return axis_angle_unchecked(axis, Num::TAU / (Num::ONE + Num::ONE));
        }
    }

    let quat: [Num; 4] = [
        Num::ONE + dot,
        from[1] * to[2] - from[2] * to[1],
        from[2] * to[0] - from[0] * to[2],
        from[0] * to[1] - from[1] * to[0],
    ];
    scale(&quat, Num::ONE / abs(quat))
}

/// Constructs a quaternion from a given axis unit vector and a given angle.
/// 
/// Returns [`None`](Option::None) if the vector is not a unit vector
pub fn axis_angle<Num, Out>(axis: impl Vector<Num>, angle: Num) -> crate::core::option::Option<Out>
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut len = axis.x()*axis.x() + axis.y()*axis.y() + axis.z()*axis.z() - Num::ONE;
    if len < Num::ZERO { len = -len; }
    if len > Num::ERROR * Num::ERROR {
        return crate::core::option::Option::None;
    }
    unsafe {
        crate::core::option::Option::Some(axis_angle_unchecked(axis, angle))
    }
}

/// Constructs a quaternion from a given axis unit vector and a given angle.
/// 
/// # Safety
/// The vector must be a unit vector
pub unsafe fn axis_angle_unchecked<Num, Out>(axis: impl Vector<Num>, angle: Num) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let half_angle = angle / (Num::ONE + Num::ONE);
    let half_angle_sin = half_angle.sin();
    Out::new_quat(
        half_angle.cos(),
        axis.x() * half_angle_sin,
        axis.y() * half_angle_sin,
        axis.z() * half_angle_sin,
    )
}

use crate::core::write;
use crate::core::result::Result;

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Writes a quaternion representation to a formatter/string.
pub fn display<Num: Axis + crate::core::fmt::Display>(quaternion: impl Quaternion<Num>, target: &mut impl crate::core::fmt::Write) -> crate::core::fmt::Result {
    if eq(&quaternion, &()) {
        return write!(target, "{}", Num::ZERO);
    }


    if quaternion.r() != Num::ZERO {
        write!(target, "{}", quaternion.r())?;

        if quaternion.i() > Num::ZERO {
            write!(target, " + {}i", quaternion.i())?;
        } else if quaternion.i() < Num::ZERO {
            write!(target, " - {}i", -quaternion.i())?;
        } else { write!(target, "")?; }

        if quaternion.j() > Num::ZERO {
            write!(target, " + {}j", quaternion.j())?;
        } else if quaternion.j() < Num::ZERO {
            write!(target, " - {}j", -quaternion.j())?;
        } else { write!(target, "")?; }

        if quaternion.k() > Num::ZERO {
            write!(target, " + {}k", quaternion.k())?;
        } else if quaternion.k() < Num::ZERO {
            write!(target, " - {}k", -quaternion.k())?;
        } else { write!(target, "")?; }

        return Result::Ok(());
    }


    if quaternion.i() != Num::ZERO {
        write!(target, "{}i", quaternion.i())?;

        if quaternion.j() > Num::ZERO {
            write!(target, " + {}j", quaternion.j())?;
        } else if quaternion.j() < Num::ZERO {
            write!(target, " - {}j", -quaternion.j())?;
        } else { write!(target, "")?; }

        if quaternion.k() > Num::ZERO {
            write!(target, " + {}k", quaternion.k())?;
        } else if quaternion.k() < Num::ZERO {
            write!(target, " - {}k", -quaternion.k())?;
        } else { write!(target, "")?; }
        
        return Result::Ok(());
    }


    if quaternion.j() != Num::ZERO {
        write!(target, "{}j", quaternion.j())?;

        if quaternion.k() > Num::ZERO {
            write!(target, " + {}k", quaternion.k())?;
        } else if quaternion.k() < Num::ZERO {
            write!(target, " - {}k", -quaternion.k())?;
        } else { write!(target, "")?; }

        return Result::Ok(());
    }

    if quaternion.k() != Num::ZERO {
        write!(target, "{}k", quaternion.k())?;
    } else {
        write!(target, "{}", Num::ZERO)?;
    }

    Result::Ok(())
}

#[cfg(feature = "alloc")]
use crate::alloc::string::String;

#[cfg(feature = "alloc")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Turns a quaternion representation into a [String].
pub fn to_string<Num: Axis + crate::core::fmt::Display>(quaternion: impl Quaternion<Num>) -> Result<String, crate::core::fmt::Error> {
    let mut string = String::new();
    display(quaternion, &mut string)?;
    Result::Ok(string)
}
