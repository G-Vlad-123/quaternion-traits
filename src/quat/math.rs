
use super::*;

/// Adds two quaternions.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::add;
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [4.0, 3.0, 2.0, -4.0];
/// let result: [f32; 4] = add::<f32, _>(a, b);
/// 
/// assert_eq!( result, [5.0, 5.0, 5.0, 0.0] );
/// ```
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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

/// Adds a scalar value to a quaternion
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::add_scalar;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let result: [f32; 4] = add_scalar::<f32, _>(quat, 3.25);
/// 
/// assert_eq!( result, [4.25, 2.0, 3.0, 4.0] );
/// ```
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn add_scalar<Num, Out>(quaternion: impl Quaternion<Num>, scalar: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r() + scalar.scalar(), 
        quaternion.i(), 
        quaternion.j(), 
        quaternion.k(),
    )
}

/// Adds a scalar value to a quaternion
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::add_complex;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let result: [f32; 4] = add_complex::<f32, _>(quat, (3.5, 2.5));
/// 
/// assert_eq!( result, [4.5, 4.5, 3.0, 4.0] );
/// ```
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn add_complex<Num, Out>(quaternion: impl Quaternion<Num>, complex: impl Complex<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r() + complex.real(), 
        quaternion.i() + complex.imaginary(), 
        quaternion.j(), 
        quaternion.k(),
    )
}

