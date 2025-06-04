

use super::*;

#[allow(non_camel_case_types)]
union __union<Simd: crate::core::marker::Copy, Elem: crate::core::marker::Copy, const N: usize> {
    simd: Simd,
    array: [Elem; N],
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_or_x64;

#[cfg(any(target_arch = "arm64ec", target_arch = "aarch64"))]
mod aarch64;

#[cfg(target_arch = "wasm32")]
#[target_feature(enable = "simd128")]
mod wasm32;

#[cfg(feature = "portable_simd")]
mod simd;
