
use super::*;

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion representation from another one.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::convert_quat;
/// 
/// let from: [u8; 4] = [1, 2, 3, 4];
/// 
/// let float = convert_quat::<f32, [f32; 4]>(from);
/// assert_eq!( float, [1.0, 2.0, 3.0, 4.0] );
/// 
/// let int =   convert_quat::<f64, (i64, i32, i16, i8)>(from);
/// assert_eq!( int, (1, 2, 3, 4) );
/// 
/// let unit =  convert_quat::<f32, ()>(from);
/// assert_eq!( unit, () );
/// 
/// let tup =   convert_quat::<f64, (u8, [u8; 3])>(from);
/// assert_eq!( tup, (1, [2, 3, 4]) );
/// ```
pub fn convert_quat<Num, To>(from: impl Quaternion<Num>) -> To
where 
    Num: Axis,
    To: QuaternionConstructor<Num>,
{
    To::from_quat(from)
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a vector representation.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::from_vector;
/// 
/// let vector: [f32; 3] = [3.14, 2.71, 1.23];
/// let quat: [f32; 4] = from_vector::<f32, [f32; 4]>(&vector);
/// 
/// assert_eq!( quat, [0.0, 3.14, 2.71, 1.23] );
/// ```
pub fn from_vector<Num, Out>(vector: impl Vector<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    QuaternionConstructor::new_quat(
        Num::ZERO,
        vector.x(),
        vector.y(),
        vector.z(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a complex number representation.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::from_complex;
/// 
/// let complex: [f32; 2] = [3.14, 2.71];
/// let quat: [f32; 4] = from_complex::<f32, [f32; 4]>(&complex);
/// 
/// assert_eq!( quat, [3.14, 2.71, 0.0, 0.0] );
/// ```
pub fn from_complex<Num, Out>(complex: impl Complex<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    QuaternionConstructor::new_quat(
        complex.real(),
        complex.imaginary(),
        Num::ZERO,
        Num::ZERO,
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a scalar value.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::from_scalar;
/// 
/// let scalar: f32 = 3.14;
/// let quat: [f32; 4] = from_scalar::<f32, [f32; 4]>(&scalar);
/// 
/// assert_eq!( quat, [3.14, 0.0, 0.0, 0.0] );
/// ```
pub fn from_scalar<Num, Out>(complex: impl Scalar<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    QuaternionConstructor::new_quat(
        complex.scalar(),
        Num::ZERO,
        Num::ZERO,
        Num::ZERO,
    )
}

// TODO add is_near

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a quaternion from a rotation.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::from_rotation;
/// use core::f32::consts::PI;
/// 
/// let rotation: [f32; 3] = [PI, 0.0, 0.0];
/// let quat: [f32; 4] = from_rotation::<f32, [f32; 4]>(&rotation);
/// 
/// assert_eq!( quat, [-4.371139e-8, 1.0, 0.0, 0.0] );
/// ```
pub fn from_rotation<Num, Out>(rotation: impl Rotation<Num>) -> Out
where 
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let cos_r: Num = Num::cos(rotation.roll() / (Num::ONE + Num::ONE));
    let sin_r: Num = Num::sin(rotation.roll() / (Num::ONE + Num::ONE));
    let cos_p: Num = Num::cos(rotation.pitch() / (Num::ONE + Num::ONE));
    let sin_p: Num = Num::sin(rotation.pitch() / (Num::ONE + Num::ONE));
    let cos_y: Num = Num::cos(rotation.yaw() / (Num::ONE + Num::ONE));
    let sin_y: Num = Num::sin(rotation.yaw() / (Num::ONE + Num::ONE));
    QuaternionConstructor::new_quat(
        cos_r * cos_p * cos_y + sin_r * sin_p * sin_y,
        sin_r * cos_p * cos_y + cos_r * sin_p * sin_y,
        cos_r * sin_p * cos_y + sin_r * cos_p * sin_y,
        cos_r * cos_p * sin_y + sin_r * sin_p * cos_y,
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates a quaternion using the given polar form.
/// 
/// Returns [`None`](Option::None) if the absolute value of `unit_vec`
/// is not near [`Num::ONE`](Axis::ONE).
pub fn from_polar_form<Num, Out>(abs: impl Scalar<Num>, angle: impl Scalar<Num>, unit_vec: impl Vector<Num>) -> Option<Out>
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    if (unit_vec.x() * unit_vec.x() + unit_vec.y() * unit_vec.y() + unit_vec.z() * unit_vec.z() - Num::ONE).abs() >= Num::ERROR * Num::ERROR {
        return Option::None;
    }
    let (sin, cos) = angle.scalar().sin_cos();
    Option::Some( Out::new_quat(
        abs.scalar() * cos,
        abs.scalar() * sin * unit_vec.x(),
        abs.scalar() * sin * unit_vec.y(),
        abs.scalar() * sin * unit_vec.z(),
    ) )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Calculates a quaternion using the given polar form.
/// 
/// # Safety
/// Make sure the absolute value of `unit_vec` is near [Num::ONE](Axis::ONE).
pub unsafe fn from_polar_form_unchecked<Num, Out>(abs: impl Scalar<Num>, angle: impl Scalar<Num>, unit_vec: impl Vector<Num>) -> Out
where
    Num: Axis,
    Out: QuaternionConstructor<Num>,
{
    let (sin, cos) = angle.scalar().sin_cos();
    Out::new_quat(
        abs.scalar() * cos,
        abs.scalar() * sin * unit_vec.x(),
        abs.scalar() * sin * unit_vec.y(),
        abs.scalar() * sin * unit_vec.z(),
    )
}

/// Cosntructs a quaternion from a 2x2 complex matrix.
/// 
/// This acts like the inverse of the [`to_matrix_2`] function,
/// therefor it checks if it's formula works.
/// 
/// If you want to skip this check you can just take in order all
/// the elements of the top row and give them their respective place
/// in the quaternion.
/// 
///     # "
///     M = [a + bi, c + di]
///         [  ..  ,   ..  ]
/// 
///         =>
/// 
///     q = a + bi + cj + dk
///     # ";
/// 
/// # Example 
/// ```
/// use quaternion_traits::quat::from_matrix_2;
/// 
/// // valid conversion
/// let valid_matrix = [
///     [(1, 2), (3, 4)],
///     [(-3,4), (1,-2)],
/// ];
/// 
/// assert_eq!(
///     //              vvv number used when doing the conversion
///     //                    vvvvvvvv complex number contained by teh matrix
///     //                                vvvvvv output quaternion type
///     from_matrix_2::<f32, (i16, i16), [f32; 4]>(valid_matrix),
///     Some( [1.0, 2.0, 3.0, 4.0] )
/// );
/// 
/// // invalid conversion
/// let invalid_matrix = [
///     [(1, 2), (3, 4)],
///     [(5, 6), (7, 8)],
/// ];
/// 
/// assert_eq!(
///     from_matrix_2::<f32, (i16, i16), [f32; 4]>(invalid_matrix),
///     None
/// );
/// ```
pub fn from_matrix_2<Num, Elem, Out>(matrix: impl Matrix<Elem, 2>) -> Option<Out>
where 
    Num: Axis,
    Elem: Complex<Num>,
    Out: QuaternionConstructor<Num>,
{
    if matrix.get_unchecked(0, 0).real() != matrix.get_unchecked(1, 1).real()
    || matrix.get_unchecked(0, 0).imaginary() != -matrix.get_unchecked(1, 1).imaginary()
    || matrix.get_unchecked(1, 0).real() != -matrix.get_unchecked(0, 1).real()
    || matrix.get_unchecked(1, 0).imaginary() != matrix.get_unchecked(0, 1).imaginary()
    {
        return Option::None;
    }
    Option::Some( Out::new_quat(
        matrix.get_unchecked(0, 0).real(),
        matrix.get_unchecked(0, 0).imaginary(),
        matrix.get_unchecked(0, 1).real(),
        matrix.get_unchecked(0, 1).imaginary(),
    ) )
}

/// Cosntructs a quaternion from a 2x2 complex matrix.
/// 
/// Does the same thing as [`from_matrix_2`] without checking if
/// the matrix is a valid representation for a quaternion.
/// 
/// # Example 
/// ```
/// use quaternion_traits::quat::from_matrix_2_unchecked;
/// 
/// // valid conversion
/// let valid_matrix = [
///     [(1, 2), (3, 4)],
///     [(-3,4), (1,-2)],
/// ];
/// 
/// assert_eq!(
///     //              vvv number used when doing the conversion
///     //                    vvvvvv complex number contained by teh matrix
///     //                              vvvvvv output quaternion type
///     from_matrix_2_unchecked::<f32, (i16, i16), [f32; 4]>(valid_matrix),
///     [1.0, 2.0, 3.0, 4.0]
/// );
/// 
/// // invalid conversion
/// let invalid_matrix = [
///     [(1, 2), (3, 4)],
///     [(5, 6), (7, 8)],
/// ];
/// 
/// assert_eq!(
///     from_matrix_2_unchecked::<f32, (i16, i16), [f32; 4]>(invalid_matrix),
///     [1.0, 2.0, 3.0, 4.0]
/// );
/// ```
pub fn from_matrix_2_unchecked<Num, Elem, Out>(matrix: impl Matrix<Elem, 2>) -> Out
where 
    Num: Axis,
    Elem: Complex<Num>,
    Out: QuaternionConstructor<Num>,
{
    Out::new_quat(
        matrix.get_unchecked(0, 0).real(),
        matrix.get_unchecked(0, 0).imaginary(),
        matrix.get_unchecked(0, 1).real(),
        matrix.get_unchecked(0, 1).imaginary(),
    )
}

/// Cosntructs a quaternion from a 3x3 matrix (DCM).
/// 
/// Note: There are quite a few ways to turn a 3x3 matrix into
/// a quaternion, this one uses 4 formulas and choses one based on
/// the inputs, for the most general use case.
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn from_matrix_3<Num, Elem, Out>(matrix: impl Matrix<Elem, 3>) -> Out
where 
    Num: Axis,
    Elem: Scalar<Num>,
    Out: QuaternionConstructor<Num>,
{
    // Adapted from: http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/index.htm

    let two: Num = Num::ONE + Num::ONE;
    let r: Num =   matrix.get_unchecked(0, 0).scalar() + matrix.get_unchecked(1, 1).scalar() + matrix.get_unchecked(2, 2).scalar();
    let i: Num =   matrix.get_unchecked(0, 0).scalar() - matrix.get_unchecked(1, 1).scalar() - matrix.get_unchecked(2, 2).scalar();
    let j: Num = - matrix.get_unchecked(0, 0).scalar() + matrix.get_unchecked(1, 1).scalar() - matrix.get_unchecked(2, 2).scalar();
    let k: Num = - matrix.get_unchecked(0, 0).scalar() - matrix.get_unchecked(1, 1).scalar() + matrix.get_unchecked(2, 2).scalar();
    let mut largest: Num = r;
    if i > largest { largest = i }
    if j > largest { largest = j }
    if k > largest { largest = k }

    if largest == r {
        largest = (largest + Num::ONE).sqrt();
        return Out::new_quat(
            largest / two,
            (matrix.get_unchecked(1, 2).scalar() - matrix.get_unchecked(2, 1).scalar()) / (largest * two),
            (matrix.get_unchecked(2, 0).scalar() - matrix.get_unchecked(0, 2).scalar()) / (largest * two),
            (matrix.get_unchecked(0, 1).scalar() - matrix.get_unchecked(1, 0).scalar()) / (largest * two),
        )
    }

    if largest == i {
        largest = (largest + Num::ONE).sqrt();
        return Out::new_quat(
            (matrix.get_unchecked(1, 2).scalar() - matrix.get_unchecked(2, 1).scalar()) / (largest * two),
            largest / two,
            (matrix.get_unchecked(0, 1).scalar() - matrix.get_unchecked(1, 0).scalar()) / (largest * two),
            (matrix.get_unchecked(2, 0).scalar() - matrix.get_unchecked(0, 2).scalar()) / (largest * two),
        )
    }

    if largest == j {
        largest = (largest + Num::ONE).sqrt();
        return Out::new_quat(
            (matrix.get_unchecked(2, 0).scalar() - matrix.get_unchecked(0, 2).scalar()) / (largest * two),
            (matrix.get_unchecked(0, 1).scalar() - matrix.get_unchecked(1, 0).scalar()) / (largest * two),
            largest / two,
            (matrix.get_unchecked(1, 2).scalar() - matrix.get_unchecked(2, 1).scalar()) / (largest * two),
        )
    }

    // largest == k 
    largest = (largest + Num::ONE).sqrt();
    return Out::new_quat(
        (matrix.get_unchecked(0, 1).scalar() - matrix.get_unchecked(1, 0).scalar()) / (largest * two),
        (matrix.get_unchecked(2, 0).scalar() - matrix.get_unchecked(0, 2).scalar()) / (largest * two),
        (matrix.get_unchecked(1, 2).scalar() - matrix.get_unchecked(2, 1).scalar()) / (largest * two),
        largest / two,
    )
    
}

/// Cosntructs a quaternion from a 4x4 matrix.
/// 
/// Note: There are many ways to turn a 4x4 matrix into
/// a quaternion, this one just does the same thing as
/// [`from_matrix_3`] but with the first 3 rows and columbs
/// of this matrix instead.
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn from_matrix_4<Num, Elem, Out>(matrix: impl Matrix<Elem, 4>) -> Out
where 
    Num: Axis,
    Elem: Scalar<Num>,
    Out: QuaternionConstructor<Num>,
{
    // Adapted from: http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/index.htm
    from_matrix_3([
        [matrix.get_unchecked(0, 0).scalar(), matrix.get_unchecked(0, 1).scalar(), matrix.get_unchecked(0, 2).scalar()],
        [matrix.get_unchecked(1, 0).scalar(), matrix.get_unchecked(1, 1).scalar(), matrix.get_unchecked(1, 2).scalar()],
        [matrix.get_unchecked(2, 0).scalar(), matrix.get_unchecked(2, 1).scalar(), matrix.get_unchecked(2, 2).scalar()],
    ])
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a complex number representation from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_vector;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// let vector: [f32; 3] = to_vector::<f32, [f32; 3]>(&quat);
/// 
/// assert_eq!( vector, [3.4, 5.6, 7.8] );
/// ```
pub fn to_vector<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where
    Num: Axis,
    Out: VectorConstructor<Num>,
{
    Out::new_vector(
        quaternion.i(),
        quaternion.j(),
        quaternion.k(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a complex number representation from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_complex;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// let complex: [f32; 2] = to_complex::<f32, [f32; 2]>(&quat);
/// 
/// assert_eq!( complex, [1.2, 3.4] );
/// ```
pub fn to_complex<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ComplexConstructor<Num>,
{
    Out::new_complex(
        quaternion.r(),
        quaternion.i(),
    )
}

#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a scalar value from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_scalar;
/// 
/// let quat: [f32; 4] = [1.2, 3.4, 5.6, 7.8];
/// let scalar = to_scalar::<f32, f32>(&quat);
/// 
/// assert_eq!( scalar, 1.2 );
/// ```
pub fn to_scalar<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: ScalarConstructor<Num>,
{
    Out::new_scalar(
        quaternion.r(),
    )
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Constructs a rotation representation from a quaternion.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_rotation;
/// use core::f32::consts::PI;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, 0.0, 0.0];
/// let rotation: [f32; 3] = to_rotation::<f32, [f32; 3]>(&quat);
/// 
/// assert_eq!( rotation, [PI, 0.0, 0.0] );
/// ```
pub fn to_rotation<Num, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Out: RotationConstructor<Num>
{
    let quat: Q<Num> = norm(quaternion);

    let two = Num::ONE + Num::ONE;
    // here I misspelled 'pitch' but it's funny so I kept it
    let peach = two * (quat.r() * quat.j() - quat.i() * quat.k());

    if peach > Num::ONE - Num::ERROR {
        return RotationConstructor::new_rotation(
            two * Num::atan2(quat.i(), quat.r()),
            Num::TAU / (two + two),
            Num::ZERO,
        )
    }

    if peach < Num::ERROR - Num::ONE {
        return RotationConstructor::new_rotation(
            -two * Num::atan2(quat.i(), quat.r()),
            -Num::TAU / (two + two),
            Num::ZERO,
        )
    }

    let roll: Num = Num::atan2(
        two * (quat.r() * quat.i() + quat.j() * quat.k()),
        Num::ONE - two * ( quat.i() * quat.i() + quat.j() * quat.j())
    );
    let pitch: Num = Num::asin(peach);
    let yaw: Num = Num::atan2(
        two * (quat.r() * quat.k() + quat.i() * quat.j()),
        Num::ONE - two * ( quat.j() * quat.j() + quat.k() * quat.k() )
    );
    RotationConstructor::new_rotation(roll, pitch, yaw)
}

/// Turns this quaternion into a 2x2 Matrix
/// 
/// Note: This uses the first representation from the
/// [wiki](https://en.wikipedia.org/wiki/Quaternion#Representation_as_complex_2_%C3%97_2_matrices)
/// on quaternions.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_matrix_2;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// 
/// let matrix: [[(i16, i16); 2]; 2] = to_matrix_2::<f32, _, _>(quat);
/// 
/// assert_eq!(
///     matrix,
///     [
///         [(1, 2), (3, 4)],
///         [(-3,4), (1,-2)],
///     ]
/// )
/// ```
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn to_matrix_2<Num, Complex, Matrix>(quaternion: impl Quaternion<Num>) -> Matrix
where 
    Num: Axis,
    Complex: ComplexConstructor<Num>,
    Matrix: MatrixConstructor<Complex, 2>,
{
    Matrix::new_matrix(
        [
            [
                Complex::new_complex(quaternion.r(), quaternion.i()),
                Complex::new_complex(quaternion.j(), quaternion.k()),
            ],
            [
                Complex::new_complex(-quaternion.j(), quaternion.k()),
                Complex::new_complex(quaternion.r(), -quaternion.i()),
            ],
        ]
    )
}

/// Turns this quaternion into a 3x3 Matrix. (DCM)
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn to_matrix_3<Num, Elem, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Elem: ScalarConstructor<Num>,
    Out: MatrixConstructor<Elem, 3>,
{
    let q = quaternion;
    let two = Num::ONE + Num::ONE;
    Out::new_matrix([
        [
            Elem::new_scalar(q.r()*q.r() + q.i()*q.i() - q.j()*q.j() - q.k()*q.k()),
            Elem::new_scalar(two * ( q.i()*q.j() + q.r()*q.k() )),
            Elem::new_scalar(two * ( q.i()*q.j() - q.r()*q.k() )),
        ],
        [
            Elem::new_scalar(two * ( q.i()*q.j() - q.r()*q.k() )),
            Elem::new_scalar(q.r()*q.r() - q.i()*q.i() + q.j()*q.j() - q.k()*q.k()),
            Elem::new_scalar(two * ( q.j()*q.k() + q.r()*q.i() )),
        ],
        [
            Elem::new_scalar(two * ( q.i()*q.k() + q.r()*q.j() )),
            Elem::new_scalar(two * ( q.j()*q.k() - q.r()*q.i() )),
            Elem::new_scalar(q.r()*q.r() - q.i()*q.i() - q.j()*q.j() + q.k()*q.k()),
        ],
    ])
}

/// Turns this quaternion into a 4x4 Matrix.
/// 
/// # Note
/// This function is not the inverse of [`from_matrix_4`], since this
/// outputs a separate 4x4 representation. There are many ways to
/// represent a quaternion using a 4x4 matrix.
/// 
/// The following formula is used:
/// 
///     # "
///     let q = w + xi + yj + zk
///         M be the matrix repr of q given by this function
/// 
///         =>
/// 
///     M = ⎡ w -x -y -z ⎤
///         ⎢ x  w -z  y ⎥
///         ⎢ y  z  w -x ⎥
///         ⎣ z -y  x  w ⎦
///     # ";
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_matrix_4;
/// 
/// let quat: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// 
/// let matrix: [[i32; 4]; 4] = to_matrix_4::<f32, _, _>(quat);
/// 
/// assert_eq!(
///     matrix,
///     [
///         [1, -2, -3, -4],
///         [2,  1, -4,  3],
///         [3,  4,  1, -2],
///         [4, -3,  2,  1],
///     ]
/// )
/// ```
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn to_matrix_4<Num, Elem, Out>(quaternion: impl Quaternion<Num>) -> Out
where 
    Num: Axis,
    Elem: ScalarConstructor<Num>,
    Out: MatrixConstructor<Elem, 4>,
{
    Out::new_matrix([
        [
            Elem::new_scalar( quaternion.r()),
            Elem::new_scalar(-quaternion.i()),
            Elem::new_scalar(-quaternion.j()),
            Elem::new_scalar(-quaternion.k()),
        ],
        [
            Elem::new_scalar( quaternion.i()),
            Elem::new_scalar( quaternion.r()),
            Elem::new_scalar(-quaternion.k()),
            Elem::new_scalar( quaternion.j()),
        ],
        [
            Elem::new_scalar( quaternion.j()),
            Elem::new_scalar( quaternion.k()), 
            Elem::new_scalar( quaternion.r()),
            Elem::new_scalar(-quaternion.i()),
        ],
        [
            Elem::new_scalar( quaternion.k()),
            Elem::new_scalar(-quaternion.j()),
            Elem::new_scalar( quaternion.i()),
            Elem::new_scalar( quaternion.r()),
        ],
    ])
}

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Gets the polar form of a quaternion.
/// 
/// The values are given in this order (`abs`, `angle`, `unit_vec`).
/// Where:
///     - `abs` = `abs(q)`
///     - `angle` = `angle(q)`
///     - `unit_vec` = `norm(vector_part(q))`
/// 
/// The equasion used: `q == abs * exp(angle * unit_vec)`
pub fn to_polar_form<Num, Abs, Angle, UnitVec>(quaternion: impl Quaternion<Num>) -> (Abs, Angle, UnitVec)
where 
    Num: Axis,
    Abs: ScalarConstructor<Num>,
    Angle: ScalarConstructor<Num>,
    UnitVec: VectorConstructor<Num>,
{
    let abs = abs(&quaternion);
    let vec_inv_abs = Num::ONE / Num::sqrt(
          quaternion.i() * quaternion.i()
        + quaternion.j() * quaternion.j()
        + quaternion.k() * quaternion.k()
    );
    (
        Abs::new_scalar(abs),
        Angle::new_scalar(Num::acos(quaternion.r() / abs)),
        UnitVec::new_vector(
            quaternion.i() * vec_inv_abs,
            quaternion.j() * vec_inv_abs,
            quaternion.k() * vec_inv_abs,
        )
    )
}
