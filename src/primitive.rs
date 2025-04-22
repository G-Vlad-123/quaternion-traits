
use crate::core::default::Default;
use crate::core::cmp::PartialEq;
use crate::core::marker::Sized;
use crate::core::marker::Copy;

#[allow(dead_code)]
pub(crate) trait Primitive: Copy + PartialEq + Default + Sized + 'static {}

macro_rules! impl_primitie_trait_for_primitive {
    ( $( $ty:ty ),+ $(,)? ) => {
        $(
            impl Primitive for $ty {}
        )+
    };
}

impl_primitie_trait_for_primitive!{
    char, bool,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
}
