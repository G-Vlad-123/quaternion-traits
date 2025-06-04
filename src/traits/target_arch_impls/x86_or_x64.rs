
use super ::*;

#[cfg(target_arch = "x86")]
use crate::core::arch::x86::{
    self as arch,
    __m128,
    __m128d,
    __m256d,
    __m128i,
    __m256i,
};

#[cfg(target_arch = "x86_64")]
use crate::core::arch::x86_64::{
    self as arch,
    __m128,
    __m128d,
    __m256d,
};

impl Quaternion<f32> for __m128
{
    fn r(&self) -> f32 {
        unsafe { __union::<Self, f32, 4> { simd: *self } .array[0] }
    }
    fn i(&self) -> f32 {
        unsafe { __union::<Self, f32, 4> { simd: *self } .array[1] }
    }
    fn j(&self) -> f32 {
        unsafe { __union::<Self, f32, 4> { simd: *self } .array[2] }
    }
    fn k(&self) -> f32 {
        unsafe { __union::<Self, f32, 4> { simd: *self } .array[3] }
    }
}

impl QuaternionConstructor<f32> for __m128
{
    fn new_quat(r: f32, i: f32, j: f32, k: f32) -> Self {
        unsafe {
            __union::<Self, f32, 4> {
                array: [ r, i, j, k ]
            }
            .simd
        }
    }
}


impl QuaternionConsts<f32> for __m128
{
    const IDENTITY: Self = unsafe {
        __union::<Self, f32, 4> {
            array: [ 1.0, 0.0, 0.0, 0.0 ]
        }
        .simd
    };

    const ORIGIN: Self = unsafe {
        __union::<Self, f32, 4> {
            array: [ 0.0, 0.0, 0.0, 0.0 ]
        }
        .simd
    };

    const NAN: Self = unsafe {
        __union::<Self, f32, 4> {
            array: [ f32::NAN, f32::NAN, f32::NAN, f32::NAN ]
        }
        .simd
    };

    const UNIT_I: Self = unsafe {
        __union::<Self, f32, 4> {
            array: [ 0.0, 1.0, 0.0, 0.0 ]
        }
        .simd
    };

    const UNIT_J: Self = unsafe {
        __union::<Self, f32, 4> {
            array: [ 0.0, 0.0, 1.0, 0.0 ]
        }
        .simd
    };

    const UNIT_K: Self = unsafe {
        __union::<Self, f32, 4> {
            array: [ 0.0, 0.0, 0.0, 1.0 ]
        }
        .simd
    };
}


impl QuaternionMethods<f32> for __m128 {
    fn add(self, other: impl Quaternion<f32>) -> Self {
        unsafe {
            arch::_mm256_castps256_ps128(
                arch::_mm256_add_ps(
                    arch::_mm256_castps128_ps256(self),
                    arch::_mm256_castps128_ps256(Self::from_quat(other))
                )
            )
        }
    }
    
    fn sub(self, other: impl Quaternion<f32>) -> Self {
        unsafe {
            arch::_mm256_castps256_ps128(
                arch::_mm256_sub_ps(
                    arch::_mm256_castps128_ps256(self),
                    arch::_mm256_castps128_ps256(Self::from_quat(other))
                )
            )
        }
    }

    // fn eq(self, other: impl Quaternion<f32>) -> bool {
    //     unsafe {
    //         arch::_mm_cmp_ps(self, Self::from_quat(other))
    //     }
    // }
}


// impl Quaternion<Std<f32>> for Std<__m128>
// {
//     fn r(&self) -> Std<f32> {
//         unsafe { __union::<Self, Std<f32>, 4> { simd: *self } .array[0] }
//     }
//     fn i(&self) -> Std<f32> {
//         unsafe { __union::<Self, Std<f32>, 4> { simd: *self } .array[1] }
//     }
//     fn j(&self) -> Std<f32> {
//         unsafe { __union::<Self, Std<f32>, 4> { simd: *self } .array[2] }
//     }
//     fn k(&self) -> Std<f32> {
//         unsafe { __union::<Self, Std<f32>, 4> { simd: *self } .array[3] }
//     }
// }


// impl QuaternionConstructor<Std<f32>> for Std<__m128>
// {
//     fn new_quat(r: Std<f32>, i: Std<f32>, j: Std<f32>, k: Std<f32>) -> Self {
//         unsafe {
//             __union::<Self, Std<f32>, 4> {
//                 array: [ r, i, j, k ]
//             }
//             .simd
//         }
//     }
// }


// impl QuaternionConsts<Std<f32>> for Std<__m128>
// {
//     const IDENTITY: Self = unsafe {
//         __union::<Self, f32, 4> {
//             array: [ 1.0, 0.0, 0.0, 0.0 ]
//         }
//         .simd
//     };

//     const ORIGIN: Self = unsafe {
//         __union::<Self, f32, 4> {
//             array: [ 0.0, 0.0, 0.0, 0.0 ]
//         }
//         .simd
//     };

