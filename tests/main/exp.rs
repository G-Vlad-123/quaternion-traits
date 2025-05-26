
use super::*;

// twice as slow as the others
fn exp_1<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: traits::Axis,
    Out: QuaternionConstructor<Num>,
{
    let vec: Q<Num> = quat::vector_part(&quaternion);
    let (sin, cos) = quat::abs::<Num, Num>(&vec).sin_cos();
    quat::scale::<Num, Out>(
        quat::add::<Num, Q<Num>>(
            quat::scale::<Num, Q<Num>>(
                quat::normalize::<Num, Q<Num>>(&vec),
                sin
            ),
            (cos, ())
        ),
        quaternion.r().exp(),
    )
}

// fastest one by a tiny beat in release
// winner as of 0.2.0.2
fn exp_2<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: traits::Axis,
    Out: QuaternionConstructor<Num>,
{
    let len = (quaternion.i()*quaternion.i() + quaternion.j()*quaternion.j() + quaternion.k()*quaternion.k()).sqrt();
    let (sin, cos) = len.sin_cos();
    let r_exp = quaternion.r().exp();
    let sin_over_len = sin / len;
    let unreal_factor = sin_over_len * r_exp;
    Out::new_quat(
        (quaternion.r().mul_add(sin_over_len, cos)) * r_exp,
        quaternion.i() * unreal_factor,
        quaternion.j() * unreal_factor,
        quaternion.k() * unreal_factor,
    )
}

// least to say about, B tier
fn exp_3<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: traits::Axis,
    Out: QuaternionConstructor<Num>,
{
    let len = (quaternion.i()*quaternion.i() + quaternion.j()*quaternion.j() + quaternion.k()*quaternion.k()).sqrt();
    let (sin, cos) = len.sin_cos();
    let r_exp = quaternion.r().exp();
    let sin_over_len = sin / len;
    let unreal_factor = len * r_exp;
    Out::new_quat(
        (quaternion.r().mul_add(sin_over_len, cos)) * r_exp,
        quaternion.i() * unreal_factor,
        quaternion.j() * unreal_factor,
        quaternion.k() * unreal_factor,
    )
}

// fastest one by a tiny bit in debug
// uses least variables (may or may not affect release mode)
fn exp_4<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: traits::Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut len = (quaternion.i()*quaternion.i() + quaternion.j()*quaternion.j() + quaternion.k()*quaternion.k()).sqrt();
    let (mut sin, cos) = len.sin_cos();
    let r_exp = quaternion.r().exp();
    len = sin / len; // saving some memory, a better name would be `frac_sin_len` or `sin_over_len`
    sin = len * r_exp; // saving some meory, a better name would be `unreal_factor`
    Out::new_quat(
        (quaternion.r().mul_add(len, cos)) * r_exp,
        quaternion.i() * sin, // we are multiplying by `unreal_factor`, not sin.
        quaternion.j() * sin, // we are multiplying by `unreal_factor`, not sin.
        quaternion.k() * sin, // we are multiplying by `unreal_factor`, not sin.
    )
}

#[test]
#[ignore]
fn exp_speed_check() {
    use ::std::time::Duration;

    #[inline(never)]
    fn exp_1_run() -> Duration {
        print!("exp_1: ");
        timer! {
            run {
                for quat in f32_quats() {
                    let _ = exp_1::<f32, [f32; 4]>(quat);
                }
            },
            silent,
            repeat 50,
        }
    }

    #[inline(never)]
    fn exp_2_run() -> Duration {
        print!("exp_2: ");
        timer! {
            run {
                for quat in f32_quats() {
                    let _ = exp_2::<f32, [f32; 4]>(quat);
                }
            },
            silent,
            repeat 50,
        }
    }

    #[inline(never)]
    fn exp_3_run() -> Duration {
        print!("exp_3: ");
        timer! {
            run {
                for quat in f32_quats() {
                    let _ = exp_3::<f32, [f32; 4]>(quat);
                }
            },
            silent,
            repeat 50,
        }
    }

    #[inline(never)]
    fn exp_4_run() -> Duration {
        print!("exp_4: ");
        timer! {
            run {
                for quat in f32_quats() {
                    let _ = exp_4::<f32, [f32; 4]>(quat);
                }
            },
            silent,
            repeat 50,
        }
    }

    // if not for never inline functions the 2nd and 3rd function (no matter the order)
    // where more then twice as slow as the first
    exp_1_run(); // slow
    exp_2_run(); // fastest (by a tiny bit)
    exp_3_run(); // fast
    exp_4_run(); // fast (but hopefully more memory efficient)

    panic!()
}
