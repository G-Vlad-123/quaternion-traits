This library provides traits and many function for dealing with quaternions.

# Reason

This library exists to provide two things:
- A way to tie in the multiple existing implementations of
quaternions in rust.
- Add quaternion functions for pure maths purpaces in rust (like `sqrt`, `cos` and `pow`)

Curently this library is implemented for:
- [core](https://doc.rust-lang.org/core/)
- [std](https://doc.rust-lang.org/std/) (feature `std`)
- [quaternion](https://crates.io/crates/quaternion)
- [num-complex](https://crates.io/crates/num-complex) (feature `num-complex`)
- [num-traits](https://crates.io/crates/num-traits) (feature `num-traits`)

# Details

Since this crate is not published on crates.io, any item in this crate
may or may not be marked as unstable in a future patch up untill
the initial release.

Currently the quaternion traits are implemented only in core rust and
[num](https://crates.io/crates/num/0.4.3),
but it's planned to add (though optional dependencies) these traits to crates like
[bevy](https://crates.io/crates/bevy), [ggez](https://crates.io/crates/ggez)
and the quaternion crates out there.

Due to how the traits are implemented this crate is naturaly usable with the
[quaternion](https://crates.io/crates/quaternion) crate. So for any projects that use
this crate this dependency is has (hopefully) the least amount of friction possible.

If possible this crate should implement functions for every quaternion use.
And it should implement every function that a crate it's comapatble with has.

This crate also currently has these pure maths functions for quaternions (excluding common ones):
`exp`, `ln`, `sqrt`, `sin`, `sinh`, `sec`, `cos`, `cosh`, `csc`, `sin_cos`, `tan`, `tanh`,
`cot`, `coth`.

This crate provides currently unstable forms of these equasions:
- `pow` (the equasion used seams to not be fully agreed on though so it's at risk of change if
  another equasion comes out that is guaranteed to be correct)
- `log` (simple `ln` division only works for complex quaternions that include the real axis and only one of 
  the other 3 imaginary axis. Will remain unstable untill a better algorithm is found
  or it's found that quaternion logs can not have a formula for whatever reason)

# Versions

Current version **0.1.2.0**

This crate uses the *HUMAN.MAJOR.MINOR.PATCH* version format.
And version *1.0.0.0* would be the first true release.

Due to the fact that there is no support for 4 version numbers
the crates.io version system willl be *[1000 * HUMAN + MAJOR].MINOR.PATCH*
So the full release version would be *1000.0.0*
(So this crate uses epoch sematic versioning)

This versioning scheme applies since *0.2.0.0*
