
use crate::core::option::Option;
use crate::{
    Axis,
    Scalar,
    UnitQuaternion,
    UnitQuaternionConstructor,
    UnitQuaternionConsts,
};
#[cfg(feature = "std")]
use crate::structs::Std;

/// Default given struct by this crate for a unit quaternion.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct UnitQuat<Num: Axis> {
    r: Num,
    i: Num,
    j: Num,
    k: Num,
}

impl<Num: Axis> UnitQuat<Num> {
    /// Constructs a new unit quaternion.
    #[inline]
    pub fn new(r: impl Scalar<Num>, i: impl Scalar<Num>, j: impl Scalar<Num>, k: impl Scalar<Num>) -> Option<Self> {
        if (r.scalar() * r.scalar() + i.scalar() * i.scalar() + j.scalar() * j.scalar() + k.scalar() * k.scalar() - Num::ONE).abs() >= Num::ERROR * Num::ERROR {
            return Option::None;
        }

        Option::Some(UnitQuat { r: r.scalar(), i: i.scalar(), j: j.scalar(), k: k.scalar() })
    }

    /// Constructs a new unit quaternion by normalizing it.
    #[inline]
    pub fn new_normalized(r: impl Scalar<Num>, i: impl Scalar<Num>, j: impl Scalar<Num>, k: impl Scalar<Num>) -> Self {
        let unscale = Num::ONE / (r.scalar() * r.scalar() + i.scalar() * i.scalar() + j.scalar() * j.scalar() + k.scalar() * k.scalar()).sqrt();
        UnitQuat {
            r: r.scalar() * unscale,
            i: i.scalar() * unscale,
            j: j.scalar() * unscale,
            k: k.scalar() * unscale,
        }
    }

    /// Constructs a new unit quaternion without checking if it's valid.
    #[inline]
    pub const unsafe fn new_uncehcekd(r: Num, i: Num, j: Num, k: Num) -> Self {
        UnitQuat { r, i, j, k }
    }
}

impl<Num: Axis> UnitQuaternion<Num> for UnitQuat<Num> {
    fn r(&self) -> Num { self.r }
    fn i(&self) -> Num { self.i }
    fn j(&self) -> Num { self.j }
    fn k(&self) -> Num { self.k }
}

impl<Num: Axis> UnitQuaternionConstructor<Num> for UnitQuat<Num> {
    #[inline]
    fn new_unit_quat(r: Num, i: Num, j: Num, k: Num) -> Option<Self> {
        Self::new(r, i, j, k)
    }

    #[inline]
    unsafe fn new_unit_quat_unchecked(r: Num, i: Num, j: Num, k: Num) -> Self {
        unsafe { Self::new_uncehcekd(r, i, j, k) }
    }
}

impl<Num: Axis> UnitQuaternionConsts<Num> for UnitQuat<Num> {
    const IDENTITY: Self = UnitQuat { r: Num::ONE, i: Num::ZERO, j: Num::ZERO, k: Num::ZERO};
    const NAN: Self = UnitQuat { r: Num::NAN, i: Num::NAN, j: Num::NAN, k: Num::NAN};
    const UNIT_R: Self = UnitQuat { r: Num::ONE, i: Num::ZERO, j: Num::ZERO, k: Num::ZERO};
    const UNIT_I: Self = UnitQuat { r: Num::ZERO, i: Num::ONE, j: Num::ZERO, k: Num::ZERO};
    const UNIT_J: Self = UnitQuat { r: Num::ZERO, i: Num::ZERO, j: Num::ONE, k: Num::ZERO};
    const UNIT_K: Self = UnitQuat { r: Num::ZERO, i: Num::ZERO, j: Num::ZERO, k: Num::ONE};
}

impl<Num: Axis> crate::core::default::Default for UnitQuat<Num> {
    fn default() -> Self { UnitQuat::IDENTITY }
}

/// Type alias for `Unit<f32>` (uses `Std<f32>` is `std` is enabled)
#[cfg(any(not(feature = "std"), doc))]
pub type Unit32 = UnitQuat<f32>;
/// Type alias for `Unit<f32>` (uses `Std<f32>` is `std` is enabled)
#[cfg(all(feature = "std", not(doc)))]
pub type Unit32 = UnitQuat<Std<f32>>;

/// Type alias for `Unit<f64>` (uses `Std<f64>` is `std` is enabled)
#[cfg(any(not(feature = "std"), doc))]
pub type Unit64 = UnitQuat<f64>;
/// Type alias for `Unit<f64>` (uses `Std<f64>` is `std` is enabled)
#[cfg(all(feature = "std", not(doc)))]
pub type Unit64 = UnitQuat<Std<f64>>;
