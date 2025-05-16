
use super::*;

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
    let exp = exp::<Num, Q<Num>>(quaternion);
    unscale(sub::<Num, Q<Num>>(&exp, inv::<Num, Q<Num>>(&exp)), Num::ONE + Num::ONE)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the secant of a quaternion.
pub fn sec<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    inv(&cos::<Num, Q<Num>>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the secant of a quaternion.
pub fn sech<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    inv(&cosh::<Num, Q<Num>>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the  cosinus of a quaternion.
pub fn cos<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    // refrence: https://math.stackexchange.com/questions/1499095/how-to-calculate-sin-cos-tan-of-a-quaternion
    // If you find a paper on this please add it here (or modify the code + add it here if it uses a diferent equasion)
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
    let exp = exp::<Num, Q<Num>>(quaternion);
    unscale(&add::<Num, Q<Num>>(&exp, &inv::<Num, Q<Num>>(&exp)), Num::ONE + Num::ONE)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the cosecant of a quaternion.
pub fn csc<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    inv(&sin::<Num, Q<Num>>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the secant of a quaternion.
pub fn csch<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    inv(&sinh::<Num, Q<Num>>(quaternion))
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
    let (sin, cos) = sin_cos::<Num, Q<Num>>(quaternion);
    div(&sin, &cos)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the sinus of a quaternion.
pub fn tanh<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let exp = exp::<Num, Q<Num>>(quaternion);
    let inv = inv::<Num, Q<Num>>(&exp);
    div(
        &sub::<Num, Q<Num>>(&exp, &inv),
        &add::<Num, Q<Num>>(&exp, &inv),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the cotangent of a quaternion
pub fn cot<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let (sin, cos) = sin_cos::<Num, Q<Num>>(quaternion);
    div(&cos, &sin)
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates the sinus of a quaternion.
pub fn coth<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let exp = exp::<Num, Q<Num>>(quaternion);
    let inv = inv::<Num, Q<Num>>(&exp);
    div(
        &add::<Num, Q<Num>>(&exp, &inv),
        &sub::<Num, Q<Num>>(&exp, &inv),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the arcsinus of a quaternion.
pub fn asin<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    mul(
        (-Num::ONE, ()),
        ln::<Num, Q<Num>>(add::<Num, Q<Num>>(
            mul::<Num, Q<Num>>(unit_i::<Num, Q<Num>>(), &quaternion),
            sqrt::<Num, Q<Num>>(sub::<Num, Q<Num>>(
                identity::<Num, Q<Num>>(),
                square::<Num, Q<Num>>(&quaternion)
            )),
        )),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the arccosinus of a quaternion.
pub fn acos<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    mul(
        (-Num::ONE, ()),
        ln::<Num, Q<Num>>(add::<Num, Q<Num>>(
            &quaternion,
            mul::<Num, Q<Num>>(
                unit_i::<Num, Q<Num>>(),
                sqrt::<Num, Q<Num>>(sub::<Num, Q<Num>>(
                    identity::<Num, Q<Num>>(),
                    square::<Num, Q<Num>>(&quaternion)
                )),
            ),
        )),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the arctangent of a quaternion.
pub fn atan<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    mul(
        (-Num::ONE / (Num::ONE + Num::ONE), ()),
        ln::<Num, Q<Num>>(div::<Num, Q<Num>>(
            sub::<Num, Q<Num>>(unit_i::<Num, Q<Num>>(), &quaternion),
            add::<Num, Q<Num>>(unit_i::<Num, Q<Num>>(), &quaternion),
        )),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the arccotangent of a quaternion.
pub fn acot<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    mul(
        (-Num::ONE / (Num::ONE + Num::ONE), ()),
        ln::<Num, Q<Num>>(div::<Num, Q<Num>>(
            add::<Num, Q<Num>>(unit_i::<Num, Q<Num>>(), &quaternion),
            sub::<Num, Q<Num>>(unit_i::<Num, Q<Num>>(), &quaternion),
        )),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the arcsecant of a quaternion.
pub fn asec<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    acos(inv::<Num, Q<Num>>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the arccosecant of a quaternion.
pub fn acsc<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    asin(inv::<Num, Q<Num>>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the inverse hyperbolic sinus of a quaternion.
pub fn asinh<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    ln(add::<Num, Q<Num>>(
        &quaternion,
        sqrt::<Num, Q<Num>>(add::<Num, Q<Num>>(
            mul::<Num, Q<Num>>(&quaternion, &quaternion),
            identity::<Num, Q<Num>>(),
        )),
    ))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the inverse hyperbolic cosinus of a quaternion.
pub fn acosh<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    ln(add::<Num, Q<Num>>(
        &quaternion,
        sqrt::<Num, Q<Num>>(sub::<Num, Q<Num>>(
            square::<Num, Q<Num>>(&quaternion),
            identity::<Num, Q<Num>>(),
        )),
    ))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the inverse hyperbolic tangent of a quaternion.
pub fn atanh<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    unscale(
        ln::<Num, Q<Num>>(div::<Num, Q<Num>>(
            add::<Num, Q<Num>>(identity::<Num, Q<Num>>(), &quaternion),
            sub::<Num, Q<Num>>(identity::<Num, Q<Num>>(), &quaternion),
        )), 
        Num::ONE + Num::ONE,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the inverse hyperbolic cotangent of a quaternion.
pub fn acoth<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    unscale(
        ln::<Num, Q<Num>>(div::<Num, Q<Num>>(
            add::<Num, Q<Num>>(&quaternion, identity::<Num, Q<Num>>()),
            sub::<Num, Q<Num>>(&quaternion, identity::<Num, Q<Num>>()),
        )), 
        Num::ONE + Num::ONE,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the inverse hyperbolic cosecant of a quaternion.
pub fn acsch<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    asinh(inv::<Num, Q<Num>>(quaternion))
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
#[inline]
/// Calculates the inverse hyperbolic secant of a quaternion.
pub fn asech<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    acosh(inv::<Num, Q<Num>>(quaternion))
}
