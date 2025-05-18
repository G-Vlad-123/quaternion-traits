
#[cfg(feature = "num-complex")]
mod num_complex_impl {
    use crate::num_complex::Complex;
    use crate::{
        Axis,
        Scalar,
        ScalarConstructor,
        ScalarConsts,
    };

    impl<Num: Axis, S: Scalar<Num>> crate::Complex<Num> for Complex<S> {
        #[inline] fn real(&self) -> Num {
            self.re.scalar()
        }
        
        #[inline] fn imaginary(&self) -> Num {
            self.im.scalar()
        }
    }

    impl<Num: Axis, S: ScalarConstructor<Num>> crate::ComplexConstructor<Num> for Complex<S> {
        #[inline] fn new_complex(real: Num, imaginary: Num) -> Self {
            Complex::new(
                S::new_scalar(real),
                S::new_scalar(imaginary),
            )
        }
    }

    impl<Num: Axis, S: ScalarConsts<Num>> crate::ComplexConsts<Num> for Complex<S> {
        const ORIGIN: Self = Complex::new(S::ZERO, S::ZERO);
        const IDENTITY: Self = Complex::new(S::ONE, S::ZERO);
        const NAN: Self = Complex::new(S::NAN, S::NAN);
        const UNIT_IMAGINARY: Self = Complex::new(S::ZERO, S::ONE);
    }
}

#[cfg(feature = "num-rational")]
mod num_rational_impl {
    use crate::core::clone::Clone;
    use crate::num_rational::Ratio;
    use crate::num_integer::Integer;
    use crate::num_traits::{
        ConstOne,
        ConstZero,
        ToPrimitive,
        Bounded,
        NumCast,
        float::FloatCore,
        Signed,
    };
    use crate::{
        Axis,
        Scalar,
        ScalarConstructor,
        ScalarConsts,
    };

    impl<Num: Axis + NumCast, Int: Integer + Clone + ToPrimitive> Scalar<Num> for Ratio<Int> {
        fn scalar(&self) -> Num {
            use crate::core::option::Option::Some;
            let (up, down) = self.clone().into_raw();
            match (Num::from(up), Num::from(down)) {
                (Some(up), Some(down)) => if down != Num::ZERO {up / down} else { Num::NAN },
                _ => Num::NAN,
            }
        }
    }

    impl<Num: Axis + NumCast, Int: Integer + Clone + ToPrimitive> Scalar<Num> for &Ratio<Int> {
        fn scalar(&self) -> Num {
            use crate::core::option::Option::Some;
            let (up, down) = (*self).clone().into_raw();
            match (Num::from(up), Num::from(down)) {
                (Some(up), Some(down)) => if down != Num::ZERO {up / down} else { Num::NAN },
                _ => Num::NAN,
            }
        }
    }

    impl<Num: Axis + NumCast, Int: Integer + Clone + ToPrimitive> Scalar<Num> for &mut Ratio<Int> {
        fn scalar(&self) -> Num {
            use crate::core::option::Option::Some;
            let (up, down) = (*self).clone().into_raw();
            match (Num::from(up), Num::from(down)) {
                (Some(up), Some(down)) => if down != Num::ZERO {up / down} else { Num::NAN },
                _ => Num::NAN,
            }
        }
    }

    // impl<Num: Axis + ToPrimitive, Int: Integer> ScalarConstructor<Num> for crate::core::option::Option<Ratio<Int>>
    // where Ratio<Int>: NumCast
    // {
    //     fn new_scalar(scalar: Num) -> crate::core::option::Option<Ratio<Int>> {
    //         <Ratio<Int>>::from(scalar)
    //     }
    // }

    impl<Num: Axis + NumCast + FloatCore, Int: Integer + Signed + Bounded + NumCast + Clone> ScalarConstructor<Num> for crate::core::option::Option<Ratio<Int>>
    where Ratio<Int>: NumCast
    {
        fn new_scalar(axis: Num) -> crate::core::option::Option<Ratio<Int>> {
            <Ratio<Int>>::approximate_float(axis)
        }
    }

