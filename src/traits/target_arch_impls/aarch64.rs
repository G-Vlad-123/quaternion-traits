
use super::*;

use crate::core::arch::aarch64::{
    self as arch,
    float64x1_t,

    float64x2_t,
    float32x2_t,

    float32x4_t,

    float64x1x2_t,
    float64x1x3_t,
    float64x1x4_t,
};

#[cfg(feature = "f16")]
use crate::core::arch::aarch64::float16x4_t;

impl Scalar<f64> for float64x1_t {
    fn scalar(self) -> f64 {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl ScalarConstructor<f64> for float64x1_t {
    fn new_scalar(axis: f64) -> float64x1_t {
        unsafe {
            core::mem::transmute(axis)
        }
    }
}

impl ScalarConsts<f64> for float64x1_t {
    const ONE: Self = unsafe {
        core::mem::transmute(1.0)
    };
    
    const ZERO: Self = unsafe {
        core::mem::transmute(0.0)
    };
    
    const NAN: Self = unsafe {
        core::mem::transmute(f64::NAN)
    };
}

impl Complex<f64> for float64x2_t {
    fn real(self) -> f64 {
        unsafe {
            __union::<Self, f64, 2>{
                simd: self
            }
            .array[0]
        }
    }

    fn imaginary(self) -> f64 {
        unsafe {
            __union::<Self, f64, 2>{
                simd: self
            }
            .array[1]
        }
    }
}

impl ComplexConstructor<f64> for float64x2_t {
    fn new_complex(real: f64, imaginary: f64) -> Self {
        unsafe {
            __union::<Self, f64, 2>{
                array: [real, imaginary]
            }
            .simd
        }
    }
}

impl ComplexConsts<f64> for float64x2_t {
    const ORIGIN: Self = unsafe {
        __union::<Self, f64, 2>{
            array: [0.0, 0.0]
        }
        .simd
    };

    const IDENTITY: Self = unsafe {
        __union::<Self, f64, 2>{
            array: [1.0, 0.0]
        }
        .simd
    };

    const NAN: Self = unsafe {
        __union::<Self, f64, 2>{
            array: [f64::NAN, f64::NAN]
        }
        .simd
    };

    const UNIT_IMAGINARY: Self = unsafe {
        __union::<Self, f64, 2>{
            array: [0.0, 1.0]
        }
        .simd
    };
}

impl Complex<f32> for float32x2_t {
    fn real(self) -> f32 {
        unsafe {
            __union::<Self, f32, 2>{
                simd: self
            }
            .array[0]
        }
    }

    fn imaginary(self) -> f32 {
        unsafe {
            __union::<Self, f32, 2>{
                simd: self
            }
            .array[1]
        }
    }
}

impl ComplexConstructor<f32> for float32x2_t {
    fn new_complex(real: f32, imaginary: f32) -> Self {
        unsafe {
            __union::<Self, f32, 2>{
                array: [real, imaginary]
            }
            .simd
        }
    }
}

impl ComplexConsts<f32> for float32x2_t {
    const ORIGIN: Self = unsafe {
        __union::<Self, f32, 2>{
            array: [0.0, 0.0]
        }
        .simd
    };

    const IDENTITY: Self = unsafe {
        __union::<Self, f32, 2>{
            array: [1.0, 0.0]
        }
        .simd
    };

    const NAN: Self = unsafe {
        __union::<Self, f32, 2>{
            array: [f32::NAN, f32::NAN]
        }
        .simd
    };

    const UNIT_IMAGINARY: Self = unsafe {
        __union::<Self, f32, 2>{
            array: [0.0, 1.0]
        }
        .simd
    };
}
