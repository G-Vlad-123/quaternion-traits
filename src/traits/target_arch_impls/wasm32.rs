
use super::*;

use crate::core::arch::wasm32::{
    self as arch,
    v128,
};

impl Quaternion<f32> for v128 {
    #[inline] fn r(&self) -> f32 { unsafe { arch::f32x4_extract_lane::<0>(self) } }
    #[inline] fn i(&self) -> f32 { unsafe { arch::f32x4_extract_lane::<1>(self) } }
    #[inline] fn j(&self) -> f32 { unsafe { arch::f32x4_extract_lane::<2>(self) } }
    #[inline] fn k(&self) -> f32 { unsafe { arch::f32x4_extract_lane::<3>(self) } }
}

impl QuaternionConstructor<f32> for v128 {
    #[inline] fn new_quat(r: f32, i: f32, j: f32, k: f32) -> f32 { unsafe { arch::f32x4(r, i, j, k) } }
}

impl QuaternionConsts<f32> for v128 {
    const ORIGIN: Self = unsafe { arch::f32x4(0.0, 0.0, 0.0, 0.0) };
    const IDENTITY: Self = unsafe { arch::f32x4(1.0, 0.0, 0.0, 0.0) };
    const NAN: Self = unsafe { arch::f32x4(f32::NAN, f32::NAN, f32::NAN, f32::NAN) };
    const UNIT_I: Self = unsafe { arch::f32x4(0.0, 1.0, 0.0, 0.0) };
    const UNIT_J: Self = unsafe { arch::f32x4(0.0, 0.0, 1.0, 0.0) };
    const UNIT_K: Self = unsafe { arch::f32x4(0.0, 0.0, 0.0, 1.0) };
}

impl QuaternionMethods<f32> for v128 {
    #[inline] fn add(self, other: impl Quaternion<f32>) -> Self {
        unsafe {
            arch::f32x4_add(self, v128::from_quat(other))
        }
    }

    #[inline] fn sub(self, other: impl Quaternion<f32>) -> Self {
        unsafe {
            arch::f32x4_sub(self, v128::from_quat(other))
        }
    }

    #[inline] fn neg(self) -> Self {
        unsafe {
            arch::f32x4_neg(self)
        }
    }

    #[inline] fn eq(self, other: impl Quaternion<f32>) -> bool {
        unsafe {
            arch::f32x4_eq(self, v128::from_quat(other))
        }
    }
}

impl Complex<f64> for v128 {
    #[inline] fn real(&self) -> f64 { unsafe { arch::f64x2_extract_lane::<0>(self) } }
    #[inline] fn imaginary(&self) -> f64 { unsafe { arch::f64x2_extract_lane::<1>(self) } }
}

impl ComplexConstructor<f64> for v128 {
    #[inline] fn new_complex(real: f64, imaginary: f64) -> f64 { unsafe { arch::f64x2(real, imaginary) } }
}

impl ComplexConsts<f64> for v128 {
    const ORIGIN: Self = unsafe { arch::f64x2(0.0, 0.0) };
    const IDENTITY: Self = unsafe { arch::f64x2(1.0, 0.0) };
    const NAN: Self = unsafe { arch::f64x2(f64::NAN, f64::NAN) };
    const UNIT_IMAGINARY: Self = unsafe { arch::f64x2(0.0, 1.0) };
}
