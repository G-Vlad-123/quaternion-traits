
#[cfg(feature = "num-complex")]
mod num_complex_impl {
    impl<Num: crate::Axis + crate::num_traits::Float> crate::Complex<Num> for crate::num_complex::Complex<Num> {
        #[inline] fn real(&self) -> Num {
            self.re
        }
        
        #[inline] fn imaginary(&self) -> Num {
            self.im
        }
    }
}
