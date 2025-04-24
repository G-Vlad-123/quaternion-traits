
use crate::Scalar;
use crate::NewScalar;
use crate::core::num::{
    NonZero,
    Wrapping,
    Saturating,
};

macro_rules! impl_scalar_T_for_primitive {
    ( $ty:ty ) => {
        impl_scalar_T_for_primitive!{ $ty : f32 }
        impl_scalar_T_for_primitive!{ $ty : f64 }
    };
    ( $ty:ty : $float:ty ) => {
        impl Scalar<$float> for $ty {
            #[inline] fn scalar(&self) -> $float { *self as $float }
        }

        impl Scalar<$float> for NonZero<$ty> {
            #[inline] fn scalar(&self) -> $float { self.get() as $float }
        }

        impl Scalar<$float> for Wrapping<$ty> {
            #[inline] fn scalar(&self) -> $float { self.0 as $float }
        }

        impl Scalar<$float> for Saturating<$ty> {
            #[inline] fn scalar(&self) -> $float { self.0 as $float }
        }
        
        impl Scalar<$float> for &$ty {
            #[inline] fn scalar(&self) -> $float { **self as $float }
        }

        impl Scalar<$float> for &NonZero<$ty> {
            #[inline] fn scalar(&self) -> $float { self.get() as $float }
        }

        impl Scalar<$float> for &Wrapping<$ty> {
            #[inline] fn scalar(&self) -> $float { self.0 as $float }
        }

        impl Scalar<$float> for &Saturating<$ty> {
            #[inline] fn scalar(&self) -> $float { self.0 as $float }
        }

        impl NewScalar<$float> for $ty {
            #[inline] fn new_scalar(axis: $float) -> $ty { axis as $ty }
        }

        impl NewScalar<$float> for crate::core::option::Option<NonZero<$ty>> {
            #[inline] fn new_scalar(axis: $float) -> Self { NonZero::new(axis as $ty) }
        }

        impl NewScalar<$float> for Wrapping<$ty> {
            #[inline] fn new_scalar(axis: $float) -> Self { Wrapping(axis as $ty) }
        }

        impl NewScalar<$float> for Saturating<$ty> {
            #[inline] fn new_scalar(axis: $float) -> Self { Saturating(axis as $ty) }
        }
    };
    ( $( $ty:ty ),+ $(,)? ) => {
        $(
            impl_scalar_T_for_primitive!{ $ty }
        )+
    };
}

impl_scalar_T_for_primitive!{
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl Scalar<f64> for f32 {
    #[inline] fn scalar(&self) -> f64 { *self as f64 }
}

impl Scalar<f32> for f64 {
    #[inline] fn scalar(&self) -> f32 { *self as f32 }
}

impl Scalar<f64> for &f32 {
    #[inline] fn scalar(&self) -> f64 { **self as f64 }
}

impl Scalar<f32> for &f64 {
    #[inline] fn scalar(&self) -> f32 { **self as f32 }
}

impl Scalar<f32> for &f32 {
    #[inline] fn scalar(&self) -> f32 { **self as f32 }
}

impl Scalar<f64> for &f64 {
    #[inline] fn scalar(&self) -> f64 { **self as f64 }
}
