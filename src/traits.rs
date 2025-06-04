/*!
Diverse traits for representing and constructing diverse value
types like quaternions, vectors, scalar values and others.
 */


pub use axis::Axis;
use crate::quat;
use crate::core::marker::Sized;
#[allow(unused_imports)]
use crate::core::option::Option;

/**
The general representation of any quaternion type.

Note: The [`r`](Quaternion::r), [`i`](Quaternion::i), [`j`](Quaternion::j) and [`k`](Quaternion::k)
methods are used as if they are cheap operations.
*/
pub trait Quaternion<Num: Axis> {
    /// The real part of this quaternion.
    fn r(&self) -> Num;
    /// The first imaginary part of this quaternion.
    fn i(&self) -> Num;
    /// The second imaginary part of this quaternion.
    fn j(&self) -> Num;
    /// The third imaginary part of this quaternion.
    fn k(&self) -> Num;
}

/**
The general representation of any unit quaternion type.
 */
pub trait UnitQuaternion<Num: Axis>: Quaternion<Num> { }

/**
The general representation for any scalar type.

Marks that this type can turn into an [`Axis`] type.

Note: The [`scalar`](Scalar::scalar) method is used as if it's a cheap operation.
*/
pub trait Scalar<Num: Axis> {
    /// The [`Axis`] representation of this scalar value.
    fn scalar(&self) -> Num;
}

/**
The general representation for any complex number type.

Note: The [`real`](Complex::real) and [`imaginary`](Complex::imaginary) methods are used as if they are cheap operations.
 */
pub trait Complex<Num: Axis> {
    /// The real part of this complex number.
    fn real(&self) -> Num;
    /// The imaginary part of this complex number.
    fn imaginary(&self) -> Num;
}

/**
The general representation for any vector type.

Note: The [`x`](Vector::x), [`y`](Vector::y) and [`z`](Vector::z)
methods are used as if they are cheap operations.
 */
pub trait Vector<Num: Axis> {
    /// The first part of this vector.
    fn x(&self) -> Num;
    /// The second part of this vector.
    fn y(&self) -> Num;
    /// The third part of this vector.
    fn z(&self) -> Num;
}

/**
The general representation of any rotation baised on euler angles.

This struct uses radians (for all prodived implementations of [`Axis`]).
No actual mesurment is inforced by the struct it'self, so if you wish to use degrees you can,
you just eather need a wrapper or to modify the values each time you use it.

Note: The [`roll`](Rotation::roll), [`pitch`](Rotation::pitch) and [`yaw`](Rotation::yaw)
methods are used as if they are cheap operations.
*/
#[cfg(feature = "rotation")]
pub trait Rotation<Num: Axis> {
    /// The roll of this rotation. (angle on the z axis)
    fn roll( &self ) -> Num;
    /// The pitch of this rotation. (angle on the y axis)
    fn pitch( &self ) -> Num;
    /// The yaw of this rotation. (angle on the x axis)
    fn yaw( &self ) -> Num;
}

/**
The general representation of any NxN rotation matrix.

Only `Matrix<_, 2>`, `Matrix<_, 3>` and `Matrix<_, 4>` have impls and are used.

Note: The [`get_unchecked`](Matrix::get_unchecked) method is used as if it's a cheap operation.
*/
#[cfg(feature = "matrix")]
pub trait Matrix<T, const N: usize> {
    /// Gets the value represented at (row, col)
    /// 
    /// # Important
    /// This value should not panic for values of
    /// `row` and `col` that are both smaller then N.
    fn get_unchecked( &self, row: usize, col: usize ) -> T;

    #[inline]
    /// Checks if `row` and `col` are out of bounds before getting the value at (row, col).
    /// 
    /// # Important
    /// By default this returns [`None`](Option::None)
    /// only if `row` and `col` are both smaller then N.
    /// 
    /// This is because it assumes that [`get_unchecked`](Matrix::get_unchecked) panics
    /// if and only if `row` or `col` is greater then or equal to N.
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        if row < N && col < N {
            Option::Some(self.get_unchecked(row, col))
        } else {
            Option::None
        }
    }

    /// Turns this matrix reprezentation into a NxN array.
    fn to_array( &self ) -> [[T; N]; N] {
        use crate::core::mem::MaybeUninit;
        let mut matrix: [[T; N]; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for row in 0..N {
            for col in 0..N {
                matrix[row][col] = self.get_unchecked(row, col);
            }
        }
        matrix
    }
}

/**
A constructor for quaternions.

Generally used for return types.
 */
pub trait QuaternionConstructor<Num: Axis>: Sized {
    /// Constructs a new quaternion.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let q: [f32; 4] = QuaternionConstructor::<f32>::new_quat(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!( q, [1.0, 2.0, 3.0, 4.0] );
    /// 
    /// let p = <[f32; 4]>::new_quat(0.0, 0.0, 0.0, 0.0);
    /// assert_eq!( p, [0.0, 0.0, 0.0, 0.0] );
    /// ```
    fn new_quat(r: Num, i: Num, j: Num, k: Num) -> Self;

    /// Constructs a new quaternion from another one.
    /// Will have same values.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let from: [u32; 4] = [1, 2, 3, 4];
    /// 
    /// let to: [f32; 4] = QuaternionConstructor::<f32>::from_quat(from);
    /// 
    /// assert_eq!( to, [1.0, 2.0, 3.0, 4.0] );
    /// ```
    #[inline]
    fn from_quat(quat: impl Quaternion<Num>) -> Self {
        QuaternionConstructor::new_quat(quat.r(), quat.i(), quat.j(), quat.k())
    }

    /// Constructs the origin quaternion. (additive identity)
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let quat: [f32; 4] = QuaternionConstructor::<f32>::origin();
    /// 
    /// assert_eq!( quat, [0.0, 0.0, 0.0, 0.0] );
    /// ```
    #[inline]
    fn origin() -> Self { quat::origin() }

    /// Constructs the real positive unit quaternion. (multiplicative identity)
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let quat: [f32; 4] = QuaternionConstructor::<f32>::identity();
    /// 
    /// assert_eq!( quat, [1.0, 0.0, 0.0, 0.0] );
    /// ```
    #[inline]
    fn identity() -> Self { quat::identity() }