/// Adds a scalar value to a quaternion
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::add_vector;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let result: [f32; 4] = add_vector::<f32, _>(quat, (3.0, 2.0, 1.0));
/// 
/// assert_eq!( result, [1.0, 5.0, 5.0, 5.0] );
/// ```
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn add_vector<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r(),
        quaternion.i() + vector.x(),
        quaternion.j() + vector.y(),
        quaternion.k() + vector.z(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Subtracts a quaternion from another one.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::sub;
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [4.0, 3.0, 2.0, -4.0];
/// let result: [f32; 4] = sub::<f32, _>(a, b);
/// 
/// assert_eq!( result, [-3.0, -1.0, 1.0, 8.0] );
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
/// Subtracts a scalar value from a quaternion.
/// 
/// # Example
/// ```rust
/// use quaternion_traits::quat::sub_scalar;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let result: [f32; 4] = sub_scalar::<f32, _>(quat, 2.0);
/// 
/// assert_eq!( result, [-1.0, 2.0, 3.0, 4.0] );
/// ```
pub fn sub_scalar<Num, Out>(quaternion: impl Quaternion<Num>, scalar: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r() - scalar.scalar(), 
        quaternion.i(), 
        quaternion.j(), 
        quaternion.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Subtracts a complex number from a quaternion.
/// 
/// # Example
/// ```rust
/// use quaternion_traits::quat::sub_complex;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let result: [f32; 4] = sub_complex::<f32, _>(quat, (2.0, -2.0));
/// 
/// assert_eq!( result, [-1.0, 4.0, 3.0, 4.0] );
/// ```
pub fn sub_complex<Num, Out>(quaternion: impl Quaternion<Num>, complex: impl Complex<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r() - complex.real(), 
        quaternion.i() - complex.imaginary(), 
        quaternion.j(), 
        quaternion.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Subtracts a vector from a quaternion.
/// 
/// # Example
/// ```rust
/// use quaternion_traits::quat::sub_vector;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let result: [f32; 4] = sub_vector::<f32, _>(quat, (1.5, 2.5, 3.5));
/// 
/// assert_eq!( result, [1.0, 0.5, 0.5, 0.5] );
/// ```
pub fn sub_vector<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r(), 
        quaternion.i() - vector.x(), 
        quaternion.j() - vector.y(), 
        quaternion.k() - vector.z(),
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
/// 
/// Multipliing by a unit quaternion is equivalent to rotating
/// by a specified angle in a specified direction.
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

/// Multiplies a quaternion with a complex number.
/// 
/// # Example
/// ```
/// # use quaternion_traits::quat::mul;
/// use quaternion_traits::quat::mul_complex;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let complex: (f32, f32) = (2.0, 2.0);
/// 
/// assert_eq!(
///     mul_complex::<f32, [f32; 4]>(quat, complex),
///     mul::<f32, [f32; 4]>(quat, (complex, (), ()))
/// );
/// ```
pub fn mul_complex<Num, Out>(left: impl Quaternion<Num>, right: impl Complex<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        left.r() * right.real() - left.i() * right.imaginary(),
        left.i() * right.real() + left.r() * right.imaginary(),
        left.j() * right.real() + left.k() * right.imaginary(),
        left.k() * right.real() - left.j() * right.imaginary(),
    )
}

/// Multiplies a quaternion with a vector.
/// 
/// # Example
/// ```
/// # use quaternion_traits::quat::mul;
/// use quaternion_traits::quat::mul_vector;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let vector: [f32; 3] = [2.0, 3.0, 1.0];
/// 
/// assert_eq!(
///     mul_vector::<f32, [f32; 4]>(quat, vector),
///     mul::<f32, [f32; 4]>(quat, ((), vector))
/// );
/// ```
pub fn mul_vector<Num, Out>(left: impl Quaternion<Num>, right: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
      -(left.i() * right.x() + left.j() * right.y() + left.k() * right.z()),
        left.r() * right.x() + left.j() * right.z() - left.k() * right.y(),
        left.r() * right.y() - left.i() * right.z() + left.k() * right.x(),
        left.r() * right.z() + left.i() * right.y() - left.j() * right.x(),
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
/// use quaternion_traits::quat::{mul, mul_reversed};
/// 
/// let a: [f32; 4] = // quaternion
/// # [1.0, 2.0, 0.0, 3.0];
/// let b: [f32; 4] = // quaternion
/// # [3.0, 1.0, 4.0, 0.0];
/// 
/// assert_eq!( mul::<f32, [f32; 4]>(&a, &b), mul_reversed::<f32, [f32; 4]>(&b, &a) );
/// ```
pub fn mul_reversed<Num, Out>(right: impl Quaternion<Num>, left: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{ mul(left, right) }

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Multiplies a complex number with a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::{mul, mul_complex_reversed};
/// 
/// let quat: [f32; 4] = // quaternion
/// # [1.0, 2.0, 0.0, 3.0];
/// let complex: (f32, f32) = // complex number
/// # (3.0, 1.0);
/// 
/// assert_eq!(
///     mul::<f32, [f32; 4]>((complex, (), ()), quat),
///     mul_complex_reversed::<f32, [f32; 4]>(quat, complex)
/// );
/// ```
pub fn mul_complex_reversed<Num, Out>(right: impl Quaternion<Num>, left: impl Complex<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        left.real() * right.r() - left.imaginary() * right.i(),
        left.real() * right.i() + left.imaginary() * right.r(),
        left.real() * right.j() - left.imaginary() * right.k(),
        left.real() * right.k() + left.imaginary() * right.j(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Multiplies a vector with a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::{mul, mul_vector_reversed};
/// 
/// let quat: [f32; 4] = // quaternion
/// # [1.0, 2.0, 0.0, 3.0];
/// let vector: [f32; 3] = // vector
/// # [3.0, 1.0, 4.0];
/// 
/// assert_eq!(
///     mul::<f32, [f32; 4]>(((), vector), quat),
///     mul_vector_reversed::<f32, [f32; 4]>(quat, vector)
/// );
/// ```
pub fn mul_vector_reversed<Num, Out>(right: impl Quaternion<Num>, left: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
       -left.x() * right.i() - left.y() * right.j() - left.z() * right.k(),
        left.x() * right.r() + left.y() * right.k() - left.z() * right.j(),
       -left.x() * right.k() + left.y() * right.r() + left.z() * right.i(),
        left.x() * right.j() - left.y() * right.i() + left.z() * right.r(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Divides a quaternion by another one.
/// 
/// Is equivalent to multiplying a quaternion with
/// another one's inverse.
/// 
/// ```
/// use quaternion_traits::quat::{div, mul, inv};
/// 
/// let a: [f32; 4] = [1.0, 2.0, 0.0, 3.0];
/// let b: [f32; 4] = [3.0, 1.0, 4.0, 0.0];
/// 
/// assert_eq!(
///     mul::<f32, [f32; 4]>(a, inv::<f32, [f32; 4]>(b)),
///     div::<f32, [f32; 4]>(a, b)
/// );
/// ```
pub fn div<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    mul::<Num, Out>(left, inv::<Num, Q<Num>>(right))
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
    mul::<Num, Out>(inv::<Num, Q<Num>>(left), &right)
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the negative of this quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::neg;
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
/// use quaternion_traits::quat::conj;
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
/// Scales a quaternion.
/// 
/// Equivalent to multiplying a quaternion by a scalar quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::scale;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, 2.0, 3.0];
/// let scaled: [f32; 4] = scale::<f32, [f32; 4]>(quat, 2);
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
/// use quaternion_traits::quat::unscale;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, 2.0, 3.0];
/// let unscaled: [f32; 4] = unscale::<f32, [f32; 4]>(quat, 2);
/// 
/// assert_eq!( unscaled, [0.0, 0.5, 1.0, 1.5] );
/// ```
pub fn unscale<Num, Out>(quaternion: impl Quaternion<Num>, scalar: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let scalar: Num = Num::ONE / scalar.scalar();
    Out::new_quat(
        quaternion.r() * scalar,
        quaternion.i() * scalar,
        quaternion.j() * scalar,
        quaternion.k() * scalar,
    )
}

/// Liniar interpolation for quaternions.
/// 
/// Uses the shortest path inbetween the two quaternions.
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn lerp<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>, at: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    add(
        if dot::<Num, Num>(&from, &to) < Num::ZERO {
            scale::<Num, Q<Num>>(add::<Num, Q<Num>>(to, &from), at)
        } else {
            scale::<Num, Q<Num>>(sub::<Num, Q<Num>>(to, &from), at)
        },
        from,
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Spherical liniar interpolation for unit quaternions.
/// 
/// Uses the shortest path inbetween the two unit
/// quaternions, returning a unit quaternion.
/// 
/// If the two given quaternions are unit quaternions
pub fn slerp_checked<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>, at: impl Scalar<Num>) -> Option<Out>
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if (abs_squared::<Num, Num>(&from) - Num::ONE).abs() < Num::ERROR * Num::ERROR
    && (abs_squared::<Num, Num>( &to ) - Num::ONE).abs() < Num::ERROR * Num::ERROR
    { Option::Some(slerp_unchecked(from, to, at)) }
    else { Option::None }
}

/// Spherical liniar interpolation for unit quaternions.
/// 
/// Uses the shortest path inbetween the two unit
/// quaternions, returning a unit quaternion.
/// 
/// The two quaternions must be unit quaternions (have an absolite value of [`Num::ONE`](Axis::ONE)).
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn slerp_unchecked<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>, at: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut dot = dot::<Num, Num>(&from, &to);

    let to: Q<Num> = if dot < Num::ZERO {
        dot = -dot;
        neg(to)
    } else {
        convert_quat(to)
    };

    if dot > Num::ONE - Num::ERROR { // for ERROR = 0.0005 => Aprox. Err < 0.017%
        return add(
            scale::<Num, Q<Num>>(add::<Num, Q<Num>>(to, &from), at),
            from
        );
    }

    let angle = dot.acos();
    let transition_angle = at.scalar() * angle;

    let sin_1 = (angle - transition_angle).sin();
    let sin_2 = transition_angle.sin();

    let coeficient = Num::ONE / (Num::ONE - dot*dot).sqrt();

    add(
        scale::<Num, Q<Num>>(from, coeficient * sin_1),
        scale::<Num, Q<Num>>(to, coeficient * sin_2),
    )
}

/// Gets the distance inbetween the coordenates of two quaternions.
/// 
/// Equivalent to getting the absolute value of 
/// 
/// ```
/// use quaternion_traits::quat::dist_euclid;
/// 
/// let a: [f32; 4] = [5.0, 0.0, 1.0, 3.0];
/// let b: [f32; 4] = [2.0, 0.0, 5.0, 3.0];
/// 
/// assert_eq!( dist_euclid::<f32, f32>(a, b), 5.0 );
/// assert_eq!( dist_euclid::<f32, f32>(a, a), 0.0 );
/// ```
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn dist_euclid<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    abs(&sub::<Num, Q<Num>>(from, to))
}

/// Calculates the cosine distance between two quaternions.
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn dist_cosine<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(Num::ONE - angle_between_cos(from, to))
}

/// Calculates the angle between two quaternions.
/// 
/// This does NOT use the [`angle`] function, and the two give diferent results.
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn angle_between<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar( ( dot::<Num, Num>(&from, &to) / (abs_squared::<Num, Num>(from) * abs_squared(to)).sqrt() ).acos() )
}

/// Calculates the cosine of the angle between two quaternions.
/// 
/// This does NOT use the [`angle`] function, and the two give diferent results.
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn angle_between_cos<Num, Out>(from: impl Quaternion<Num>, to: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(dot::<Num, Num>(&from, &to) / (abs_squared::<Num, Num>(from) * abs_squared(to)).sqrt())
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Normalizes a quaternion.
/// 
/// The normal of a quaternion always has the same "direction"
/// but with an absolute value of [`Num::ONE`](Axis::ONE).
/// 
/// If the quaternion is the origin it returns the origin.
/// 
/// If you know you have a non-zero quaternion then you
/// can skip the if check by using `unscale(q, abs::<Num, Num>(q))` or `scale(q, Num::ONE / abs(q))`
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::normalize;
/// 
/// let quat: [f32; 4] = [0.0, 3.25, 0.0, 0.0];
/// let normal: [f32; 4] = normalize::<f32, [f32; 4]>(&quat);
/// 
/// assert_eq!( normal, [0.0, 1.0, 0.0, 0.0] );
/// ```
pub fn normalize<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if eq(&quaternion, ()) { return origin() }
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
/// use quaternion_traits::quat::abs;
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
/// Calculates the squared absolute value of the quaternion multiplied by [`Num::ERROR`](Axis::ERROR) to the -2 power.
/// Then it takes the square root.
/// The it multiplies by [`Num::ERROR`](Axis::ERROR) to the first power.
/// 
/// Note: The operations above are the rough order in which they are done.
/// 
/// # Example
/// Floating point aproximation causes an error :(
/// ```should_panic
/// use quaternion_traits::quat::abs;
/// 
/// let smol: [f32; 4] = [
///     1e-21,
///     3e-21,
///     9e-21,
///     3e-21,
/// ];
/// 
/// assert_eq!( abs::<f32, f32>(smol), 1e-20 );
/// ```
/// 
/// It works!
/// ```
/// use quaternion_traits::quat::abs_small;
/// 
/// let smol: [f32; 4] = [
///     1e-21,
///     3e-21,
///     9e-21,
///     3e-21,
/// ];
/// 
/// assert_eq!( abs_small::<f32, f32>(smol), 1e-20 );
/// ```
/// 
/// The aproximation catches us again :(
/// ```should_panic
/// use quaternion_traits::quat::abs_small;
/// 
/// let smol: [f32; 4] = [
///     1e-31,
///     3e-31,
///     9e-31,
///     3e-31,
/// ];
/// 
/// assert_eq!( abs_small::<f32, f32>(smol), 1e-30 );
/// ```
pub fn abs_small<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
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
/// use quaternion_traits::quat::abs_squared;
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
/// 
/// Note: This isn't named arg because it does not represent the
/// argument of the quaternion. 
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
/// 
/// Note: This isn't named arg_cos because it does not use the
/// argument of the quaternion.
pub fn angle_cos<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(quaternion.r() / abs::<Num, Num>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the inverse quaternion of a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::{inv, mul, identity, is_near};
/// 
/// let quat: [f32; 4] = [1.0, 3.0, 9.0, 3.0];
/// let inv_quat: [f32; 4] = inv::<f32, [f32; 4]>(&quat);
/// 
/// assert!( is_near::<f32>(
///     mul::<f32, [f32; 4]>(quat, inv_quat),
///     identity::<f32, [f32; 4]>()
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
/// use quaternion_traits::quat::{ln, exp, is_near};
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let ln_quat: [f32; 4] = ln::<f32, [f32; 4]>(quat);
/// 
/// assert!( is_near::<f32>(exp::<f32, [f32; 4]>(ln_quat), quat) );
/// ```
/// The function [`is_near`] is used here because of finite floating point precision.
pub fn ln<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let absolute: Num = abs(&quaternion);
    add(
        scale::<Num, Q<Num>>(
            normalize::<Num, Q<Num>>(
                vector_part::<Num, Q<Num>>(&quaternion),
            ),
            (quaternion.r() / absolute).acos()
        ), 
        (absolute.ln(), ())
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Raises the number e to a quaternion power.
/// 
/// e ≈ 2.71828...
/// 
/// ```
/// use quaternion_traits::quat::{exp, ln, is_near};
/// 
/// let quat: [f32; 4] = [1.0, 3.14, 0.0, 0.0];
/// let exp_quat: [f32; 4] = exp::<f32, [f32; 4]>(quat);
/// 
/// assert!( is_near::<f32>(ln::<f32, [f32; 4]>(exp_quat), quat) );
/// ```
/// The function [`is_near`] is used here because of finite floating point precision.
pub fn exp<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let vec: Q<Num> = vector_part(&quaternion);
    let (sin, cos) = abs::<Num, Num>(&vec).sin_cos();
    scale::<Num, Out>(
        add::<Num, Q<Num>>(
            scale::<Num, Q<Num>>(
                normalize::<Num, Q<Num>>(&vec),
                sin
            ),
            (cos, ())
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
/// use quaternion_traits::quat::{log, pow_i};
/// 
/// let base: [f32; 4] = [0.0, 2.0, 1.0, 0.0];
/// let quat: [f32; 4] = pow_i::<f32, [f32; 4]>(base, 3);
/// let log_quat: [f32; 4] = log::<f32, [f32; 4]>(base, quat);
/// 
/// assert_eq!( log_quat, [3.0, 0.0, 0.0, 0.0] );
/// ```
pub fn log<Num, Out>(base: impl Quaternion<Num>, num: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    div::<Num, Out>(&ln::<Num, Q<Num>>(num), &ln::<Num, Q<Num>>(base))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the square root of a quaternion.
/// 
/// This uses a diferent algorthm from [`pow_f`].
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::{sqrt, mul, is_near};
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// 
/// let sqrt: [f32; 4] = sqrt::<f32, _>(quat);
/// 
/// assert!( is_near::<f32>(
///     mul::<f32, [f32; 4]>(sqrt, sqrt),
///     quat
/// ) );
/// ```
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
    let unit = normalize::<Num, Q<Num>>(vector_part::<Num, Q<Num>>(&quaternion)).1;
    let abs: Num = abs::<Num, Num>(&quaternion);
    let unreal_part: Num = Num::sqrt( (abs - r) / (Num::ONE + Num::ONE) );
    Out::new_quat (
        Num::sqrt( (abs + r) / (Num::ONE + Num::ONE) ),
        unit[0] * unreal_part,
        unit[1] * unreal_part,
        unit[2] * unreal_part,
    )
}

/// Calculares the square of a quaternion.
/// 
/// Equivalent to `mul(q, q)`
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::{square, mul};
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// 
/// assert_eq!(
///     square::<f32, [f32; 4]>(quat),
///     mul::<f32, [f32; 4]>(quat, quat)
/// );
/// ```
#[inline]
pub fn square<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        quaternion.r() * quaternion.r() - quaternion.i() * quaternion.i() - quaternion.j() * quaternion.j() - quaternion.k() * quaternion.k(),
        quaternion.r() * quaternion.i() + quaternion.i() * quaternion.r() + quaternion.j() * quaternion.k() - quaternion.k() * quaternion.j(),
        quaternion.r() * quaternion.j() - quaternion.i() * quaternion.k() + quaternion.j() * quaternion.r() + quaternion.k() * quaternion.i(),
        quaternion.r() * quaternion.k() + quaternion.i() * quaternion.j() - quaternion.j() * quaternion.i() + quaternion.k() * quaternion.r(),
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
    if eq(&base, &identity::<Num, Q<Num>>()) { return identity() }
    if exp == 0 { return identity(); }
    let mut out: Q<Num> = identity::<Num, Q<Num>>();
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
    if eq(&base, ()) { return origin(); }
    if eq(&base, identity::<Num, Q<Num>>()) { return identity() }
    if exp == 0 { return identity(); }
    let mut out = identity::<Num, Q<Num>>();
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
        crate::quat::exp::<Num, Q<Num>>(
            scale::<Num, Q<Num>>(
                vector_part::<Num, Q<Num>>(base),
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
    crate::quat::exp(&mul::<Num, Q<Num>>(&ln::<Num, Q<Num>>(base), &exp))
}

fn γ<Num: Axis>() -> Num {
    let γ_limit = {
        (Num::ONE + Num::ONE)
        * (Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE)
        * (Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE)
        * (Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE + Num::ONE)
    };
    let mut at = Num::ZERO;
    let mut result = -γ_limit * γ_limit.ln();
    for _ in 1..2000 {
        at = at + Num::ONE;
        result = result + Num::ONE / at;
    }
    result
}

const LNGAMMA_REPEATS: u16 = 2000;
/// Calculates the natural logarithm of the gamma function with a quaternion input.
/// 
/// Equivalent to `ln(gamma(q))` (assuming infinite precision + infinite loops).
pub fn lngamma<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    // refrence: https://mathworld.wolfram.com/LogGammaFunction.html (or search it on wiki)
    // Since the only multiplication involved is divison inbetween a
    // quaternion and a real and teh rest are additions/subtractions which are commutative
    // and since all functions involved are included
    // I assumed that this funtion can be expanded to the quaternion set without change.
    let mut result: Q<Num> = sub(scale::<Num, Q<Num>>(&quaternion, -γ::<Num>()), ln::<Num, Q<Num>>(&quaternion));
    let mut at: Num = Num::ZERO;
    let mut fraction: Q<Num>;
    for _ in 0..LNGAMMA_REPEATS {
        at = at + Num::ONE;
        fraction = unscale(&quaternion, at);
        result = add(
            result,
            sub::<Num, Q<Num>>(
                fraction,
                ln::<Num, Q<Num>>(
                    add::<Num, Q<Num>>(fraction, (Num::ONE, ()))
                ),
            )
        )
    }
    Out::from_quat(result)
}

/// Calculates the gamma function applies to a quaternion.
/// 
/// The gamma of a number is the factorial of sed number - 1.
/// 
/// # Note
/// This function uses [`lngamma`] to calculate it's value,
/// if you need the naturla logarigthm of the gamma function
/// use that function directly.
pub fn gamma<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    // TODO use specialized formula
    exp(lngamma::<Num, Q<Num>>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the dot product of two quaternions.
/// 
/// Fun fact: the dot product of a quaternion with it'self returns the squared absolute value :)
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::dot;
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

/// Gives the hadamard product of two quaternions.
/// 
/// It uses the following formula:
/// 
///     # "
///     let q1 = w1 + x1*i + y1*j + z1*k
///         q2 = w2 + x2*i + y2*j + z2*k
/// 
///       =>
/// 
///     q1 ∘ q2 = w1 * w2
///             + x1 * x2 * i
///             + y1 * y2 * j
///             + z1 * z2 * k
///     # ";
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::hadamard;
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [5.0, 4.0, 3.0, 2.0];
/// let result: [f32; 4] = hadamard::<f32, _>(a, b);
/// 
/// assert_eq!(
///     result,
///     [5.0, 8.0, 9.0, 8.0]
/// )
/// ```
pub fn hadamard<Num, Out>(left: impl Quaternion<Num>, right: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        left.r() * right.r(),
        left.i() * right.i(),
        left.j() * right.j(),
        left.k() * right.k(),
    )
}