    impl<Num: Axis + NumCast, Int: Integer + Clone + ToPrimitive + ConstOne + ConstZero> ScalarConsts<Num> for Ratio<Int> {
        const ZERO: Self = <Self as ConstZero>::ZERO;
        const ONE: Self = <Self as ConstOne>::ONE;
        const NAN: Self = Ratio::new_raw(Int::ONE, Int::ZERO);
    }
}

#[cfg(feature = "num-bigint")]
mod num_bigint_impl {
    use crate::core::option::Option;
    use crate::num_bigint::{
        BigUint,
        BigInt,
    };
    use crate::num_traits::{
        ToPrimitive,
        FromPrimitive,
    };
    use crate::{
        Scalar,
        ScalarConstructor,
    };
    #[cfg(feature = "std")]
    use crate::structs::Std;

    impl Scalar<f32> for BigInt {
        #[inline] fn scalar(&self) -> f32 { self.to_f32().unwrap() } // Can not return `None`
    }

    impl ScalarConstructor<f32> for Option<BigInt> {
        #[inline] fn new_scalar(axis: f32) -> Self { BigInt::from_f32(axis) }
    }

    impl Scalar<f64> for BigInt {
        #[inline] fn scalar(&self) -> f64 { self.to_f64().unwrap() } // Can not return `None`
    }

    impl ScalarConstructor<f64> for Option<BigInt> {
        #[inline] fn new_scalar(axis: f64) -> Self { BigInt::from_f64(axis) }
    }

    #[cfg(feature = "std")]
    impl Scalar<Std<f32>> for BigInt {
        #[inline] fn scalar(&self) -> Std<f32> { Std(self.to_f32().unwrap()) } // Can not return `None`
    }

    #[cfg(feature = "std")]
    impl ScalarConstructor<Std<f32>> for Option<BigInt> {
        #[inline] fn new_scalar(axis: Std<f32>) -> Self { BigInt::from_f32(axis.0) }
    }

    #[cfg(feature = "std")]
    impl Scalar<Std<f64>> for BigInt {
        #[inline] fn scalar(&self) -> Std<f64> { Std(self.to_f64().unwrap()) } // Can not return `None`
    }

    #[cfg(feature = "std")]
    impl ScalarConstructor<Std<f64>> for Option<BigInt> {
        #[inline] fn new_scalar(axis: Std<f64>) -> Self { BigInt::from_f64(axis.0) }
    }

    impl Scalar<f32> for BigUint {
        #[inline] fn scalar(&self) -> f32 { self.to_f32().unwrap() } // Can not return `None`
    }

    impl ScalarConstructor<f32> for Option<BigUint> {
        #[inline] fn new_scalar(axis: f32) -> Self { BigUint::from_f32(axis) }
    }

    impl Scalar<f64> for BigUint {
        #[inline] fn scalar(&self) -> f64 { self.to_f64().unwrap() } // Can not return `None`
    }

    impl ScalarConstructor<f64> for Option<BigUint> {
        #[inline] fn new_scalar(axis: f64) -> Self { BigUint::from_f64(axis) }
    }

    #[cfg(feature = "std")]
    impl Scalar<Std<f32>> for BigUint {
        #[inline] fn scalar(&self) -> Std<f32> { Std(self.to_f32().unwrap()) } // Can not return `None`
    }

    #[cfg(feature = "std")]
    impl ScalarConstructor<Std<f32>> for Option<BigUint> {
        #[inline] fn new_scalar(axis: Std<f32>) -> Self { BigUint::from_f32(axis.0) }
    }

    #[cfg(feature = "std")]
    impl Scalar<Std<f64>> for BigUint {
        #[inline] fn scalar(&self) -> Std<f64> { Std(self.to_f64().unwrap()) } // Can not return `None`
    }

    #[cfg(feature = "std")]
    impl ScalarConstructor<Std<f64>> for Option<BigUint> {
        #[inline] fn new_scalar(axis: Std<f64>) -> Self { BigUint::from_f64(axis.0) }
    }

}