    /// Constructs a quaternion with all [`Num::NAN`s](Axis::NAN).
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let quat: [f32; 4] = QuaternionConstructor::<f32>::nan();
    /// 
    /// assert!( quat[0].is_nan() );
    /// assert!( quat[1].is_nan() );
    /// assert!( quat[2].is_nan() );
    /// assert!( quat[3].is_nan() );
    /// ```
    #[inline]
    fn nan() -> Self { quat::nan() }

    /// Constructs the unit quaternion on the real axis.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let unit_r: [f32; 4] = QuaternionConstructor::<f32>::unit_r();
    /// 
    /// assert_eq!( unit_r, [1.0, 0.0, 0.0, 0.0] );
    /// ```
    #[inline]
    fn unit_r() -> Self { quat::unit_r() }

    /// Constructs the unit quaternion on the first imaginary axis.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let unit_i: [f32; 4] = QuaternionConstructor::<f32>::unit_i();
    /// 
    /// assert_eq!( unit_i, [0.0, 1.0, 0.0, 0.0] );
    /// ```
    #[inline]
    fn unit_i() -> Self { quat::unit_i() }

    /// Constructs the unit quaternion on the second imaginary axis.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let unit_j: [f32; 4] = QuaternionConstructor::<f32>::unit_j();
    /// 
    /// assert_eq!( unit_j, [0.0, 0.0, 1.0, 0.0] );
    /// ```
    #[inline]
    fn unit_j() -> Self { quat::unit_j() }

    /// Constructs the unit quaternion on the third imaginary axis.
    ///
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionConstructor;
    /// 
    /// let unit_k: [f32; 4] = QuaternionConstructor::<f32>::unit_k();
    /// 
    /// assert_eq!( unit_k, [0.0, 0.0, 0.0, 1.0] );
    /// ```
    #[inline]
    fn unit_k() -> Self { quat::unit_k() }
}

/**
The general representation of any unit quaternion type.

Note: The [`r`](Quaternion::r), [`i`](Quaternion::i), [`j`](Quaternion::j) and [`k`](Quaternion::k)
methods are used as if they are cheap operations.
 */
pub trait UnitQuaternionConstructor<Num: Axis>: Sized {
    /// Constructs a new unit quaternion without
    /// checking if it's a valid unit quaternion.
    /// 
    /// # Safety
    /// Any quaternion representation that has the r, i, j and k
    ///  valuesgiven as input is a unit quaternion.
    unsafe fn new_unit_quat_unchecked(r: Num, i: Num, j: Num, k: Num) -> Self;

    /// Constructs a new unit quaternion.
    #[inline]
    fn new_unit_quat(r: Num, i: Num, j: Num, k: Num) -> Option<Self> {
        if (r * r + i * i + j * j + k * k - Num::ONE).abs() < Num::ERROR * Num::ERROR {
            unsafe {
                Option::Some(Self::new_unit_quat_unchecked(r, i, j, k))
            }
        } else {
            Option::None
        }
    }

    /// Constructs a new unit quaternion from another one.
    /// 
    /// Will have same values.
    #[inline]
    fn from_unit_quat(quat: impl UnitQuaternion<Num>) -> Self {
        unsafe {
            UnitQuaternionConstructor::new_unit_quat_unchecked(quat.r(), quat.i(), quat.j(), quat.k())
        }
    }

    /// Constructs a new unit quaternion from a normal one.
    /// 
    /// Will have the same values.
    #[inline]
    fn from_quat(quat: impl Quaternion<Num>) -> Option<Self> {
        UnitQuaternionConstructor::new_unit_quat(quat.r(), quat.i(), quat.j(), quat.k())
    }

    /// Constructs a new unit quaternion from a normal one.
    /// 
    /// Will have the same values.
    /// 
    /// # Safety
    /// The given quaternion is a unit quaternion.
    #[inline]
    unsafe fn from_quat_unchecked(quat: impl Quaternion<Num>) -> Self {
        unsafe {
            UnitQuaternionConstructor::new_unit_quat_unchecked(quat.r(), quat.i(), quat.j(), quat.k())
        }
    }
} 

/**
A constructor for vectors.

Generally used for return types.
 */
pub trait VectorConstructor<Num: Axis>: Sized {
    /// Constructs a new vector.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::traits::VectorConstructor;
    /// 
    /// let vec: [f32; 3] = VectorConstructor::new_vector(1.0, 2.0, 3.0);
    /// 
    /// assert_eq!( vec, [1.0, 2.0, 3.0] );
    /// ```
    fn new_vector(x: Num, y: Num, z: Num) -> Self;

    #[inline]
    /// Constructs a new vector from another one.
    /// Will have same values.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::traits::VectorConstructor;
    /// 
    /// let from: [f32; 3] = [1.0, 2.0, 3.0];
    /// 
    /// let to: (f32, u32, i32) = VectorConstructor::<f32>::from_vector(from);
    /// 
    /// assert_eq!( to, (1.0_f32, 2_u32, 3_i32) );
    /// ```
    fn from_vector(vector: impl Vector<Num>) -> Self {
        VectorConstructor::new_vector(vector.x(), vector.y(), vector.z())
    }
} 

/**
A constructor for complex numbers.

Generally used for return types.
 */
pub trait ComplexConstructor<Num: Axis>: Sized {
    /// Constructs a new complex number.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::traits::ComplexConstructor;
    /// 
    /// let complex: (f32, f32) = ComplexConstructor::new_complex(1.0, 2.0);
    /// 
    /// assert_eq!( complex, (1.0, 2.0) );
    /// ```
    fn new_complex(r: Num, i: Num) -> Self;

    #[inline]
    /// Constructs a new complex number from another one.
    /// Will have same values.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::traits::ComplexConstructor;
    /// 
    /// let from: (f32, f32) = (1.0, 2.0);
    /// 
    /// let to: (i8, u16) = ComplexConstructor::<f32>::from_complex(from);
    /// 
    /// assert_eq!( to, (1_i8, 2_u16) );
    /// ```
    fn from_complex(complex: impl Complex<Num>) -> Self {
        ComplexConstructor::new_complex(complex.real(), complex.imaginary())
    }
} 

/**
A constructor for scalar values.

Generally used for return types.
 */
pub trait ScalarConstructor<Num: Axis>: Sized {
    /// Constructs a new scalar value.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::traits::ScalarConstructor;
    /// 
    /// let number: f32 = ScalarConstructor::new_scalar(1.0);
    /// 
    /// assert_eq!( number, 1.0 );
    /// ```
    fn new_scalar(axis: Num) -> Self;

