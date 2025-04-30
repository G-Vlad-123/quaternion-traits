
#![allow(unused_imports)]
#![allow(dead_code)]

use quaternion_traits::*;
use core::assert;
use core::assert_eq;

/// made my own for the sake of testing in both release and dev mode
#[cfg(feature = "std")]
macro_rules! timer {
    ( run $code:block $(,)? ) => {
        {
            let start = ::std::time::Instant::now();
            $code
            let finish = ::std::time::Instant::now();
            let duration = finish.duration_since(start);
            ::std::dbg!(duration)
        }
    };
    ( run $code:block, silent $(,)? ) => {
        {
            let start = ::std::time::Instant::now();
            $code
            let finish = ::std::time::Instant::now();
            finish.duration_since(start)
        }
    };
    ( run $code:block, repeat $repeat:expr $(,)? ) => {
        {
            let mut avrege = ::std::time::Duration::ZERO;
            for _ in 0u32..$repeat {
                avrege += timer!{ run $code }
            }
            avrege /= $repeat;
            ::std::dbg!(avrege)
        }
    };
    ( run $code:block, repeat $repeat:expr, silent $(,)? ) => {
        {
            let mut avrege = ::std::time::Duration::ZERO;
            for _ in 0u32..$repeat {
                avrege += timer!{ run $code }
            }
            avrege /= $repeat;
            avrege
        }
    };
    ( run $code:block, silent, repeat $repeat:expr $(,)? ) => {
        {
            let mut avrege = ::std::time::Duration::ZERO;
            for _ in 0u32..$repeat {
                avrege += timer!{ run $code, silent }
            }
            avrege /= $repeat;
            ::std::dbg!(avrege)
        }
    };
    ( run $code:block, silent, repeat $repeat:expr, silent $(,)? ) => {
        {
            let mut avrege = ::std::time::Duration::ZERO;
            for _ in 0u32..$repeat {
                avrege += timer!{ run $code, silent }
            }
            avrege /= $repeat;
            avrege
        }
    };
}

#[cfg(feature = "std")]
use std::println;

const F32S: [f32; 61] = [
    0.0,
    1.0,
    f32::EPSILON,
    core::f32::consts::E,
    core::f32::consts::FRAC_1_PI,
    core::f32::consts::FRAC_1_SQRT_2,
    core::f32::consts::FRAC_2_PI,
    core::f32::consts::FRAC_2_SQRT_PI,
    core::f32::consts::FRAC_PI_2,
    core::f32::consts::FRAC_PI_3,
    core::f32::consts::FRAC_PI_4,
    core::f32::consts::FRAC_PI_6,
    core::f32::consts::FRAC_PI_8,
    core::f32::consts::LN_2,
    core::f32::consts::LN_10,
    core::f32::consts::LOG2_10,
    core::f32::consts::LOG2_E,
    core::f32::consts::LOG10_2,
    core::f32::consts::LOG10_E,
    core::f32::consts::PI,
    core::f32::consts::SQRT_2,
    core::f32::consts::TAU,
    1e8 * f32::EPSILON,
    1e8 * core::f32::consts::E,
    1e8 * core::f32::consts::FRAC_1_PI,
    1e8 * core::f32::consts::FRAC_1_SQRT_2,
    1e8 * core::f32::consts::FRAC_2_PI,
    1e8 * core::f32::consts::FRAC_2_SQRT_PI,
    1e8 * core::f32::consts::FRAC_PI_2,
    1e8 * core::f32::consts::FRAC_PI_3,
    1e8 * core::f32::consts::FRAC_PI_4,
    1e8 * core::f32::consts::FRAC_PI_6,
    1e8 * core::f32::consts::FRAC_PI_8,
    1e8 * core::f32::consts::LN_2,
    1e8 * core::f32::consts::LN_10,
    1e8 * core::f32::consts::LOG2_10,
    1e8 * core::f32::consts::LOG2_E,
    1e8 * core::f32::consts::LOG10_2,
    1e8 * core::f32::consts::LOG10_E,
    1e8 * core::f32::consts::PI,
    1e8 * core::f32::consts::SQRT_2,
    1e8 * core::f32::consts::TAU,
    // 1e-8 * f32::EPSILON, // <--- Only thing that breaks sqrt (it's square is too small)
    1e-8 * core::f32::consts::E,
    1e-8 * core::f32::consts::FRAC_1_PI,
    1e-8 * core::f32::consts::FRAC_1_SQRT_2,
    1e-8 * core::f32::consts::FRAC_2_PI,
    1e-8 * core::f32::consts::FRAC_2_SQRT_PI,
    1e-8 * core::f32::consts::FRAC_PI_2,
    1e-8 * core::f32::consts::FRAC_PI_3,
    1e-8 * core::f32::consts::FRAC_PI_4,
    1e-8 * core::f32::consts::FRAC_PI_6,
    1e-8 * core::f32::consts::FRAC_PI_8,
    1e-8 * core::f32::consts::LN_2,
    1e-8 * core::f32::consts::LN_10,
    1e-8 * core::f32::consts::LOG2_10,
    1e-8 * core::f32::consts::LOG2_E,
    1e-8 * core::f32::consts::LOG10_2,
    1e-8 * core::f32::consts::LOG10_E,
    1e-8 * core::f32::consts::PI,
    1e-8 * core::f32::consts::SQRT_2,
    1e-8 * core::f32::consts::TAU,
];

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct F32_Quats {
    r: usize,
    i: usize,
    j: usize,
    k: usize,
}

