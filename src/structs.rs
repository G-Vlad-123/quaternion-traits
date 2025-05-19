/*!
This module provides structs for ease of use and/or changing functionality of other items in this crate.
*/

mod quat_struct;
pub use quat_struct::*;

mod unit_struct;
pub use unit_struct::*;

#[cfg(feature = "std")]
mod std_struct;
#[cfg(feature = "std")]
pub use std_struct::*;

#[cfg(feature = "display")]
mod quaternion_formatter;
#[cfg(feature = "display")]
pub use quaternion_formatter::*;