    #[inline]
    /// Constructs a new scalar value from another one.
    /// 
    /// Should represent the same scalar value.
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::traits::ScalarConstructor;
    /// 
    /// let from: f32 = 2763.0;
    /// 
    /// let to: u32 = ScalarConstructor::<f32>::from_scalar(from);
    /// 
    /// assert_eq!( to, 2763_u32 );
    /// ```
    fn from_scalar(scalar: impl Scalar<Num>) -> Self {
        ScalarConstructor::new_scalar(scalar.scalar())
    }
} 

/**
A constructor for values that represent euler angles.

Generally used for return types.
 */
#[cfg(feature = "rotation")]
pub trait RotationConstructor<Num: Axis>: Sized {
    /// Constructs a new rotation.
    fn new_rotation(roll: Num, pitch: Num, yaw: Num) -> Self;

    #[inline]
    /// Constructs a new rotation from another one.
    /// Will have same values.
    fn from_rotation(rotation: impl Rotation<Num>) -> Self {
        RotationConstructor::new_rotation(rotation.roll(), rotation.pitch(), rotation.yaw())
    }
}

/**
A constructor for values that represent a NxN matrix.

Generally used for return types.
 */
#[cfg(feature = "matrix")]
pub trait MatrixConstructor<Num, const N: usize>: Sized {
    /// Constructs a new matrix.
    fn new_matrix(matrix: [[Num; N]; N]) -> Self;

    #[inline]
    /// Constructs a new rotation from another one.
    /// Will have same values.
    fn from_matrix(matrix: impl Matrix<Num, N>) -> Self {
        MatrixConstructor::new_matrix(matrix.to_array())
    }
}

/// Adds constants associated with any quaternion.
pub trait QuaternionConsts<Num: Axis>: Sized + Quaternion<Num> {
    /// The origin quaternion. (Aditive identity)
    const ORIGIN: Self;
    /// The positive real unit quaternion. (Multiplicative identity)
    const IDENTITY: Self;
    /// A quaternion with all [`Num::NAN`s](Axis::NAN).
    const NAN: Self;

    /// The unit quaternion on the real axis.
    const UNIT_R: Self = Self::IDENTITY;
    /// The unit quaternion on the first imaginary axis.
    const UNIT_I: Self;
    /// The unit quaternion on the second imaginary axis.
    const UNIT_J: Self;
    /// The unit quaternion on the third imaginary axis.
    const UNIT_K: Self;
}

/// Adds constants associated with any unit quaternion.
pub trait UnitQuaternionConsts<Num: Axis>: Sized + UnitQuaternion<Num> {
    /// The positive real unit quaternion. (Multiplicative identity)
    const IDENTITY: Self;
    /// A quaternion with all [`Num::NAN`s](Axis::NAN).
    const NAN: Self;

    /// The unit quaternion on the real axis.
    const UNIT_R: Self = Self::IDENTITY;
    /// The unit quaternion on the first imaginary axis.
    const UNIT_I: Self;
    /// The unit quaternion on the second imaginary axis.
    const UNIT_J: Self;
    /// The unit quaternion on the third imaginary axis.
    const UNIT_K: Self;
}

/// Adds constants associated with any scalar value.
pub trait ScalarConsts<Num: Axis>: Sized + Scalar<Num> {
    /// The origin scalar value. (Aditive identity)
    const ZERO: Self;
    /// The positive real unit scalar value. (Multiplicative identity)
    const ONE: Self;
    /// The scalar representation of [`Num::NAN`](Axis::NAN).
    const NAN: Self;
}

/// Adds constants associated with any complex number.
pub trait ComplexConsts<Num: Axis>: Sized + Complex<Num> {
    /// The origin complex number. (Aditive identity)
    const ORIGIN: Self;
    /// The positive real unit complex number. (Multiplicative identity)
    const IDENTITY: Self;
    /// A complex number with all [`Num::NAN`s](Axis::NAN).
    const NAN: Self;

    /// The unit complex number on the real axis.
    const UNIT_REAL: Self = Self::IDENTITY;
    /// The unit  complex number on the imaginary axis.
    const UNIT_IMAGINARY: Self;
}

/// Adds constants associated with any vectors.
pub trait VectorConsts<Num: Axis>: Sized + Vector<Num> {
    /// The origin vector. (Aditive identity)
    const ORIGIN: Self;
    /// A vector with all [`Num::NAN`s](Axis::NAN).
    const NAN: Self;

    /// The unit vector on the x axis.
    const UNIT_X: Self;
    /// The unit vector on the y axis.
    const UNIT_Y: Self;
    /// The unit vector on the z axis.
    const UNIT_Z: Self;
}

pub use quat_methods::QuaternionMethods;

// Quat impls

impl<Num: Axis> Quaternion<Num> for () {
    #[inline(always)] fn r(&self) -> Num { Num::ZERO }
    #[inline(always)] fn i(&self) -> Num { Num::ZERO }
    #[inline(always)] fn j(&self) -> Num { Num::ZERO }
    #[inline(always)] fn k(&self) -> Num { Num::ZERO }
}

impl<Num: Axis> QuaternionConstructor<Num> for () {
    #[inline(always)] fn new_quat(_: Num, _: Num, _: Num, _: Num) { }
    #[inline(always)] fn from_quat(_: impl Quaternion<Num>) { }
}

impl<Num: Axis, T> Quaternion<Num> for [T; 0] {
    #[inline(always)] fn r(&self) -> Num { Num::ZERO }
    #[inline(always)] fn i(&self) -> Num { Num::ZERO }
    #[inline(always)] fn j(&self) -> Num { Num::ZERO }
    #[inline(always)] fn k(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> QuaternionConstructor<Num> for [T; 0] {
    #[inline(always)] fn new_quat(_: Num, _: Num, _: Num, _: Num) -> Self { [] }
    #[inline(always)] fn from_quat(_: impl Quaternion<Num>) -> Self { [] }
}

impl<Num: Axis, S, V> Quaternion<Num> for (S, V)
where 
    S: Scalar<Num>,
    V: Vector<Num>,
{
    #[inline(always)] fn r(&self) -> Num { self.0.scalar() }
    #[inline(always)] fn i(&self) -> Num { self.1.x()  }
    #[inline(always)] fn j(&self) -> Num { self.1.y() }
    #[inline(always)] fn k(&self) -> Num { self.1.z()  }
}

