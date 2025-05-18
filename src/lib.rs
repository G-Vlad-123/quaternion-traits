
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
- [`rem`](quat::rem) (the equasion used uses gaussian intigers, but untill a paper showing this is the convention
or that another method is prefered this will remain as unstable)
- [`pow`](quat::pow_q) (the equasion used seams to not be fully agreed on though so it's at risk of change if
  another equasion comes out that is guaranteed to be correct)
- [`log`](quat::log) (simple [`ln`](quat::ln) division only works for complex quaternions that include the real axis and
  only one of the other 3 imaginary axis. Will remain unstable unill a better algorithm is found
  or it's found that quaternion logs can not have a formula for whatever reason)

List of features:
- `std`: (enabled by default) Adds `alloc` feature, adds [Std](struct::Std) struct.
- `alloc`: Adds [Quaternion], [Vector], [Complex] and [Scalar] implementations for
Box, Arc, Rc and Cow, adds the [`to_string`](quat::to_string) function.
- `full`: (enabled by default) Enables all stable feature flags that don't have any dependencies
(`qol_fns`, `math_fns`, `trigonometry`, `rotation`, `matrix`, `display`).
- `qol_fns`: Adds quality of life functions and methods. (eg: [`add_scalar`](quat::add_scalar))
- `math_fns`: Adds pure math focused functions. (eg: [`sqrt`](quat::sqrt))
- `trigonometry`: Adds trigonomentric functions. (eg: [`sin`](quat::sin))
- `rotation`: Adds rotation arithmatic functions. (eg: [`rotation_from_to`](quat::rotation_from_to))
- `matrix`: Adds matrix arithmatic functions. (eg: [`to_matrix_3`](quat::to_matrix_3))
- `display`: Adds [`str`] and [`String`](crate::alloc::string::String) functions. (eg: [`display`](quat::display))
- `unstable`: Enables items that may change functionality or may be removed entirely.

List of dependency features:
- `num`: Adds all the num traits `num-traits`, `num-complex`, `num-rational`, `num-bigint`.
- `num-traits`: Adds diverse trait implementations for [`Quat`](structs::Quat).
- `num-complex`: Adds [Complex] implementation for the Complex struct.
- `num-rational`: Adds [Scalar] implementations for the Ratio struct.
- `num-bigint`: Adds [Scalar] implementation for the BigUint and BigInt structs.
- `serde`: Adds [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html)
and [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) implementation
for [`Std`](structs::Std).

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

#![allow(confusable_idents)]
#![allow(mixed_script_confusables)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(feature = "num-traits", feature = "num-complex", feature = "num-rational", feature = "num-bigint"))]
extern crate num_traits;

#[cfg(feature = "num-complex")]
extern crate num_complex;

#[cfg(feature = "num-rational")]
extern crate num_rational;

#[cfg(feature = "num-bigint")]
extern crate num_bigint;

#[cfg(feature = "num-rational")]
extern crate num_integer;

#[cfg(feature = "serde")]
extern crate serde;

extern crate core;
extern crate libm;
extern crate thiserror as err;

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
};
#[cfg(feature = "rotation")]
use traits::{
    Rotation,
    RotationConstructor,
};
#[cfg(feature = "matrix")]
use traits::{
    Matrix,
    MatrixConstructor,
};

pub mod quat;

pub mod structs;


