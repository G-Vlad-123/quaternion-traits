
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
