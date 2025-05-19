
use super::*;

#[inline]
/// Adds two unit quaternions.
/// 
/// # Safety
/// The output must be a unit quaternion.
pub unsafe fn add_unchecked<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    new_unit(
        left.r() + right.r(),
        left.i() + right.i(),
        left.j() + right.j(),
        left.k() + right.k(),
    )
}

#[inline]
/// Adds two unit quaternions.
pub unsafe fn add_checked<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Option<Out>
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    let unit: U<Num> = new_unit(
        left.r() + right.r(),
        left.i() + right.i(),
        left.j() + right.j(),
        left.k() + right.k(),
    );
    
    if dot::<Num, Num>(unit, unit) - Num::ONE < Num::ERROR * Num::ERROR {
        return Option::Some(Out::from_unit_quat(unit))
    }

    Option::None
}

#[inline]
/// Adds two unit quaternions.
pub fn add_any<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
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
/// Multiplies two unit quaternions.
pub fn mul<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    new_unit(
        left.r() * right.r() - left.i() * right.i() - left.j() * right.j() - left.k() * right.k(),
        left.r() * right.i() + left.i() * right.r() + left.j() * right.k() - left.k() * right.j(),
        left.r() * right.j() - left.i() * right.k() + left.j() * right.r() + left.k() * right.i(),
        left.r() * right.k() + left.i() * right.j() - left.j() * right.i() + left.k() * right.r(),
    )
}

#[inline]
/// Multiplies two unit quaternions in reversed order.
pub fn mul_reversed<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    mul(right, left)
}

#[inline]
/// Divides a quaternion by another quaternion.
pub fn div<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    mul(left, inv::<Num, U<Num>>(right))
}

#[inline]
/// Gets the negative of this unit quaternion.
pub fn neg<Num, Out>(quaternion: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    new_unit(
        -quaternion.r(),
        -quaternion.i(),
        -quaternion.j(),
        -quaternion.k(),
    )
}

#[inline]
/// Gets the conjugate of this unit quaternion.
pub fn conj<Num, Out>(quaternion: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    new_unit(
         quaternion.r(),
        -quaternion.i(),
        -quaternion.j(),
        -quaternion.k(),
    )
}

#[inline]
/// Gets the inverse of this unit quaternion.
pub fn inv<Num, Out>(quaternion: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    // unit quaternions their conjugate and inverse are equal
    new_unit(
         quaternion.r(),
        -quaternion.i(),
        -quaternion.j(),
        -quaternion.k(),
    )
}

#[inline]
#[cfg(feature = "math_fns")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Spherical liniar interpolation for unit quaternions.
/// 
/// Uses the shortest path inbetween the two unit
/// quaternions, returning a unit quaternion.
/// 
/// If the two given quaternions are unit quaternions
pub fn slerp<Num, Out>(from: impl UnitQuaternion<Num>, to: impl UnitQuaternion<Num>, at: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    let mut dot = dot::<Num, Num>(&from, &to);

    let to: U<Num> = if dot < Num::ZERO {
        dot = -dot;
        neg(to)
    } else {
        <U<Num>>::from_unit_quat(to)
    };

    if dot > Num::ONE - Num::ERROR { // for ERROR = 0.0005 => Aprox. Err < 0.017%
        return unsafe {
            add_unchecked(
                scale::<Num, U<Num>>(add_unchecked::<Num, U<Num>>(to, &from), at),
                from
            )
        };
    }

    let angle = dot.acos();
    let transition_angle = at.scalar() * angle;

    let sin_1 = (angle - transition_angle).sin();
    let sin_2 = transition_angle.sin();

    let coeficient = Num::ONE / (Num::ONE - dot*dot).sqrt();

    unsafe {
        add_unchecked(
            scale::<Num, U<Num>>(from, coeficient * sin_1),
            scale::<Num, U<Num>>(to, coeficient * sin_2),
        )
    }
}

/// Scales a unit quaternion and returns a unit quaternion.
/// 
/// # Safety
/// Eather the output is a unit quaternion or it's
/// used in an operation that returns a unit quaternion.
pub unsafe fn scale<Num, Out>(quaternion: impl UnitQuaternion<Num>, scalar: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    new_unit(
        quaternion.r() * scalar.scalar(),
        quaternion.i() * scalar.scalar(),
        quaternion.j() * scalar.scalar(),
        quaternion.k() * scalar.scalar(),
    )
}

/// Calculates teh dot product of two unit quaternions.
pub fn dot<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(left.r() * right.r() + left.i() * right.i() + left.j() * right.j() + left.k() * right.k())
}