impl<Num: Axis, S, V> QuaternionConstructor<Num> for (S, V)
where 
    S: ScalarConstructor<Num>,
    V: VectorConstructor<Num>,
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> (S, V) {
        (
            ScalarConstructor::new_scalar(r),
            VectorConstructor::new_vector(i, j, k)
        )
    }
}

impl<Num: Axis, R, I, J, K> Quaternion<Num> for (R, I, J, K)
where
    R: Scalar<Num>,
    I: Scalar<Num>,
    J: Scalar<Num>,
    K: Scalar<Num>,
{
    #[inline(always)] fn r(&self) -> Num { self.0.scalar() }
    #[inline(always)] fn i(&self) -> Num { self.1.scalar() }
    #[inline(always)] fn j(&self) -> Num { self.2.scalar() }
    #[inline(always)] fn k(&self) -> Num { self.3.scalar() }
}

impl<Num: Axis, R, I, J, K> QuaternionConstructor<Num> for (R, I, J, K)
where
    R: ScalarConstructor<Num>,
    I: ScalarConstructor<Num>,
    J: ScalarConstructor<Num>,
    K: ScalarConstructor<Num>,
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> (R, I, J, K) {
        (
            ScalarConstructor::new_scalar(r),
            ScalarConstructor::new_scalar(i),
            ScalarConstructor::new_scalar(j),
            ScalarConstructor::new_scalar(k),
        )
    }
}

impl<Num: Axis, S> Quaternion<Num> for [S; 4]
where S: Scalar<Num>
{
    #[inline(always)] fn r(&self) -> Num { self[0].scalar() }
    #[inline(always)] fn i(&self) -> Num { self[1].scalar() }
    #[inline(always)] fn j(&self) -> Num { self[2].scalar() }
    #[inline(always)] fn k(&self) -> Num { self[3].scalar() }
}

impl<Num: Axis, S> QuaternionConstructor<Num> for [S; 4]
where S: ScalarConstructor<Num>
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> [S; 4] {
        [
            ScalarConstructor::new_scalar(r),
            ScalarConstructor::new_scalar(i),
            ScalarConstructor::new_scalar(j),
            ScalarConstructor::new_scalar(k),
        ]
    }
}

impl<Num: Axis, C, J, K> Quaternion<Num> for (C, J, K)
where
    C: Complex<Num>,
    J: Scalar<Num>,
    K: Scalar<Num>,
{
    #[inline(always)] fn r(&self) -> Num { self.0.real() }
    #[inline(always)] fn i(&self) -> Num { self.0.imaginary() }
    #[inline(always)] fn j(&self) -> Num { self.1.scalar() }
    #[inline(always)] fn k(&self) -> Num { self.2.scalar() }
}

impl<Num: Axis, C, J, K> QuaternionConstructor<Num> for (C, J, K)
where
    C: ComplexConstructor<Num>,
    J: ScalarConstructor<Num>,
    K: ScalarConstructor<Num>,
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> (C, J, K) {
        (
            ComplexConstructor::new_complex(r, i),
            ScalarConstructor::new_scalar(j),
            ScalarConstructor::new_scalar(k),
        )
    }
}

impl<Num: Axis, T> Quaternion<Num> for &T
where T: Quaternion<Num>
{
    #[inline(always)] fn r(&self) -> Num { (*self).r() }
    #[inline(always)] fn i(&self) -> Num { (*self).i() }
    #[inline(always)] fn j(&self) -> Num { (*self).j() }
    #[inline(always)] fn k(&self) -> Num { (*self).k() }
}

impl<Num: Axis, R, I, J, K> QuaternionMethods<Num> for (R, I, J, K)
where 
    R: Scalar<Num> + ScalarConstructor<Num>,
    I: Scalar<Num> + ScalarConstructor<Num>,
    J: Scalar<Num> + ScalarConstructor<Num>,
    K: Scalar<Num> + ScalarConstructor<Num>,
{}
impl<Num: Axis, R, I, J, K> QuaternionConsts<Num> for (R, I, J, K)
where 
    R: ScalarConsts<Num>,
    I: ScalarConsts<Num>,
    J: ScalarConsts<Num>,
    K: ScalarConsts<Num>,
{
    const ORIGIN: Self = (R::ZERO, I::ZERO, J::ZERO, K::ZERO);
    const IDENTITY: Self = (R::ONE, I::ZERO, J::ZERO, K::ZERO);
    const NAN: Self = (R::NAN, I::NAN, J::NAN, K::NAN);
    const UNIT_I: Self = (R::ZERO, I::ONE, J::ZERO, K::ZERO);
    const UNIT_J: Self = (R::ZERO, I::ZERO, J::ONE, K::ZERO);
    const UNIT_K: Self = (R::ZERO, I::ZERO, J::ZERO, K::ONE);
}

impl<Num: Axis, S> QuaternionMethods<Num> for [S; 4]
where S: Scalar<Num> + ScalarConstructor<Num>
{}
impl<Num: Axis, S> QuaternionConsts<Num> for [S; 4]
where S: ScalarConsts<Num>
{
    const ORIGIN: Self = [S::ZERO, S::ZERO, S::ZERO, S::ZERO];
    const IDENTITY: Self = [S::ONE, S::ZERO, S::ZERO, S::ZERO];
    const NAN: Self = [S::NAN, S::NAN, S::NAN, S::NAN];
    const UNIT_I: Self = [S::ZERO, S::ONE, S::ZERO, S::ZERO];
    const UNIT_J: Self = [S::ZERO, S::ZERO, S::ONE, S::ZERO];
    const UNIT_K: Self = [S::ZERO, S::ZERO, S::ZERO, S::ONE];
}

