
use super::*;

/// Constructs a unit quaternion with all [`Num::NAN`s](Axis::NAN).
/// 
/// # Example
/// ```
/// # use quaternion_traits::structs::UnitQuat;
/// use quaternion_traits::unit::nan;
/// 
/// let quat: UnitQuat<f32> = nan::<f32, _>();
/// 
/// assert!( quat.r.is_nan() );
/// assert!( quat.i.is_nan() );
/// assert!( quat.j.is_nan() );
/// assert!( quat.k.is_nan() );
/// ```
#[inline]
pub fn nan<Num, Out>() -> Out
where
    Num: Axis,
    Out: UnitQuaternionConstructor<Num>,
{
    unsafe {
        Out::new_unit_quat_unchecked(Num::NAN, Num::NAN, Num::NAN, Num::NAN)
    }
}
