
/*!
This library provides traits and many function for dealing with quaternions.

# Reason
This library exists to provide two things:
- A way to tie in the multiple existing implementations of
quaternions in rust.
- Add functions for pure maths purpaces in rust (like [`sin`] and [`pow_q`])

Curently this library is implemented for:
- [core](https://doc.rust-lang.org/core/)
- [std](https://doc.rust-lang.org/std/) (feature `std`)
- [quaternion](https://crates.io/crates/quaternion)
- [num-complex](https://crates.io/crates/num-complex) (feature `num-complex`)
- [num-traits](https://crates.io/crates/num-traits) (feature `num-traits`)

# Details

Currently the quaternion traits are implemented only in core rust and num,
but it's planned to add (though optional dependencies) these traits to crates like bevy,
ggez and the quaternion crates out there.

...

Due to how the traits are implemented this crate is naturaly usable with the
[quaternion](https://crates.io/crates/quaternion) crate. So for any crates that use
this crate this dependency is (hopefully) frictionless.

...

List of features:
- `std`: Adds `alloc` feature, adds [Std] struct.
- `alloc`: Adds [Quaternion], [Vector], [Complex] and [Scalar] implementations for
Box, Arc, Rc and Cow.
- `num-traits`: Adds [Pow](https://docs.rs/num-traits/latest/num_traits/pow/trait.Pow.html) to [Quat],
(if `std` is enabled) adds [Float](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html)
and all the required traits to the [Std] struct.
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

mod traits;
pub use traits::{
    Axis,
    Quaternion, NewQuaternion, QuaternionMethods,
    Vector, NewVector,
    Complex, NewComplex,
    Scalar, NewScalar,
    Rotation, NewRotation,
};

mod quat;
pub use quat::*;

mod quat_struct;
pub use quat_struct::Quat;

mod primitive;
#[allow(unused_imports)]
use primitive::Primitive;

#[cfg(feature = "std")]
mod std_impl;
#[cfg(feature = "std")]
pub use std_impl::Std;