//     const NAN: Self = unsafe {
//         __union::<Self, f32, 4> {
//             array: [ f32::NAN, f32::NAN, f32::NAN, f32::NAN ]
//         }
//         .simd
//     };

//     const UNIT_I: Self = unsafe {
//         __union::<Self, f32, 4> {
//             array: [ 0.0, 1.0, 0.0, 0.0 ]
//         }
//         .simd
//     };

//     const UNIT_J: Self = unsafe {
//         __union::<Self, f32, 4> {
//             array: [ 0.0, 0.0, 1.0, 0.0 ]
//         }
//         .simd
//     };

//     const UNIT_K: Self = unsafe {
//         __union::<Self, f32, 4> {
//             array: [ 0.0, 0.0, 0.0, 1.0 ]
//         }
//         .simd
//     };
// }


// impl QuaternionMethods<Std<f32>> for Std<__m128> {
//     fn add(self, other: impl Quaternion<Std<f32>>) -> Self {
//         Std(
//             unsafe {
//                 arch::_mm256_castps256_ps128(
//                     arch::_mm256_add_ps(
//                         arch::_mm256_castps128_ps256(self.0),
//                         arch::_mm256_castps128_ps256(Self::from_quat(other).0)
//                     )
//                 )
//             }
//         )
//     }
    
//     fn sub(self, other: impl Quaternion<Std<f32>>) -> Self {
//         Std(
//             unsafe {
//                 arch::_mm256_castps256_ps128(
//                     arch::_mm256_sub_ps(
//                         arch::_mm256_castps128_ps256(self.0),
//                         arch::_mm256_castps128_ps256(Self::from_quat(other).0)
//                     )
//                 )
//             }
//         )
//     }
// }


impl Quaternion<f64> for __m256d
{
    fn r(&self) -> f64 {
        unsafe { __union::<Self, f64, 4> { simd: *self } .array[0] }
    }
    fn i(&self) -> f64 {
        unsafe { __union::<Self, f64, 4> { simd: *self } .array[1] }
    }
    fn j(&self) -> f64 {
        unsafe { __union::<Self, f64, 4> { simd: *self } .array[2] }
    }
    fn k(&self) -> f64 {
        unsafe { __union::<Self, f64, 4> { simd: *self } .array[3] }
    }
}


impl QuaternionConstructor<f64> for __m256d
{
    fn new_quat(r: f64, i: f64, j: f64, k: f64) -> Self {
        unsafe {
            __union::<Self, f64, 4> {
                array: [ r, i, j, k ]
            }
            .simd
        }
    }
}


impl QuaternionConsts<f64> for __m256d
{
    const IDENTITY: Self = unsafe {
        __union::<Self, f64, 4> {
            array: [ 1.0, 0.0, 0.0, 0.0 ]
        }
        .simd
    };

    const ORIGIN: Self = unsafe {
        __union::<Self, f64, 4> {
            array: [ 0.0, 0.0, 0.0, 0.0 ]
        }
        .simd
    };

    const NAN: Self = unsafe {
        __union::<Self, f64, 4> {
            array: [ f64::NAN, f64::NAN, f64::NAN, f64::NAN ]
        }
        .simd
    };

    const UNIT_I: Self = unsafe {
        __union::<Self, f64, 4> {
            array: [ 0.0, 1.0, 0.0, 0.0 ]
        }
        .simd
    };

    const UNIT_J: Self = unsafe {
        __union::<Self, f64, 4> {
            array: [ 0.0, 0.0, 1.0, 0.0 ]
        }
        .simd
    };

    const UNIT_K: Self = unsafe {
        __union::<Self, f64, 4> {
            array: [ 0.0, 0.0, 0.0, 1.0 ]
        }
        .simd
    };
}

impl QuaternionMethods<f64> for __m256d {
    fn add(self, other: impl Quaternion<f64>) -> Self {
        unsafe {
            arch::_mm256_add_pd(self, Self::from_quat(other))
        }
    }

    fn sub(self, other: impl Quaternion<f64>) -> Self {
        unsafe {
            arch::_mm256_sub_pd(self, Self::from_quat(other))
        }
    }
}


// impl Quaternion<Std<f64>> for Std<__m256d>
// {
//     fn r(&self) -> Std<f64> {
//         unsafe { __union::<Self, Std<f64>, 4> { simd: *self } .array[0] }
//     }
//     fn i(&self) -> Std<f64> {
//         unsafe { __union::<Self, Std<f64>, 4> { simd: *self } .array[1] }
//     }
//     fn j(&self) -> Std<f64> {
//         unsafe { __union::<Self, Std<f64>, 4> { simd: *self } .array[2] }
//     }
//     fn k(&self) -> Std<f64> {
//         unsafe { __union::<Self, Std<f64>, 4> { simd: *self } .array[3] }
//     }
// }


