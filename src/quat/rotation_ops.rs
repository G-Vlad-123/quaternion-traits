
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
        unsafe {
            return axis_angle_unchecked(axis, Num::TAU / (Num::ONE + Num::ONE));
        }
    }

    let quat: Q<Num> = (
        Num::ONE + dot,
        [
            from[1] * to[2] - from[2] * to[1],
            from[2] * to[0] - from[0] * to[2],
            from[0] * to[1] - from[1] * to[0],
        ],
    );
    scale(&quat, Num::ONE / abs(quat))
}

/// Constructs a unit quaternion from a given axis unit vector and a given angle.
/// 
/// Returns [`None`](Option::None) if axis is not a unit vector.
pub fn axis_angle<Num, Out>(axis: impl Vector<Num>, angle: impl Scalar<Num>) -> crate::core::option::Option<Out>
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

/// Constructs a unit quaternion from a given axis unit vector and a given angle.
/// 
/// # Safety
/// Axis must be a unit vector.
pub unsafe fn axis_angle_unchecked<Num, Out>(axis: impl Vector<Num>, angle: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let half_angle = angle.scalar() / (Num::ONE + Num::ONE);
    let half_angle_sin = half_angle.sin();
    Out::new_quat(
        half_angle.cos(),
        axis.x() * half_angle_sin,
        axis.y() * half_angle_sin,
        axis.z() * half_angle_sin,
    )
}