impl<Num: Axis, S, V> QuaternionMethods<Num> for (S, V)
where 
    S: Scalar<Num> + ScalarConstructor<Num>,
    V: Vector<Num> + VectorConstructor<Num>,
{
    #[inline]
    fn vector_part(self) -> Self {
        (S::new_scalar(Num::ZERO), self.1)
    }

    #[inline]
    fn scalar_part(self) -> Self {
        (self.0, V::new_vector(Num::ZERO, Num::ZERO, Num::ZERO))
    }

    #[inline]
    fn from_vector(vector: impl Vector<Num>) -> Self {
        (S::new_scalar(Num::ZERO), V::from_vector(vector))
    }

    #[inline]
    fn from_scalar(scalar: impl Scalar<Num>) -> Self {
        (S::from_scalar(scalar), V::new_vector(Num::ZERO, Num::ZERO, Num::ZERO))
    }

    #[inline]
    // There might be edgecases where `from_vector(v)` is not `new_vector(v.x(), v.y(), v.z())`
    fn to_vector<Out: VectorConstructor<Num>>(self) -> Out {
        VectorConstructor::from_vector(self.1)
    }

    #[inline]
    // There might be edgecases where `from_scalar(s)` is not `new_scalar(s.scalar())`
    fn to_scalar<Out: ScalarConstructor<Num>>(self) -> Out {
        ScalarConstructor::from_scalar(self.0)
    }
}
impl<Num: Axis, S, V> QuaternionConsts<Num> for (S, V)
where 
    S: ScalarConsts<Num>,
    V: VectorConsts<Num>,
{
    const ORIGIN: Self = (S::ZERO, V::ORIGIN);
    const IDENTITY: Self = (S::ONE, V::ORIGIN);
    const NAN: Self = (S::NAN, V::NAN);
    const UNIT_I: Self = (S::ZERO, V::UNIT_X);
    const UNIT_J: Self = (S::ZERO, V::UNIT_Y);
    const UNIT_K: Self = (S::ZERO, V::UNIT_Z);
}

impl<Num: Axis, C, J, K> QuaternionMethods<Num> for (C, J, K)
where 
    C: Complex<Num> + ComplexConstructor<Num>,
    J: Scalar<Num> + ScalarConstructor<Num>,
    K: Scalar<Num> + ScalarConstructor<Num>,
{
    #[inline]
    fn complex_part(self) -> Self {
        (self.0, J::new_scalar(Num::ZERO), K::new_scalar(Num::ZERO))
    }

    #[inline]
    fn from_complex(complex: impl Complex<Num>) -> Self {
        (C::from_complex(complex), J::new_scalar(Num::ZERO), K::new_scalar(Num::ZERO))
    }

    #[inline]
    // There might be edgecases where `from_complex(c)` is not `new_complex(c.real(), c.imaginary())`
    fn to_complex<Out: ComplexConstructor<Num>>(self) -> Out {
        ComplexConstructor::from_complex(self.0)
    }
}
impl<Num: Axis, C, J, K> QuaternionConsts<Num> for (C, J, K)
where 
    C: ComplexConsts<Num>,
    J: ScalarConsts<Num>,
    K: ScalarConsts<Num>,
{
    const ORIGIN: Self = (C::ORIGIN, J::ZERO, K::ZERO);
    const IDENTITY: Self = (C::IDENTITY, J::ZERO, K::ZERO);
    const NAN: Self = (C::NAN, J::NAN, K::NAN);
    const UNIT_I: Self = (C::UNIT_IMAGINARY, J::ZERO, K::ZERO);
    const UNIT_J: Self = (C::ORIGIN, J::ONE, K::ZERO);
    const UNIT_K: Self = (C::ORIGIN, J::ZERO, K::ONE);
}

impl<Num: Axis, Q> Quaternion<Num> for (Q, )
where Q: Quaternion<Num>
{
    #[inline(always)] fn r(&self) -> Num { self.0.r() }
    #[inline(always)] fn i(&self) -> Num { self.0.i() }
    #[inline(always)] fn j(&self) -> Num { self.0.j() }
    #[inline(always)] fn k(&self) -> Num { self.0.k() }
}

impl<Num: Axis, Q> QuaternionConstructor<Num> for (Q, )
where Q: QuaternionConstructor<Num>
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> Self {
        (
            QuaternionConstructor::new_quat(r, i, j, k),
        )
    }
}

impl<Num: Axis, Q> Quaternion<Num> for [Q; 1]
where Q: Quaternion<Num>
{
    #[inline(always)] fn r(&self) -> Num { self[0].r() }
    #[inline(always)] fn i(&self) -> Num { self[0].i() }
    #[inline(always)] fn j(&self) -> Num { self[0].j() }
    #[inline(always)] fn k(&self) -> Num { self[0].k() }
}

impl<Num: Axis, Q> QuaternionConstructor<Num> for [Q; 1]
where Q: QuaternionConstructor<Num>
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> Self {
        [
            QuaternionConstructor::new_quat(r, i, j, k),
        ]
    }
}

// Unit Quaternion impls

impl<Num: Axis, U> UnitQuaternion<Num> for &U
where U: UnitQuaternion<Num>
{}

impl<Num: Axis, S: ScalarConstructor<Num>> UnitQuaternionConstructor<Num> for [S; 4]
{
    #[inline]
    unsafe fn new_unit_quat_unchecked(r: Num, i: Num, j: Num, k: Num) -> Self {
        Self::new_quat(r, i, j, k)
    }
}

impl<Num: Axis, Q> UnitQuaternion<Num> for (Q, )
where Q: UnitQuaternion<Num>
{ }

impl<Num: Axis, Q> UnitQuaternionConstructor<Num> for (Q, )
where Q: UnitQuaternionConstructor<Num>
{
    #[inline] unsafe fn new_unit_quat_unchecked(r: Num, i: Num, j: Num, k: Num) -> Self {
        (
            unsafe {UnitQuaternionConstructor::new_unit_quat_unchecked(r, i, j, k)},
        )
    }
}

impl<Num: Axis, Q> UnitQuaternion<Num> for [Q; 1]
where Q: UnitQuaternion<Num>
{ }

impl<Num: Axis, Q> UnitQuaternionConstructor<Num> for [Q; 1]
where Q: UnitQuaternionConstructor<Num>
{
    #[inline] unsafe fn new_unit_quat_unchecked(r: Num, i: Num, j: Num, k: Num) -> Self {
        [
            unsafe {UnitQuaternionConstructor::new_unit_quat_unchecked(r, i, j, k)},
        ]
    }
}

// Scalar impls

impl<Num: Axis> Scalar<Num> for () {
    #[inline(always)] fn scalar(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> Scalar<Num> for [T; 0] {
    #[inline(always)] fn scalar(&self) -> Num { Num::ZERO }
}

impl<Num: Axis> Scalar<Num> for Num {
    #[inline(always)] fn scalar(&self) -> Num { *self }
}

