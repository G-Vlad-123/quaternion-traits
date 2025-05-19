/*!
Functions for dealing with generic quaternions.

This crate provides a lot of functions (`143`) including
both convetnional ones ([`add`], [`mul`]), helper ones ([`display`](display::display),
[`product`]), game/graphichs ones ([`to_matrix_3`], [`rotation_from_to`]) and
pure math ones ([`cos`], [`ln`]).

# Note
If you use this crate for it's traits and already have another quaternion
crate (or you use a crate that provides quaternions already) unless necesarry
it's recommended you use the functions/methods of the alrady used crate, as
this crate is general use while other crates might provide more focused implementations
that may provide more optimized functions.

This module is here to fill any gaps or provide functionality that you don't already have.
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

type Q<N> = (N, [N; 3]);

mod inputless;
pub use inputless::*;

mod meta_manipulation;
pub use meta_manipulation::*;

mod math;
pub use math::*;

mod relational_ops;
pub use relational_ops::*;

#[cfg(feature = "rotation")]
mod rotation_ops;
#[cfg(feature = "rotation")]
pub use rotation_ops::*;

mod iterator_ops;
pub use iterator_ops::*;

mod conversions;
pub use conversions::*;

#[cfg(feature = "trigonometry")]
mod trigonometry;
#[cfg(feature = "trigonometry")]
pub use trigonometry::*;

#[cfg(feature = "display")]
mod display;
#[cfg(feature = "display")]
pub use display::*;
