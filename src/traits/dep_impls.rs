
#[cfg(feature = "num-complex")]
mod num_complex_impl {
    use crate::num_complex::Complex;

    impl<Num: crate::Axis + crate::num_traits::Float> crate::Complex<Num> for Complex<Num> {
        #[inline] fn real(&self) -> Num {
            self.re
        }
        
        #[inline] fn imaginary(&self) -> Num {
            self.im
        }
    }
}