impl<Num: Axis> ScalarConsts<Num> for Num {
    const ZERO: Self = <Num as Axis>::ZERO;
    const ONE: Self = <Num as Axis>::ONE;
    const NAN: Self = <Num as Axis>::NAN;
}

impl<From: Axis, To: Axis> ScalarConstructor<From> for To
where From: Scalar<To>
{
    #[inline(always)] fn new_scalar( scalar: From ) -> To { scalar.scalar() }
}

impl<Num: Axis, S> Scalar<Num> for (S, )
where S: Scalar<Num>
{
    #[inline(always)] fn scalar(&self) -> Num { self.0.scalar() }
}

impl<Num: Axis, S> ScalarConstructor<Num> for (S, )
where S: ScalarConstructor<Num>
{
    #[inline(always)] fn new_scalar( axis: Num ) -> (S, ) { (ScalarConstructor::new_scalar(axis), ) }
}

impl<Num: Axis, S> ScalarConsts<Num> for (S, )
where S: ScalarConsts<Num>
{
    const ZERO: Self = (S::ZERO, );
    const ONE: Self = (S::ONE, );
    const NAN: Self = (S::NAN, );
}

impl<Num: Axis, S> Scalar<Num> for [S; 1]
where S: Scalar<Num>
{
    #[inline(always)] fn scalar(&self) -> Num { self[0].scalar() }
}

impl<Num: Axis, S> ScalarConstructor<Num> for [S; 1]
where S: ScalarConstructor<Num>
{
    #[inline(always)] fn new_scalar( axis: Num ) -> [S; 1] { [ScalarConstructor::new_scalar(axis)] }
}

impl<Num: Axis, S> ScalarConsts<Num> for [S; 1]
where S: ScalarConsts<Num>
{
    const ZERO: Self = [S::ZERO];
    const ONE: Self = [S::ONE];
    const NAN: Self = [S::NAN];
}

// Complex impls

impl<Num: Axis> Complex<Num> for () {
    #[inline(always)] fn real(&self) -> Num { Num::ZERO }
    #[inline(always)] fn imaginary(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> Complex<Num> for [T; 0] {
    #[inline(always)] fn real(&self) -> Num { Num::ZERO }
    #[inline(always)] fn imaginary(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, R, I> Complex<Num> for (R, I)
where 
    R: Scalar<Num>,
    I: Scalar<Num>,
{
    #[inline(always)] fn real(&self) -> Num { self.0.scalar() }
    #[inline(always)] fn imaginary(&self) -> Num { self.1.scalar() }
}

impl<Num: Axis, R, I> ComplexConstructor<Num> for (R, I)
where 
    R: ScalarConstructor<Num>,
    I: ScalarConstructor<Num>,
{
    #[inline] fn new_complex(r: Num, i: Num) -> (R, I) {
        (
            ScalarConstructor::new_scalar(r),
            ScalarConstructor::new_scalar(i),
        )
    }
}

impl<Num: Axis, R, I> ComplexConsts<Num> for (R, I)
where 
    R: ScalarConsts<Num>,
    I: ScalarConsts<Num>,
{
    const ORIGIN: Self = (R::ZERO, I::ZERO);
    const IDENTITY: Self = (R::ONE, I::ZERO);
    const NAN: Self = (R::NAN, I::NAN);
    const UNIT_IMAGINARY: Self = (R::ZERO, I::ONE);
}

impl<Num: Axis, S> Complex<Num> for [S; 2]
where S: Scalar<Num>
{
    #[inline(always)] fn real(&self) -> Num { self[0].scalar() }
    #[inline(always)] fn imaginary(&self) -> Num { self[1].scalar() }
}

impl<Num: Axis, S> ComplexConstructor<Num> for [S; 2]
where 
    S: ScalarConstructor<Num>,
{
    #[inline] fn new_complex(r: Num, i: Num) -> [S; 2] {
        [
            ScalarConstructor::new_scalar(r),
            ScalarConstructor::new_scalar(i),
        ]
    }
}

impl<Num: Axis, S> ComplexConsts<Num> for [S; 2]
where 
    S: ScalarConsts<Num>,
{
    const ORIGIN: Self = [S::ZERO, S::ZERO];
    const IDENTITY: Self = [S::ONE, S::ZERO];
    const NAN: Self = [S::NAN, S::NAN];
    const UNIT_IMAGINARY: Self = [S::ZERO, S::ONE];
}

impl<Num: Axis, T> Complex<Num> for &T
where T: Complex<Num>
{
    #[inline(always)] fn real(&self) -> Num { (*self).real() }
    #[inline(always)] fn imaginary(&self) -> Num { (*self).imaginary() }
}

impl<Num: Axis, C> Complex<Num> for (C, )
where C: Complex<Num>
{
    #[inline(always)] fn real(&self) -> Num { self.0.real() }
    #[inline(always)] fn imaginary(&self) -> Num { self.0.imaginary() }
}

impl<Num: Axis, C> ComplexConstructor<Num> for (C, )
where C: ComplexConstructor<Num>
{
    #[inline] fn new_complex(real: Num, imaginary: Num) -> Self {
        (
            ComplexConstructor::new_complex(real, imaginary),
        )
    }
}

impl<Num: Axis, C> Complex<Num> for [C; 1]
where C: Complex<Num>
{
    #[inline(always)] fn real(&self) -> Num { self[0].real() }
    #[inline(always)] fn imaginary(&self) -> Num { self[0].imaginary() }
}

impl<Num: Axis, C> ComplexConstructor<Num> for [C; 1]
where C: ComplexConstructor<Num>
{
    #[inline] fn new_complex(real: Num, imaginary: Num) -> Self {
        [
            ComplexConstructor::new_complex(real, imaginary),
        ]
    }
}

// Vector impls

