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

Due to how the traits are implemented this crate is naturaly usable with the
[quaternion](https://crates.io/crates/quaternion) crate. So for any crates that use
this crate this dependency is (hopefully) frictionless.

