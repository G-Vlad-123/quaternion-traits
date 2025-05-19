/*!
Functions for dealing with unit quaternions.

# Note
If you use this crate for it's traits and already have another quaternion
crate (or you use a crate that provides quaternions already) unless necesarry
it's recommended you use the functions/methods of the alrady used crate, as
this crate is general use while other crates might provide more focused implementations
that may provide more optimized functions.

This module is here to fill any gaps or provide functionality that you don't already have.

Since this is a new module it may still have missing features.
 */

use crate::core::option::Option;
use crate::{
    Axis,

    Quaternion,
    QuaternionConstructor,

    UnitQuaternion,
    UnitQuaternionConstructor,

    Vector,
    VectorConstructor,

    Complex,
    ComplexConstructor,

    Scalar,
    ScalarConstructor,
};

#[cfg(feature = "rotation")]
use crate::{
    Rotation,
    RotationConstructor,
};

#[cfg(feature = "matrix")]
use crate::{
    Matrix,
    MatrixConstructor,
};

#[inline(always)]
// short hard for the unsafe unchecked call.
fn new_unit<Num: Axis, Out: UnitQuaternionConstructor<Num>>(r: Num, i: Num, j: Num, k: Num) -> Out {
    crate::core::debug_assert!( r * r + i * i + j * j + k * k - Num::ONE < Num::ERROR * Num::ERROR );
    unsafe { Out::new_unit_quat_unchecked(r, i, j, k) }
}

type U<N> = crate::structs::UnitQuat<N>;

mod math;
pub use math::*;
