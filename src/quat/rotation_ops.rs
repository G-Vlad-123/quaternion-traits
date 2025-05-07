
use super::*;

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
        return from_axis_angle_unchecked(axis, Num::TAU / (Num::ONE + Num::ONE));
    }
    
    let quat: Q<Num> = (
        Num::ONE + dot,
        [
            from[1] * to[2] - from[2] * to[1],
            from[2] * to[0] - from[0] * to[2],
            from[0] * to[1] - from[1] * to[0],
        ],
    );
    unscale(quat, abs::<Num, Num>(quat)) // same as using `normalize` but skips the if check
}

/// Constructs a quaternion from a given axis vector and a given angle.
/// 
/// If the axis vector is a unit vector, then
/// the returned quaternion is a unit quaternion.
/// 
/// # Examples
/// Normalized
/// ```
/// use quaternion_traits::quat::from_axis_angle_unchecked;
/// # use core::f32::consts::{PI, SQRT_2};
/// 
/// let vector: [f32; 3] = [0.0, 1.0, 0.0];
/// let angle: f32 = PI / 2.0; // 90º
/// 
/// let quat: [f32; 4] = from_axis_angle_unchecked::<f32, _>(vector, angle);
/// 
/// assert_eq!(
///     quat,
///     [SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0, 0.0]
/// )
/// ```
/// 
/// Not normalized
/// ```
/// use quaternion_traits::quat::from_axis_angle_unchecked;
/// # use core::f32::consts::{PI, SQRT_2};
/// 
/// let vector: [f32; 3] = [0.0, 2.0, 0.0];
/// let angle: f32 = PI / 2.0; // 90º
/// 
/// let quat: [f32; 4] = from_axis_angle_unchecked::<f32, _>(vector, angle);
/// 
/// assert_eq!(
///     quat,
///     [SQRT_2 / 2.0, 0.0, SQRT_2, 0.0]
/// )
/// ```
pub fn from_axis_angle_unchecked<Num, Out>(axis: impl Vector<Num>, angle: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let (sin, cos) = (angle.scalar() / (Num::ONE + Num::ONE)).sin_cos();
    Out::new_quat(
        cos,
        axis.x() * sin,
        axis.y() * sin,
        axis.z() * sin,
    )
}

/// Constructs a quaternion from a given axis vector and a given angle.
/// 
/// If the axis vector is a unit vector, then
/// the returned quaternion is a unit quaternion.
/// 
/// # Examples
/// Normalized
/// ```
/// use quaternion_traits::quat::from_axis_angle_checked;
/// # use core::f32::consts::{PI, SQRT_2};
/// 
/// let vector: [f32; 3] = [0.0, 1.0, 0.0];
/// let angle: f32 = PI / 2.0; // 90º
/// 
/// let quat: Option<[f32; 4]> = from_axis_angle_checked::<f32, _>(vector, angle);
/// 
/// assert_eq!(
///     quat,
///     Some( [SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0, 0.0] )
/// )
/// ```
/// 
/// Not normalized
/// ```
/// use quaternion_traits::quat::from_axis_angle_checked;
/// # use core::f32::consts::{PI, SQRT_2};
/// 
/// let vector: [f32; 3] = [0.0, 2.0, 0.0];
/// let angle: f32 = PI / 2.0; // 90º
/// 
/// let quat: Option<[f32; 4]> = from_axis_angle_checked::<f32, _>(vector, angle);
/// 
/// assert_eq!(
///     quat,
///     None
/// )
/// ```
pub fn from_axis_angle_checked<Num, Out>(axis: impl Vector<Num>, angle: impl Scalar<Num>) -> Option<Out>
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if axis.x()*axis.x() + axis.y()*axis.y() + axis.z()*axis.z() - Num::ONE < Num::ERROR * Num::ERROR {
        Option::Some(from_axis_angle_unchecked(axis, angle))
    } else {
        Option::None
    }
}

