
use super::*;
use crate::core::iter::{
    Iterator,
    IntoIterator,
};

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Adds all the quaternions in an iterator.
/// 
/// Returns the origin quaternion if the iterator is empty.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::{sum, add};
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [3.0, -2.0, 1.0, -4.0];
/// let c: [f32; 4] = [1.0, 1.3, 2.2, 3.1];
/// 
/// let normal: [f32; 4] = add::<f32, [f32; 4]>(&add::<f32, [f32; 4]>(&a, &b), &c);
/// let iter: [f32; 4] = sum::<f32, [f32; 4]>( [a, b, c] );
/// 
/// assert_eq!(normal, iter);
/// ```
pub fn sum<Num, Out>(iter: impl IntoIterator<Item: Quaternion<Num>>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut sum = (Num::ZERO, [Num::ZERO; 3]);
    for quaternion in iter {
        sum = add(sum, quaternion);
    }
    Out::from_quat(sum)
}

// const PRODUCT_MARGIN: usize = 0xFFFFFFF;
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Multiplies all the quaternions in an iterator.
/// 
/// Returns the identity quaternion if the iterator is empty.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::{product, mul};
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [3.0, -2.0, 1.0, -4.0];
/// let c: [f32; 4] = [1.0, 1.3, 2.2, 3.1];
/// 
/// let normal: [f32; 4] = mul::<f32, [f32; 4]>(&mul::<f32, [f32; 4]>(&a, &b), &c);
/// let iter: [f32; 4] = product::<f32, [f32; 4]>( [a, b, c] );
/// 
/// assert_eq!(normal, iter);
/// ```
pub fn product<Num, Out>(iter: impl IntoIterator<Item: Quaternion<Num>>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let mut iter = iter.into_iter();
    let mut product = match iter.next() {
        Option::Some(ok) => Q::<Num>::from_quat(ok),
        Option::None => return identity(),
    };
    // if Iterator::size_hint(&iter).0 > PRODUCT_MARGIN
    // || match Iterator::size_hint(&iter).1 {
    //     Option::Some(some) => some > PRODUCT_MARGIN << 1,
    //     Option::None => true,
    // } {
        for quaternion in iter {
            product = mul(product, quaternion);
            if eq(product, ()) {
                return Out::from_quat(());
            }
        }
    // } else {
    //     for quaternion in iter {
    //         sum = mul(&sum, &quaternion);
    //     }
    // }
    Out::from_quat(product)
}