impl F32_Quats {
    const fn new() -> Self {
        F32_Quats { r: 0, i: 0, j: 0, k: 0 }
    }
}

#[inline(always)]
fn f32_quats() -> F32_Quats {
    F32_Quats::new()
}

impl core::iter::ExactSizeIterator for F32_Quats {}
impl core::iter::FusedIterator for F32_Quats {}
impl core::iter::Iterator for F32_Quats {
    type Item = [f32; 4];

    fn next(&mut self) -> core::option::Option<[f32; 4]> {
        use core::option::Option;
        if self.k == 60 { return Option::None }
        if self.j == 60 { self.j = 0; self.k += 1 }
        if self.i == 60 { self.i = 0; self.j += 1 }
        if self.r == 60 { self.r = 0; self.i += 1 }
        let out = [
            F32S[self.r],
            F32S[self.i],
            F32S[self.j],
            F32S[self.k],
        ];
        self.r += 1;
        Option::Some(out)
    }
}

#[test]
#[ignore]
fn sqrt_accuracy() {
    let mut mul_result: [f32; 4];
    let mut sqrt_result: [f32; 4];
    for quat in F32_Quats::new() {
        if quat == quat::origin::<f32, [f32; 4]>() { continue }
        mul_result = quat::mul::<f32, [f32; 4]>(&quat, &quat);
        sqrt_result = quat::sqrt::<f32, [f32; 4]>(&mul_result);
        let error: f32 = {
            let original_dist = quat::abs::<f32, f32>(&quat);
            let checking_dist = quat::abs::<f32, f32>(&sqrt_result);
            f32::abs(original_dist/checking_dist - 1.0)
        };
        assert!(
            error < 0.005,
            "{quat:?}\n{mul_result:?}\n{sqrt_result:?}\nerror: {error:?}"
        );
    }
}

#[test]
#[ignore]
#[cfg(feature = "std")]
#[allow(unexpected_cfgs)]
fn timing_pow_f_vs_sqrt() {
    let pow_f_average = timer! {
        run {
            for quat in f32_quats() {
                let _ = quat::pow_f::<f32, [f32; 4]>(quat, 0.5);
            }
        },
        repeat 5,
    };

    println!();
    
    let sqrt_average = timer! {
        run {
            for quat in f32_quats() {
                let _ = quat::sqrt::<f32, [f32; 4]>(quat);
            }
        },
        repeat 5,
    };

    println!("
    dev:
        pow_f(q, 0.5) takes ~`7.3`s to run on all F32_Quats
        sqrt(q)       takes ~`3.8`s to run on all F32_Quats

    release:
        pow_f(q, 0.5) takes ~`263`ms to run on all F32_Quats
        sqrt(q)       takes ~`41.5`ms to run on all F32_Quats
    ");
    assert!( sqrt_average < pow_f_average );
}