impl<Num: Axis> Vector<Num> for () {
    #[inline(always)] fn x(&self) -> Num { Num::ZERO }
    #[inline(always)] fn y(&self) -> Num { Num::ZERO }
    #[inline(always)] fn z(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> Vector<Num> for [T; 0] {
    #[inline(always)] fn x(&self) -> Num { Num::ZERO }
    #[inline(always)] fn y(&self) -> Num { Num::ZERO }
    #[inline(always)] fn z(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, X, Y, Z> Vector<Num> for (X, Y, Z)
where
    X: Scalar<Num>,
    Y: Scalar<Num>,
    Z: Scalar<Num>,
{
    #[inline(always)] fn x(&self) -> Num { self.0.scalar() }
    #[inline(always)] fn y(&self) -> Num { self.1.scalar() }
    #[inline(always)] fn z(&self) -> Num { self.2.scalar() }
}

impl<Num: Axis, X, Y, Z> VectorConstructor<Num> for (X, Y, Z)
where
    X: ScalarConstructor<Num>,
    Y: ScalarConstructor<Num>,
    Z: ScalarConstructor<Num>,
{
    #[inline] fn new_vector(x: Num, y: Num, z: Num) -> (X, Y, Z) {
        (
            ScalarConstructor::new_scalar(x),
            ScalarConstructor::new_scalar(y),
            ScalarConstructor::new_scalar(z),
        )
    }
}

impl<Num: Axis, X, Y, Z> VectorConsts<Num> for (X, Y, Z)
where
    X: ScalarConsts<Num>,
    Y: ScalarConsts<Num>,
    Z: ScalarConsts<Num>,
{
    const ORIGIN: Self = (X::ZERO, Y::ZERO, Z::ZERO);
    const NAN: Self = (X::NAN, Y::NAN, Z::NAN);
    const UNIT_X: Self = (X::ONE, Y::ZERO, Z::ZERO);
    const UNIT_Y: Self = (X::ZERO, Y::ONE, Z::ZERO);
    const UNIT_Z: Self = (X::ZERO, Y::ZERO, Z::ONE);
}

impl<Num: Axis, S> Vector<Num> for [S; 3]
where S: Scalar<Num>
{
    #[inline(always)] fn x(&self) -> Num { self[0].scalar() }
    #[inline(always)] fn y(&self) -> Num { self[1].scalar() }
    #[inline(always)] fn z(&self) -> Num { self[2].scalar() }
}

impl<Num: Axis, S> VectorConstructor<Num> for [S; 3]
where
    S: ScalarConstructor<Num>,
{
    #[inline] fn new_vector(x: Num, y: Num, z: Num) -> [S; 3] {
        [
            ScalarConstructor::new_scalar(x),
            ScalarConstructor::new_scalar(y),
            ScalarConstructor::new_scalar(z),
        ]
    }
}

impl<Num: Axis, S> VectorConsts<Num> for [S; 3]
where
    S: ScalarConsts<Num>,
{
    const ORIGIN: Self = [S::ZERO, S::ZERO, S::ZERO];
    const NAN: Self = [S::NAN, S::NAN, S::NAN];
    const UNIT_X: Self = [S::ONE, S::ZERO, S::ZERO];
    const UNIT_Y: Self = [S::ZERO, S::ONE, S::ZERO];
    const UNIT_Z: Self = [S::ZERO, S::ZERO, S::ONE];
}

impl<Num: Axis, T> Vector<Num> for &T
where T: Vector<Num>
{
    #[inline(always)] fn x(&self) -> Num { (*self).x() }
    #[inline(always)] fn y(&self) -> Num { (*self).y() }
    #[inline(always)] fn z(&self) -> Num { (*self).z() }
}

impl<Num: Axis, V> Vector<Num> for (V, )
where V: Vector<Num>
{
    #[inline(always)] fn x(&self) -> Num { self.0.x() }
    #[inline(always)] fn y(&self) -> Num { self.0.y() }
    #[inline(always)] fn z(&self) -> Num { self.0.z() }
}

impl<Num: Axis, V> VectorConstructor<Num> for (V, )
where V: VectorConstructor<Num>
{
    #[inline] fn new_vector(x: Num, y: Num, z: Num) -> Self {
        (
            VectorConstructor::new_vector(x, y, z),
        )
    }
}

impl<Num: Axis, V> Vector<Num> for [V; 1]
where V: Vector<Num>
{
    #[inline(always)] fn x(&self) -> Num { self[0].x() }
    #[inline(always)] fn y(&self) -> Num { self[0].y() }
    #[inline(always)] fn z(&self) -> Num { self[0].z() }
}

impl<Num: Axis, V> VectorConstructor<Num> for [V; 1]
where V: VectorConstructor<Num>
{
    #[inline] fn new_vector(x: Num, y: Num, z: Num) -> Self {
        [
            VectorConstructor::new_vector(x, y, z),
        ]
    }
}

// Rotation impls

