
/*!
This library provides traits and many function for dealing with quaternions.

# Reason

This library exists to provide two things:
- A way to tie in the multiple existing implementations of
quaternions in rust.
- Add functions for pure maths purpaces in rust (like [`sqrt`](quat::sqrt), [`cos`](quat::cos) and [`pow`](quat::pow_f))

Curently this library is implemented for:
- [core](https://doc.rust-lang.org/core/)
- [std](https://doc.rust-lang.org/std/) (feature `std`)
- [quaternion](https://crates.io/crates/quaternion)
- [num-complex](https://crates.io/crates/num-complex) (feature `num-complex`)
- [num-traits](https://crates.io/crates/num-traits) (feature `num-traits`)

# Details

Currently the quaternion traits are implemented only in core rust and num,
but it's planned to add (though optional dependencies) these traits to crates like
[bevy](https://crates.io/crates/bevy), [ggez](https://crates.io/crates/ggez)
and the other quaternion crates out there.

Due to how the traits are implemented this crate is naturaly usable with the
[quaternion](https://crates.io/crates/quaternion) crate. So for any crates that use
this crate this dependency is (hopefully) frictionless.

If possible this crate should implement functions for every quaternion use.
And it should implement every function that a crate it's comapatble with has.

This crate also currently has these pure maths functions for quaternions (excluding common ones):
[`exp`](quat::exp), [`ln`](quat::ln), [`sqrt`](quat::sqrt), [`sin`](quat::sin),
[`sinh`](quat::sinh), [`sec`](quat::sec), [`cos`](quat::cos), [`cosh`](quat::cosh),
[`csc`](quat::csc), [`sin_cos`](quat::sin_cos), [`tan`](quat::tan), [`tanh`](quat::tanh),
[`cot`](quat::cot), [`coth`](quat::coth), [`asin`](quat::asin), [`asinh`](quat::asinh),
[`asec`](quat::asec), [`acos`](quat::acos), [`acosh`](quat::acosh), [`acsc`](quat::acsc),
[`atan`](quat::atan), [`atanh`](quat::atanh), [`acot`](quat::acot), [`acoth`](quat::acoth).

This crate provides currently an unstable form of these functions:
- [`pow`](quat::pow_q) (the equasion used seams to not be fully agreed on though so it's at risk of change if
  another equasion comes out that is guaranteed to be correct)
- [`log`](quat::log) (simple [`ln`](quat::ln) division only works for complex quaternions that include the real axis and
  only one of the other 3 imaginary axis. Will remain unstable unill a better algorithm is found
  or it's found that quaternion logs can not have a formula for whatever reason)

List of features:
- `unstable`: Enables items that may change functionality or may be removed entirely.
- `std`: Adds `alloc` feature, adds [Std](struct::Std) struct.
- `alloc`: Adds [Quaternion], [Vector], [Complex] and [Scalar] implementations for
Box, Arc, Rc and Cow.
- `num-traits`: (If `unstable` is enabled) adds [Pow](https://docs.rs/num-traits/latest/num_traits/pow/trait.Pow.html) to [Quat],
(if `std` is enabled) adds [Float](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html)
and all the required traits to the [Std](struct::Std) struct.
- `num-complex`: Adds `num-traits` feature, adds [Complex] implementation for the Complex struct in this crate.

 */

#![crate_type = "lib"]
#![no_implicit_prelude]
#![no_std]

#![forbid(unconditional_recursion)]
#![forbid(unused_features)]
#![forbid(missing_docs)]

#![deny(unexpected_cfgs)]

#![warn(ambiguous_associated_items)]
#![warn(while_true)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "num-traits")]
extern crate num_traits;

#[cfg(feature = "num-complex")]
extern crate num_complex;

extern crate core;
extern crate libm;

pub mod traits;
pub use traits::{
    Quaternion,
    QuaternionConstructor,
    QuaternionConsts,
    QuaternionMethods,
};
#[allow(unused_imports)]
use traits::{
    Axis,

    Vector,
    VectorConstructor,
    VectorConsts,

    Complex,
    ComplexConstructor,
    ComplexConsts,

    Scalar,
    ScalarConstructor,
    ScalarConsts,

    Rotation,
    RotationConstructor,

    Matrix,
    MatrixConstructor,
};

pub mod quat;

mod quat_struct;
#[cfg(feature = "std")]
mod std_impl;

pub mod structs {
    /*!
    This module provides structs for ease of use and/or changing functionality.
    */
    pub use crate::quat_struct::*;

    #[cfg(feature = "std")]
    pub use crate::std_impl::*;
}