// impl QuaternionConstructor<Std<f64>> for Std<__m256d>
// {
//     fn new_quat(r: Std<f64>, i: Std<f64>, j: Std<f64>, k: Std<f64>) -> Self {
//         unsafe {
//             __union::<Self, Std<f64>, 4> {
//                 array: [ r, i, j, k ]
//             }
//             .simd
//         }
//     }
// }


// impl QuaternionConsts<Std<f64>> for Std<__m256d>
// {
//     const IDENTITY: Self = unsafe {
//         __union::<Self, f64, 4> {
//             array: [ 1.0, 0.0, 0.0, 0.0 ]
//         }
//         .simd
//     };

//     const ORIGIN: Self = unsafe {
//         __union::<Self, f64, 4> {
//             array: [ 0.0, 0.0, 0.0, 0.0 ]
//         }
//         .simd
//     };

//     const NAN: Self = unsafe {
//         __union::<Self, f64, 4> {
//             array: [ f64::NAN, f64::NAN, f64::NAN, f64::NAN ]
//         }
//         .simd
//     };

//     const UNIT_I: Self = unsafe {
//         __union::<Self, f64, 4> {
//             array: [ 0.0, 1.0, 0.0, 0.0 ]
//         }
//         .simd
//     };

//     const UNIT_J: Self = unsafe {
//         __union::<Self, f64, 4> {
//             array: [ 0.0, 0.0, 1.0, 0.0 ]
//         }
//         .simd
//     };

//     const UNIT_K: Self = unsafe {
//         __union::<Self, f64, 4> {
//             array: [ 0.0, 0.0, 0.0, 1.0 ]
//         }
//         .simd
//     };
// }

// impl QuaternionMethods<Std<f64>> for Std<__m256d> {
//     fn add(self, other: impl Quaternion<Std<f64>>) -> Self {
//         Std(
//             unsafe {
//                 arch::_mm256_add_pd(self.0, Self::from_quat(other).0)
//             }
//         )
//     }

//     fn sub(self, other: impl Quaternion<Std<f64>>) -> Self {
//         Std(
//             unsafe {
//                 arch::_mm256_sub_pd(self.0, Self::from_quat(other).0)
//             }
//         )
//     }
// }


impl Complex<f64> for __m128d
{
    fn real(&self) -> f64 {
        unsafe { __union::<Self, f64, 2> { simd: *self } .array[0] }
    }
    fn imaginary(&self) -> f64 {
        unsafe { __union::<Self, f64, 2> { simd: *self } .array[1] }
    }
}


impl ComplexConstructor<f64> for __m128d
{
    fn new_complex(real: f64, imaginary: f64) -> Self {
        unsafe {
            __union::<Self, f64, 2> {
                array: [ real, imaginary ]
            }
            .simd
        }
    }
}


impl ComplexConsts<f64> for __m128d
{
    const IDENTITY: Self = unsafe {
        __union::<Self, f64, 2> {
            array: [ 1.0, 0.0 ]
        }
        .simd
    };

    const ORIGIN: Self = unsafe {
        __union::<Self, f64, 2> {
            array: [ 0.0, 0.0 ]
        }
        .simd
    };

    const NAN: Self = unsafe {
        __union::<Self, f64, 2> {
            array: [ f64::NAN, f64::NAN ]
        }
        .simd
    };

    const UNIT_IMAGINARY: Self = unsafe {
        __union::<Self, f64, 2> {
            array: [ 0.0, 1.0 ]
        }
        .simd
    };
}


// impl Complex<Std<f64>> for Std<__m128d>
// {
//     fn real(&self) -> Std<f64> {
//         unsafe { __union::<Self, Std<f64>, 2> { simd: *self } .array[0] }
//     }
//     fn imaginary(&self) -> Std<f64> {
//         unsafe { __union::<Self, Std<f64>, 2> { simd: *self } .array[1] }
//     }
// }


// impl ComplexConstructor<Std<f64>> for Std<__m128d>
// {
//     fn new_complex(real: Std<f64>, imaginary: Std<f64>) -> Self {
//         unsafe {
//             __union::<Self, Std<f64>, 2> {
//                 array: [ real, imaginary ]
//             }
//             .simd
//         }
//     }
// }


// impl ComplexConsts<Std<f64>> for Std<__m128d>
// {
//     const IDENTITY: Self = unsafe {
//         __union::<Self, f64, 2> {
//             array: [ 1.0, 0.0 ]
//         }
//         .simd
//     };

//     const ORIGIN: Self = unsafe {
//         __union::<Self, f64, 2> {
//             array: [ 0.0, 0.0 ]
//         }
//         .simd
//     };

//     const NAN: Self = unsafe {
//         __union::<Self, f64, 2> {
//             array: [ f64::NAN, f64::NAN ]
//         }
//         .simd
//     };

//     const UNIT_IMAGINARY: Self = unsafe {
//         __union::<Self, f64, 2> {
//             array: [ 0.0, 1.0 ]
//         }
//         .simd
//     };
// }
