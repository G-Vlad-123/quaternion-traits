
use super::*;

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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
    
    if (dot::<Num, Num>(unit, unit) - Num::ONE).abs() < Num::ERROR * Num::ERROR {
        return Option::Some(Out::from_unit_quat(unit))
    }

    Option::None
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Multiplies two unit quaternions in reversed order.
pub fn mul_reversed<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    mul(right, left)
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Divides a quaternion by another quaternion.
pub fn div<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    mul(left, inv::<Num, U<Num>>(right))
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
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

/// Calculates the dot product of two unit quaternions.
pub fn dot<Num, Out>(left: impl UnitQuaternion<Num>, right: impl UnitQuaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(left.r() * right.r() + left.i() * right.i() + left.j() * right.j() + left.k() * right.k())
}

/// Calculates the naturla logarithm of a unit quaternion.
#[cfg(feature = "math_fns")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn ln<Num, Out>(quaternion: impl UnitQuaternion<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let factor = quaternion.r().acos()
                    / Num::sqrt(
                        quaternion.i()*quaternion.i()
                      + quaternion.j()*quaternion.j()
                      + quaternion.k()*quaternion.k()
                    );
    Out::new_quat(
        Num::ZERO,
        quaternion.i() * factor, 
        quaternion.j() * factor, 
        quaternion.k() * factor,
    )
}

#[cfg(any(feature = "math_fns", feature = "trigonometry"))]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Raises the number e to a unit quaternion power.
/// 
/// `e ≈ 2.71828...`
/// 
/// Faster then running [`quat::exp`],
/// returns [`None`](Option::None) if `q.r()` is **not** near [`Num::ZERO`](Axis::ZERO).
/// 
/// If `q.r()` is not near use [`quat::exp`] as no optimizations can be made to it.
pub fn exp<Num, Out>(quaternion: impl UnitQuaternion<Num>) -> Option<Out>
where 
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    if quaternion.r().abs() < Num::ERROR {
        const SIN: f64 = 0.8414709848;
        const COS: f64 = 0.54030230586;
        let factor = Num::from_f64(SIN);
        Option::Some(
            new_unit(
                Num::from_f64(COS),
                quaternion.i() * factor,
                quaternion.j() * factor,
                quaternion.k() * factor,
            )
        )
    } else {
        Option::None
    }
}

/// Calculates the square root of a unit quaternion.
/// 
/// Faster then [`quat::sqrt`] if you know you have a unit quaternion.
#[cfg(feature = "math_fns")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn sqrt<Num, Out>(quaternion: impl UnitQuaternion<Num>) -> Out
where 
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    if quat::is_scalar(&quaternion) {
        use crate::core::cmp::Ordering::{Greater, Less};
        use crate::core::option::Option::Some;
        return match quaternion.r().partial_cmp(&Num::ZERO) {
            Some(Greater) => unsafe { Out::from_quat_unchecked((Num::ONE, ())) },
            Some(Less) => unsafe { Out::from_quat_unchecked([Num::ZERO, Num::ONE, Num::ZERO, Num::ZERO]) },
            _ => nan(),
        }
    }

    let r: Num = quaternion.r(); // alias
    let frac_1_sqrt_2 = Num::from_f64(crate::core::f64::consts::FRAC_1_SQRT_2);

    if r.abs() < Num::ERROR {
        return unsafe {
            Out::new_unit_quat_unchecked(
                frac_1_sqrt_2,
                quaternion.i() * frac_1_sqrt_2,
                quaternion.j() * frac_1_sqrt_2,
                quaternion.k() * frac_1_sqrt_2,
            )
        };
    }
    
    let factor = Num::sqrt(
        (Num::ONE - r) / (
            quaternion.i() * quaternion.i()
          + quaternion.j() * quaternion.j()
          + quaternion.k() * quaternion.k()
        )
    ) * frac_1_sqrt_2;

    new_unit(
        Num::sqrt(Num::ONE + r) * frac_1_sqrt_2,
        quaternion.i() * factor,
        quaternion.j() * factor,
        quaternion.k() * factor,
    )
}