#[cfg(feature = "rotation")]
impl<Num: Axis> Rotation<Num> for () {
    #[inline(always)] fn roll(&self) -> Num { Num::ZERO }
    #[inline(always)] fn pitch(&self) -> Num { Num::ZERO }
    #[inline(always)] fn yaw(&self) -> Num { Num::ZERO }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, T> Rotation<Num> for [T; 0] {
    #[inline(always)] fn roll(&self) -> Num { Num::ZERO }
    #[inline(always)] fn pitch(&self) -> Num { Num::ZERO }
    #[inline(always)] fn yaw(&self) -> Num { Num::ZERO }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, R> Rotation<Num> for (R, )
where R: Rotation<Num>
{
    #[inline(always)] fn roll(&self) -> Num { self.0.roll() }
    #[inline(always)] fn pitch(&self) -> Num { self.0.pitch() }
    #[inline(always)] fn yaw(&self) -> Num { self.0.yaw() }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, R> RotationConstructor<Num> for (R, )
where R: RotationConstructor<Num>
{
    #[inline] fn new_rotation(roll: Num, pitch: Num, yaw: Num) -> (R, ) {
        (
            RotationConstructor::new_rotation(roll, pitch, yaw),
        )
    }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, R> Rotation<Num> for [R; 1]
where R: Rotation<Num>
{
    #[inline(always)] fn roll(&self) -> Num { self[0].roll() }
    #[inline(always)] fn pitch(&self) -> Num { self[0].pitch() }
    #[inline(always)] fn yaw(&self) -> Num { self[0].yaw() }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, R> RotationConstructor<Num> for [R; 1]
where R: RotationConstructor<Num>
{
    #[inline] fn new_rotation(roll: Num, pitch: Num, yaw: Num) -> [R; 1] {
        [
            RotationConstructor::new_rotation(roll, pitch, yaw),
        ]
    }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, R, P, Y> Rotation<Num> for (R, P, Y)
where
    R: Scalar<Num>,
    P: Scalar<Num>,
    Y: Scalar<Num>,
{
    #[inline(always)] fn roll(&self) -> Num { self.0.scalar() }
    #[inline(always)] fn pitch(&self) -> Num { self.1.scalar() }
    #[inline(always)] fn yaw(&self) -> Num { self.2.scalar() }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, R, P, Y> RotationConstructor<Num> for (R, P, Y)
where
    R: ScalarConstructor<Num>,
    P: ScalarConstructor<Num>,
    Y: ScalarConstructor<Num>,
{
    #[inline] fn new_rotation(roll: Num, pitch: Num, yaw: Num) -> (R, P, Y) {
        (
            ScalarConstructor::new_scalar(roll),
            ScalarConstructor::new_scalar(pitch),
            ScalarConstructor::new_scalar(yaw),
        )
    }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, S> Rotation<Num> for [S; 3]
where S: Scalar<Num>
{
    #[inline(always)] fn roll(&self) -> Num { self[0].scalar() }
    #[inline(always)] fn pitch(&self) -> Num { self[1].scalar() }
    #[inline(always)] fn yaw(&self) -> Num { self[2].scalar() }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, S> RotationConstructor<Num> for [S; 3]
where
    S: ScalarConstructor<Num>,
{
    #[inline] fn new_rotation(i: Num, j: Num, k: Num) -> [S; 3] {
        [
            ScalarConstructor::new_scalar(i),
            ScalarConstructor::new_scalar(j),
            ScalarConstructor::new_scalar(k),
        ]
    }
}

#[cfg(feature = "rotation")]
impl<Num: Axis, T> Rotation<Num> for &T
where T: Rotation<Num>
{
    fn roll(&self) -> Num { (*self).roll() }
    fn pitch(&self) -> Num { (*self).pitch() }
    fn yaw(&self) -> Num { (*self).yaw() }
}

// Matrix impls

// TODO Try to optimize these transfomations + make then be as good to inline as they can get

#[cfg(feature = "matrix")]
impl<T: crate::core::clone::Clone, const N: usize> Matrix<T, N> for [[T; N]; N]
{
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        self[row][col].clone()
    }
}

#[cfg(feature = "matrix")]
impl<T: crate::core::clone::Clone, const N: usize> MatrixConstructor<T, N> for [[T; N]; N]
{
    #[inline]
    fn new_matrix(matrix: [[T; N]; N]) -> Self { matrix }
}

#[cfg(feature = "matrix")]
mod matrix;

#[cfg(feature = "matrix")]
impl<T, M, const N: usize> Matrix<T, N> for &M
where M: Matrix<T, N>
{
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        (*self).get_unchecked(row, col)
    }
}

#[cfg(feature = "matrix")]
impl<Num: Axis, M, const N: usize> Matrix<Num, N> for (M, )
where M: Matrix<Num, N>
{
    #[inline(always)] fn get_unchecked( &self, row: usize, col: usize ) -> Num {
        self.0.get_unchecked(row, col)
    }
}

#[cfg(feature = "matrix")]
impl<Num: Axis, M, const N: usize> MatrixConstructor<Num, N> for (M, )
where M: MatrixConstructor<Num, N>
{
    #[inline] fn new_matrix(matrix: [[Num; N]; N]) -> Self {
        (
            MatrixConstructor::new_matrix(matrix),
        )
    }
}

// feature impls

#[cfg(feature = "alloc")]
use crate::alloc::{
    sync::Arc,
    boxed::Box,
    borrow::{
        Cow,
        ToOwned,
    },
    rc::Rc,
};

use crate::core::mem::ManuallyDrop;
use crate::core::cell::{
    Ref,
    RefMut,
    LazyCell,
};

macro_rules! ref_impls {
    ( $ty:ty $(: $( $trait:ident ),+ )? ) => {
        impl<Num: Axis, T> Quaternion<Num> for $ty
        where T: Quaternion<Num> $($( + $trait )+)?
        {
            fn r(&self) -> Num { (*(*self)).r() }
            fn i(&self) -> Num { (*(*self)).i() }
            fn j(&self) -> Num { (*(*self)).j() }
            fn k(&self) -> Num { (*(*self)).k() }
        }
        
        impl<Num: Axis, T> UnitQuaternion<Num> for $ty
        where T: UnitQuaternion<Num> $($( + $trait )+)?
        { }

        impl<Num: Axis, T> Vector<Num> for $ty
        where T: Vector<Num> $($( + $trait )+)?
        {
            fn x(&self) -> Num { (*(*self)).x() }
            fn y(&self) -> Num { (*(*self)).y() }
            fn z(&self) -> Num { (*(*self)).z() }
        }

        impl<Num: Axis, T> Complex<Num> for $ty
        where T: Complex<Num> $($( + $trait )+)?
        {
            fn real(&self) -> Num { (*(*self)).real() }
            fn imaginary(&self) -> Num { (*(*self)).imaginary() }
        }

        #[cfg(feature = "rotation")]
        impl<Num: Axis, T> Rotation<Num> for $ty
        where T: Rotation<Num> $($( + $trait )+)?
        {
            fn roll(&self) -> Num { (*(*self)).roll() }
            fn pitch(&self) -> Num { (*(*self)).pitch() }
            fn yaw(&self) -> Num { (*(*self)).yaw() }
        }

        #[cfg(feature = "matrix")]
        impl<Elem, const N: usize, T> Matrix<Elem, N> for $ty
        where T: Matrix<Elem, N> $($( + $trait )+)?
        {
            #[inline] fn get_unchecked( &self, row: usize, col: usize ) -> Elem { (*(*self)).get_unchecked(row, col) }

            #[inline] fn get( &self, row: usize, col: usize ) -> Option<Elem> { (*(*self)).get(row, col) }

            #[inline] fn to_array( &self ) -> [[Elem; N]; N] { (*(*self)).to_array() }
        }
    };
}

#[cfg(feature = "alloc")] ref_impls!{Box<T>}
#[cfg(feature = "alloc")] ref_impls!{Rc<T>}
#[cfg(feature = "alloc")] ref_impls!{Arc<T>}
#[cfg(feature = "alloc")] ref_impls!{Cow<'_, T>: ToOwned}
ref_impls!{LazyCell<T>}
ref_impls!{Ref<'_, T>}
ref_impls!{RefMut<'_, T>}
ref_impls!{ManuallyDrop<T>}
ref_impls!{&mut T}

// Other impls

mod axis;

mod quat_methods;

mod core_impls;

mod dep_impls;

mod target_arch_impls;