/// Constructs a unit quaternion from a given axis vector and an angle.
/// 
/// Returns a unit quaternion no matter if the axis vector is normalized it'self or not.
/// 
/// # Example
/// Normalized
/// ```
/// use quaternion_traits::quat::from_axis_angle;
/// # use core::f32::consts::{PI, SQRT_2};
/// 
/// let vector: [f32; 3] = [0.0, 1.0, 0.0];
/// let angle: f32 = PI / 2.0; // 90º
/// 
/// let quat: [f32; 4] = from_axis_angle::<f32, _>(vector, angle);
/// 
/// assert_eq!(
///     quat,
///     [SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0, 0.0]
/// )
/// ```
/// 
/// Not normalized
/// ```
/// use quaternion_traits::quat::from_axis_angle;
/// # use core::f32::consts::{PI, SQRT_2};
/// 
/// let vector: [f32; 3] = [0.0, 2.0, 0.0];
/// let angle: f32 = PI / 2.0; // 90º
/// 
/// let quat: [f32; 4] = from_axis_angle::<f32, _>(vector, angle);
/// 
/// assert_eq!(
///     quat,
///     [SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0, 0.0]
/// )
/// ```
pub fn from_axis_angle<Num, Out>(axis: impl Vector<Num>, angle: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let (sin, cos) = (angle.scalar() / (Num::ONE + Num::ONE)).sin_cos();
    let scalar = sin / (axis.x()*axis.x() + axis.y()*axis.y() + axis.z()*axis.z()).sqrt();
    Out::new_quat(
        cos,
        axis.x() * scalar,
        axis.y() * scalar,
        axis.z() * scalar,
    )
}

/// Gets a quaternion's axis and angle.
/// 
/// Alike [`to_polar_form`] but ignores the absolute value of the quaternion.
pub fn to_axis_angle<Num, Vector, Scalar>(quaternion: impl Quaternion<Num>) -> (Vector, Scalar)
where 
    Num: Axis,
    Vector: crate::VectorConstructor<Num>,
    Scalar: crate::ScalarConstructor<Num>,
{
    if quaternion.i() == Num::ZERO || quaternion.j() == Num::ZERO || quaternion.k() == Num::ZERO {
        return (Vector::new_vector(Num::ZERO, Num::ZERO, Num::ZERO), Scalar::new_scalar(Num::ZERO));
    }
    let vec_abs = (quaternion.i()*quaternion.i() + quaternion.j()*quaternion.j() + quaternion.k()*quaternion.k()).sqrt();
    let vec_inv_abs = Num::ONE / vec_abs;
    let angle = (Num::ONE + Num::ONE) * vec_abs.min(Num::ONE).asin();
    (
        Vector::new_vector(quaternion.i() * vec_inv_abs, quaternion.j() * vec_inv_abs, quaternion.k() * vec_inv_abs),
        Scalar::new_scalar( if quaternion.r() >= Num::ZERO {angle} else {-angle} )
    )
}

// TODO check `rotate_from_to_shortest` from quaternion_core
// TODO check `point_rotation` from quaternion_core

/// Point rotation by a quaternion. (Frame fixed)
/// 
/// Given formula:
/// `q v *q` where `abs(q) == 1`
/// 
/// Normalizes the quaternion before operating.
/// 
/// If you want to go around this then use:
/// [`point_rotation_checked`] or [`point_rotation_unchecked`].
/// 
/// # Example
/// ```
/// # use core::f32::consts::PI;
/// use quaternion_traits::traits::Axis;
/// use quaternion_traits::quat::{point_rotation, mul, conj, from_axis_angle};
/// 
/// let vector: [f32; 3] = [1.0, 0.5, -8.0];
/// let quat: [f32; 4] = from_axis_angle::<f32, _>([0.2, 1.0, -2.0], PI);
/// 
/// let result: [f32; 3] = point_rotation::<f32, _>(quat, vector);
/// 
/// // Equivalent but more expensive
/// let check: [f32; 3] = mul::<f32, (f32, [f32; 3])>(
///     mul::<f32, [f32; 4]>(
///         quat,
///         (0.0, vector)
///     ),
///     conj::<f32, [f32; 4]>(quat),
/// ).1;
/// 
/// assert!( (result[0] - check[0]).abs() < <f32 as Axis>::ERROR );
/// assert!( (result[1] - check[1]).abs() < <f32 as Axis>::ERROR );
/// assert!( (result[2] - check[2]).abs() < <f32 as Axis>::ERROR );
/// ```
/// 
// Example gotten from https://docs.rs/quaternion-core/latest/quaternion_core/fn.point_rotation.html
pub fn point_rotation<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    point_rotation_unchecked(normalize::<Num, Q<Num>>(quaternion), vector)
}

/// Checked point rotation by a quaternion. (Frame fixed)
/// 
/// Checks if the quaternion is normalized before operating.
/// Returns [`None`](Option::None) if it's not normalized.
pub fn point_rotation_checked<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Option<Out>
where 
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    if is_normalized::<Num>(&quaternion) {
        Option::Some(point_rotation_unchecked(quaternion, vector))
    } else {
        Option::None
    }
}

/// Unchecked point rotation by a quaternion. (Frame fixed)
/// 
/// Performs the operation no matter what.
/// For non-normalized quaternions the output is
/// (determenistic) undefined behaviour.
pub fn point_rotation_unchecked<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    let temp: [Num; 3] = [
        quaternion.r() * vector.x() + quaternion.j() * vector.z() - quaternion.k() * vector.y(),
        quaternion.r() * vector.y() + quaternion.k() * vector.x() - quaternion.i() * vector.z(),
        quaternion.r() * vector.z() + quaternion.i() * vector.y() - quaternion.j() * vector.x(),
    ];
    let two = Num::ONE + Num::ONE;
    Out::new_vector(
        vector.x() + (quaternion.j() * temp[2] - quaternion.k() * temp[1]) * two,
        vector.y() + (quaternion.k() * temp[0] - quaternion.i() * temp[2]) * two,
        vector.z() + (quaternion.i() * temp[1] - quaternion.j() * temp[0]) * two,
    )
}

/// Frame rotation by a quaternion. (Point fixed)
/// 
/// Given formula:
/// `*q v q` where `abs(q) == 1`
/// 
/// Normalizes the quaternion before operating.
/// 
/// If you want to go around this then use:
/// [`frame_rotation_checked`] or [`frame_rotation_unchecked`].
/// 
/// # Example
/// ```
/// # use core::f32::consts::PI;
/// use quaternion_traits::{quat::{mul, conj, from_axis_angle}, traits::Axis};
/// use quaternion_traits::quat::point_rotation;
/// 
/// let vector: [f32; 3] = [1.0, 0.5, -8.0];
/// let quat: [f32; 4] = from_axis_angle::<f32, _>([0.2, 1.0, -2.0], PI);
/// 
/// let result: [f32; 3] = point_rotation::<f32, _>(quat, vector);
/// 
/// // Equivalent but more expensive
/// let check: [f32; 3] = mul::<f32, (f32, [f32; 3])>(
///     mul::<f32, [f32; 4]>(
///         conj::<f32, [f32; 4]>(quat),
///         (0.0, vector)
///     ),
///     quat,
/// ).1;
/// 
/// assert!( (result[0] - check[0]).abs() < <f32 as Axis>::ERROR );
/// assert!( (result[1] - check[1]).abs() < <f32 as Axis>::ERROR );
/// assert!( (result[2] - check[2]).abs() < <f32 as Axis>::ERROR );
/// ```
/// 
// Example gotten from https://docs.rs/quaternion-core/latest/quaternion_core/fn.frame_rotation.html
pub fn frame_rotation<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    frame_rotation_unchecked(normalize::<Num, Q<Num>>(quaternion), vector)
}

/// Checked frame rotation by a quaternion. (Point fixed)
/// 
/// Checks if the quaternion is normalized before operating.
/// Returns [`None`](Option::None) if it's not normalized.
pub fn frame_rotation_checked<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Option<Out>
where 
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    if is_normalized(&quaternion) {
        Option::Some(frame_rotation_unchecked(quaternion, vector))
    } else {
        Option::None
    }
}

/// Checked frame rotation by a quaternion. (Point fixed)
/// 
/// Performs the operation no matter what.
/// For non-normalized quaternions the output is
/// (determenistic) undefined behaviour.
pub fn frame_rotation_unchecked<Num, Out>(quaternion: impl Quaternion<Num>, vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    let temp: [Num; 3] = [
        vector.x() * quaternion.r() + vector.y() * quaternion.k() - vector.z() * quaternion.j(),
        vector.y() * quaternion.r() + vector.z() * quaternion.i() - vector.x() * quaternion.k(),
        vector.z() * quaternion.r() + vector.x() * quaternion.j() - vector.y() * quaternion.i(),
    ];
    let two = Num::ONE + Num::ONE;
    Out::new_vector(
        vector.x() + (temp[1] * quaternion.k() - temp[2] * quaternion.j()) * two,
        vector.y() + (temp[2] * quaternion.i() - temp[0] * quaternion.k()) * two,
        vector.z() + (temp[0] * quaternion.j() - temp[1] * quaternion.i()) * two,
    )
}
