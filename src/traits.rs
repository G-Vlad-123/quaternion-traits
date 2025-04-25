
pub use axis::Axis;
use crate::quat;
use crate::core::marker::Sized;

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
pub trait Rotation<Num: Axis> {
    /// The roll of this rotation.
    fn roll( &self ) -> Num;
    /// The pitch of this rotation.
    fn pitch( &self ) -> Num;
    /// The yaw of this rotation.
    fn yaw( &self ) -> Num;
}

/**
A constructor for quaternions.
 */
pub trait NewQuaternion<Num: Axis>: Sized {
    /// Constructs a new quaternion
    fn new_quat(r: Num, i: Num, j: Num, k: Num) -> Self;

    #[inline]
    /// Constructs a new quaternion from another one.
    /// Will have same values.
    fn from_quat(quat: impl Quaternion<Num>) -> Self {
        NewQuaternion::new_quat(quat.r(), quat.i(), quat.j(), quat.k())
    }

    #[inline]
    /// Constructs the origin quaternion. (additive identity)
    fn origin() -> Self { quat::origin() }

    #[inline]
    /// Constructs the multiplicative identity for quaternions.
    fn identity() -> Self { quat::identity() }

    #[inline]
    /// Constructs a quaternion with all [`Num::NAN`s](Axis::NAN).
    fn nan() -> Self { quat::nan() }
} 

/**
A constructor for vectors.
 */
pub trait NewVector<Num: Axis>: Sized {
    /// Constructs a new vector.
    fn new_vector(i: Num, j: Num, k: Num) -> Self;

    #[inline]
    /// Constructs a new vector from another one.
    /// Will have same values.
    fn from_vector(vector: impl Vector<Num>) -> Self {
        NewVector::new_vector(vector.x(), vector.y(), vector.z())
    }
} 

/**
A constructor for complex numbers.
 */
pub trait NewComplex<Num: Axis>: Sized {
    /// Constructs a new complex number.
    fn new_complex(r: Num, i: Num) -> Self;

    #[inline]
    /// Constructs a new complex number from another one.
    /// Will have same values.
    fn from_complex(complex: impl Complex<Num>) -> Self {
        NewComplex::new_complex(complex.real(), complex.imaginary())
    }
} 

/**
A constructor for scalar values.
 */
pub trait NewScalar<Num: Axis>: Sized {
    /// Constructs a new scalar value.
    fn new_scalar(axis: Num) -> Self;

    #[inline]
    /// Constructs a new scalar value from another one.
    /// Will have same values.
    fn from_scalar(scalar: impl Scalar<Num>) -> Self {
        NewScalar::new_scalar(scalar.scalar())
    }
} 

/**
A constructor for scalar values.
 */
pub trait NewRotation<Num: Axis>: Sized {
    /// Constructs a new scalar value.
    fn new_rotation(roll: Num, pitch: Num, yaw: Num) -> Self;

    #[inline]
    /// Constructs a new rotation from another one.
    /// Will have same values.
    fn from_rotation(rotation: impl Rotation<Num>) -> Self {
        NewRotation::new_rotation(rotation.roll(), rotation.pitch(), rotation.yaw())
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

/**
Adds most functions in this crate as methods that take `self` and/or return `Self`.

Most methods have dry documentation and just link to their respective crate function.
If possible it's planned to have the same* documentation for both instances of each function
in the future.

\* With examples that match their respective function.
 */
pub trait QuaternionMethods<Num: Axis>: Quaternion<Num> + NewQuaternion<Num> + Sized {
    /// Constructs the origin quaternion. (Additive identity)
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionMethods;
    /// 
    /// let quat: [f32; 4] = QuaternionMethods::<f32>::origin();
    /// 
    /// assert_eq!( quat, [0.0, 0.0, 0.0, 0.0] );
    /// ```
    #[inline] fn origin() -> Self { quat::origin() }
    /// Constructs the positive real unit quaternion. (Multiplicative identity)
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionMethods;
    /// 
    /// let quat: [f32; 4] = QuaternionMethods::<f32>::identity();
    /// 
    /// assert_eq!( quat, [1.0, 0.0, 0.0, 0.0] );
    /// ```
    #[inline] fn identity() -> Self { quat::identity() }
    /// Constructs a quaternion that has all axies set to [`Num::NAN`s](Axis::NAN).
    /// 
    /// # Example
    /// ```
    /// use quaternion_traits::QuaternionMethods;
    /// 
    /// let quat: [f32; 4] = QuaternionMethods::<f32>::nan();
    /// 
    /// assert!( quat[0].is_nan() );
    /// assert!( quat[1].is_nan() );
    /// assert!( quat[2].is_nan() );
    /// assert!( quat[3].is_nan() );
    /// ```
    #[inline] fn nan() -> Self { quat::nan() }
    /// Adds two quaternions togheder.
    /// 
    /// Check [the add function](crate::add) in the root for more info.
    #[inline] fn add(self, other: impl Quaternion<Num>) -> Self { quat::add(self, other) }
    /// Subtracts a quaternion from another one.
    /// 
    /// Check [the sub function](crate::sub) in the root for more info.
    #[inline] fn sub(self, other: impl Quaternion<Num>) -> Self { quat::sub(self, other) }
    /// Muliplies a quaternion to another one.
    /// 
    /// Check [the mul function](crate::mul) in the root for more info.
    #[inline] fn mul(self, other: impl Quaternion<Num>) -> Self { quat::mul(self, other) }
    /// Muliplies a quaternion to another one in a reversed order.
    /// 
    /// Check [the mul_reversed function](crate::mul_reversed) in the root for more info.
    #[inline] fn mul_reversed(self, other: impl Quaternion<Num>) -> Self { quat::mul_reversed(self, other) }
    /// Divides a quaternion from another one.
    /// 
    /// Check [the div function](crate::div) in the root for more info.
    #[inline] fn div(self, other: impl Quaternion<Num>) -> Self { quat::div(self, other) }
    /// Divides a quaternion from another one.
    /// 
    /// Check [the div_reversed function](crate::div_reversed) in the root for more info.
    #[inline] fn div_reversed(self, other: impl Quaternion<Num>) -> Self { quat::div_reversed(self, other) }
    /// Scales a quaternion.
    /// 
    /// Equivalent to multiplying a quaternion by a scalar quaternion.
    /// 
    /// Check [the scale function](crate::scale) in the root for more info.
    #[inline] fn scale(self, other: impl Scalar<Num>) -> Self { quat::scale(self, other) }
    /// Scales a quaternion by the inverse of the scalar.
    /// 
    /// Equivalent to dividing a quaternion by a scalar quaternion.
    /// 
    /// Check [the unscale function](crate::unscale) in the root for more info.
    #[inline] fn unscale(self, other: impl Scalar<Num>) -> Self { quat::unscale(self, other) }
    /// Checks if a quaternion is a scalar value.
    /// 
    /// Check [the is_scalar function](crate::is_scalar) in the root for more info.
    #[inline] fn is_scalar(self) -> bool { quat::is_scalar(self) }
    /// Checks if a quaternion is a complex value.
    /// 
    /// Check [the is_complex function](crate::is_complex) in the root for more info.
    #[inline] fn is_complex(self) -> bool { quat::is_complex(self) }
    /// Checks if a quaternion is a vector value.
    /// 
    /// Check [the is_vector function](crate::is_vector) in the root for more info.
    #[inline] fn is_vector(self) -> bool { quat::is_vector(self) }
    /// Checks if a quaternion is on an axis plane.
    /// 
    /// Check [the is_on_axis_plane function](crate::is_on_axis_plane) in the root for more info.
    #[inline] fn is_on_axis_plane(self) -> bool { quat::is_on_axis_plane(self) }
    /// Checks if two quaternion represent the same value.
    /// 
    /// Check [the eq function](crate::eq) in the root for more info.
    #[inline] fn eq(self, other: impl Quaternion<Num>) -> bool { quat::eq(self, other) }
    /// Gets the absolute value of a quaternion. (Also knows as it's "length")
    /// 
    /// Check [the abs function](crate::abs) in the root for more info.
    #[inline] fn abs(self) -> Num { quat::abs(self) }
    /// Gets the squared absolute value of a quaternion. (Also knows as it's squared "length")
    /// 
    /// Check [the abs_squared function](crate::abs_squared) in the root for more info.
    #[inline] fn abs_squared(self) -> Num { quat::abs_squared(self) }
    /// Gets the dot product of two quaternions.
    /// 
    /// Check [the dot function](crate::dot) in the root for more info.
    #[inline] fn dot(self, other: impl Quaternion<Num>) -> Num { quat::dot(self, other) }
    /// Gets the normal of a quaternion.
    /// 
    /// Check [the norm function](crate::norm) in the root for more info.
    #[inline] fn norm(self) -> Self { quat::norm(self) }
    /// Gets the conjugate of a quaternion.
    /// 
    /// Check [the conj function](crate::conj) in the root for more info.
    #[inline] fn conj(self) -> Self { quat::conj(self) }
    /// Gets the negative of a quaternion.
    /// 
    /// Check [the neg function](crate::neg) in the root for more info.
    #[inline] fn neg(self) -> Self { quat::neg(self) }
    /// Gets the inverse of a quaternion.
    /// 
    /// Check [the inv function](crate::inv) in the root for more info.
    #[inline] fn inv(self) -> Self { quat::inv(self) }
    /// Checks if the distance inbetween two quaternions is less then [`Num::ERROR`](Axis::ERROR).
    /// 
    /// Check [the is_near function](crate::is_near) in the root for more info.
    #[inline] fn is_near(self, other: impl Quaternion<Num>) -> bool { quat::is_near(self, other) }
    /// Gets the distance inbetween the coordonates of two quaternions.
    /// 
    /// Check [the dist function](crate::dist) in the root for more info.
    #[inline] fn dist(self, other: impl Quaternion<Num>) -> Num { quat::dist(self, other) }
    /// Gets the square root of a quaternion.
    /// 
    /// Check [the sqrt function](crate::sqrt) in the root for more info.
    #[inline] fn sqrt(self) -> Self { quat::sqrt(self) }
    /// Raises a quaternion to an integer power.
    /// 
    /// Check [the pow_i function](crate::pow_i) in the root for more info.
    #[inline] fn pow_i(self, exp: i32) -> Self { quat::pow_i(self, exp) }
    /// Raises a quaternion to a positive integer power.
    /// 
    /// Check [the pow_u function](crate::pow_u) in the root for more info.
    #[inline] fn pow_u(self, exp: u32) -> Self { quat::pow_u(self, exp) }
    /// Raises a quaternion to a scalar power.
    /// 
    /// Check [the pow_f function](crate::pow_f) in the root for more info.
    #[inline] fn pow_f(self, exp: impl Scalar<Num>) -> Self { quat::pow_f(self, exp) }
    /// Raises a quaternion to a quaternion power.
    /// 
    /// Check [the pow_q function](crate::pow_q) in the root for more info.
    #[cfg(feature = "unstable")]
    #[inline] fn pow_q(self, exp: impl Quaternion<Num>) -> Self { quat::pow_q(self, exp) }
    /// Raises the number e to a quaternion power.
    /// 
    /// Check [the exp function](crate::exp) in the root for more info.
    #[inline] fn exp(self) -> Self { quat::exp(self) }
    /// Gets the natural logarithm of a quaternion.
    /// 
    /// Check [the ln function](crate::ln) in the root for more info.
    #[inline] fn ln(self) -> Self { quat::ln(self) }
    /// Gets the logarithm of a quaternion.
    /// 
    /// Check [the log function](crate::log) in the root for more info.
    #[cfg(feature = "unstable")]
    #[inline] fn log(self, base: impl Quaternion<Num>) -> Self { quat::log(self, base) }
    /// Gets the sinus of a quaternion.
    /// 
    /// Check [the sin function](crate::sin) in the root for more info.
    #[inline] fn sin(self) -> Self { quat::sin(self) }
    /// Gets the hyperbolic sinus of a quaternion.
    #[inline] fn sinh(self) -> Self { quat::sinh(self) }
    /// Gets the secant of a quaternion.
    #[inline] fn sec(self) -> Self { quat::sec(self) }
    /// Gets the cosinus of a quaternion.
    /// 
    /// Check [the cos function](crate::cos) in the root for more info.
    #[inline] fn cos(self) -> Self { quat::cos(self) }
    /// Gets the hyperbolic cosinus of a quaternion.
    #[inline] fn cosh(self) -> Self { quat::cosh(self) }
    /// Gets the cosecant of a quaternion.
    #[inline] fn csc(self) -> Self { quat::csc(self) }
    /// Gets the tangent of a quaternion.
    /// 
    /// Check [the tan function](crate::tan) in the root for more info.
    #[inline] fn tan(self) -> Self { quat::tan(self) }
    /// Gets the hyperbolic tangent of a quaternion.
    #[inline] fn tanh(self) -> Self { quat::tan(self) }
    /// Gets the vector part of a quaternion.
    /// 
    /// Check [the vector_part function](crate::vector_part) in the root for more info.
    #[inline] fn vector_part(self) -> Self { quat::vector_part(self) }
    /// Gets the complex part of a quaternion.
    /// 
    /// Check [the complex_part function](crate::complex_part) in the root for more info.
    #[inline] fn complex_part(self) -> Self { quat::complex_part(self) }
    /// Gets the scalar part of a quaternion.
    /// 
    /// Check [the scalar_part function](crate::scalar_part) in the root for more info.
    #[inline] fn scalar_part(self) -> Self { quat::scalar_part(self) }
    /// Turns a quaternion representation into a vector value representation.
    /// 
    /// Check [the to_vector function](crate::to_vector) in the root for more info.
    #[inline] fn to_vector<V: NewVector<Num>>(self) -> V { quat::to_vector(self) }
    /// Turns a quaternion representation into a complex number representation.
    /// 
    /// Check [the to_complex function](crate::to_complex) in the root for more info.
    #[inline] fn to_complex<C: NewComplex<Num>>(self) -> C { quat::to_complex(self) }
    /// Turns a quaternion representation into a scalar representation.
    /// 
    /// Check [the to_scalar function](crate::to_scalar) in the root for more info.
    #[inline] fn to_scalar<S: NewScalar<Num>>(self) -> S { quat::to_scalar(self) }
    /// Turns a quaternion representation into a rotation.
    /// 
    /// Check [the to_rotation function](crate::to_rotation) in the root for more info.
    #[inline] fn to_rotation<R: NewRotation<Num>>(self) -> R { quat::to_rotation(self) }
    /// Constructs a quaternion representation from a vector.
    /// 
    /// Check [the from_vector function](crate::from_vector) in the root for more info.
    #[inline] fn from_vector<V: Vector<Num>>(vector: V) -> Self { quat::from_vector(vector) }
    /// Constructs a quaternion representation from a complex number.
    /// 
    /// Check [the from_complex function](crate::from_complex) in the root for more info.
    #[inline] fn from_complex<C: Complex<Num>>(complex: C) -> Self { quat::from_complex(complex) }
    /// Constructs a quaternion representation from a scalar value.
    /// 
    /// Check [the from_scalar function](crate::from_scalar) in the root for more info.
    #[inline] fn from_scalar<S: Scalar<Num>>(scalar: S) -> Self { quat::from_scalar(scalar) }
    /// Constructs a unit quaternion representation from a rotation.
    /// 
    /// Check [the from_rotation function](crate::from_rotation) in the root for more info.
    #[inline] fn from_rotation<R: Rotation<Num>>(rotation: R) -> Self { quat::from_rotation(rotation) }
}

// Quat impls

impl<Num: Axis> Quaternion<Num> for () {
    #[inline] fn r(&self) -> Num { Num::ZERO }
    #[inline] fn i(&self) -> Num { Num::ZERO }
    #[inline] fn j(&self) -> Num { Num::ZERO }
    #[inline] fn k(&self) -> Num { Num::ZERO }
}

impl<Num: Axis> NewQuaternion<Num> for () {
    #[inline] fn new_quat(_: Num, _: Num, _: Num, _: Num) { }
    #[inline] fn from_quat(_: impl Quaternion<Num>) { }
}

impl<Num: Axis, T> Quaternion<Num> for [T; 0] {
    #[inline] fn r(&self) -> Num { Num::ZERO }
    #[inline] fn i(&self) -> Num { Num::ZERO }
    #[inline] fn j(&self) -> Num { Num::ZERO }
    #[inline] fn k(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> NewQuaternion<Num> for [T; 0] {
    #[inline] fn new_quat(_: Num, _: Num, _: Num, _: Num) -> Self { [] }
    #[inline] fn from_quat(_: impl Quaternion<Num>) -> Self { [] }
}

impl<Num: Axis, S, V> Quaternion<Num> for (S, V)
where 
    S: Scalar<Num>,
    V: Vector<Num>,
{
    #[inline] fn r(&self) -> Num { self.0.scalar() }
    #[inline] fn i(&self) -> Num { self.1.x()  }
    #[inline] fn j(&self) -> Num { self.1.y() }
    #[inline] fn k(&self) -> Num { self.1.z()  }
}

impl<Num: Axis, S, V> NewQuaternion<Num> for (S, V)
where 
    S: NewScalar<Num>,
    V: NewVector<Num>,
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> (S, V) {
        (
            NewScalar::new_scalar(r),
            NewVector::new_vector(i, j, k)
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
    #[inline] fn r(&self) -> Num { self.0.scalar() }
    #[inline] fn i(&self) -> Num { self.1.scalar() }
    #[inline] fn j(&self) -> Num { self.2.scalar() }
    #[inline] fn k(&self) -> Num { self.3.scalar() }
}

impl<Num: Axis, R, I, J, K> NewQuaternion<Num> for (R, I, J, K)
where
    R: NewScalar<Num>,
    I: NewScalar<Num>,
    J: NewScalar<Num>,
    K: NewScalar<Num>,
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> (R, I, J, K) {
        (
            NewScalar::new_scalar(r),
            NewScalar::new_scalar(i),
            NewScalar::new_scalar(j),
            NewScalar::new_scalar(k),
        )
    }
}

impl<Num: Axis, S> Quaternion<Num> for [S; 4]
where S: Scalar<Num>
{
    #[inline] fn r(&self) -> Num { self[0].scalar() }
    #[inline] fn i(&self) -> Num { self[1].scalar() }
    #[inline] fn j(&self) -> Num { self[2].scalar() }
    #[inline] fn k(&self) -> Num { self[3].scalar() }
}

impl<Num: Axis, S> NewQuaternion<Num> for [S; 4]
where S: NewScalar<Num>
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> [S; 4] {
        [
            NewScalar::new_scalar(r),
            NewScalar::new_scalar(i),
            NewScalar::new_scalar(j),
            NewScalar::new_scalar(k),
        ]
    }
}

impl<Num: Axis, C, J, K> Quaternion<Num> for (C, J, K)
where
    C: Complex<Num>,
    J: Scalar<Num>,
    K: Scalar<Num>,
{
    #[inline] fn r(&self) -> Num { self.0.real() }
    #[inline] fn i(&self) -> Num { self.0.imaginary() }
    #[inline] fn j(&self) -> Num { self.1.scalar() }
    #[inline] fn k(&self) -> Num { self.2.scalar() }
}

impl<Num: Axis, C, J, K> NewQuaternion<Num> for (C, J, K)
where
    C: NewComplex<Num>,
    J: NewScalar<Num>,
    K: NewScalar<Num>,
{
    #[inline] fn new_quat(r: Num, i: Num, j: Num, k: Num) -> (C, J, K) {
        (
            NewComplex::new_complex(r, i),
            NewScalar::new_scalar(j),
            NewScalar::new_scalar(k),
        )
    }
}

impl<Num: Axis, T> Quaternion<Num> for &T
where T: Quaternion<Num>
{
    fn r(&self) -> Num { (*self).r() }
    fn i(&self) -> Num { (*self).i() }
    fn j(&self) -> Num { (*self).j() }
    fn k(&self) -> Num { (*self).k() }
}

impl<Num: Axis> QuaternionMethods<Num> for () {}
impl<Num: Axis> QuaternionConsts<Num> for () {
    const ORIGIN: Self = ();
    const IDENTITY: Self = ();
    const NAN: Self = ();
    const UNIT_I: Self = ();
    const UNIT_J: Self = ();
    const UNIT_K: Self = ();
}

impl<Num: Axis, T> QuaternionMethods<Num> for [T; 0] {}
impl<Num: Axis, T> QuaternionConsts<Num> for [T; 0] {
    const ORIGIN: Self = [];
    const IDENTITY: Self = [];
    const NAN: Self = [];
    const UNIT_I: Self = [];
    const UNIT_J: Self = [];
    const UNIT_K: Self = [];
}

impl<Num: Axis, R, I, J, K> QuaternionMethods<Num> for (R, I, J, K)
where 
    R: Scalar<Num> + NewScalar<Num>,
    I: Scalar<Num> + NewScalar<Num>,
    J: Scalar<Num> + NewScalar<Num>,
    K: Scalar<Num> + NewScalar<Num>,
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
where S: Scalar<Num> + NewScalar<Num>
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
    S: Scalar<Num> + NewScalar<Num>,
    V: Vector<Num> + NewVector<Num>,
{}
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
    C: Complex<Num> + NewComplex<Num>,
    J: Scalar<Num> + NewScalar<Num>,
    K: Scalar<Num> + NewScalar<Num>,
{}
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

// Scalar impls

impl<Num: Axis> Scalar<Num> for () {
    #[inline] fn scalar(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> Scalar<Num> for [T; 0] {
    #[inline] fn scalar(&self) -> Num { Num::ZERO }
}

impl<Num: Axis> Scalar<Num> for Num {
    #[inline] fn scalar(&self) -> Num { *self }
}

impl<Num: Axis> ScalarConsts<Num> for Num {
    const ZERO: Self = <Num as Axis>::ZERO;
    const ONE: Self = <Num as Axis>::ONE;
    const NAN: Self = <Num as Axis>::NAN;
}

impl<From: Axis, To: Axis> NewScalar<From> for To
where From: Scalar<To>
{
    #[inline] fn new_scalar( scalar: From ) -> To { scalar.scalar() }
}

impl<Num: Axis, S> Scalar<Num> for (S, )
where S: Scalar<Num>
{
    #[inline] fn scalar(&self) -> Num { self.0.scalar() }
}

impl<Num: Axis, S> NewScalar<Num> for (S, )
where S: NewScalar<Num>
{
    #[inline] fn new_scalar( axis: Num ) -> (S, ) { (NewScalar::new_scalar(axis), ) }
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
    #[inline] fn scalar(&self) -> Num { self[0].scalar() }
}

impl<Num: Axis, S> NewScalar<Num> for [S; 1]
where S: NewScalar<Num>
{
    #[inline] fn new_scalar( axis: Num ) -> [S; 1] { [NewScalar::new_scalar(axis)] }
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
    #[inline] fn real(&self) -> Num { Num::ZERO }
    #[inline] fn imaginary(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> Complex<Num> for [T; 0] {
    #[inline] fn real(&self) -> Num { Num::ZERO }
    #[inline] fn imaginary(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, R, I> Complex<Num> for (R, I)
where 
    R: Scalar<Num>,
    I: Scalar<Num>,
{
    #[inline] fn real(&self) -> Num { self.0.scalar() }
    #[inline] fn imaginary(&self) -> Num { self.1.scalar() }
}

impl<Num: Axis, R, I> NewComplex<Num> for (R, I)
where 
    R: NewScalar<Num>,
    I: NewScalar<Num>,
{
    #[inline] fn new_complex(r: Num, i: Num) -> (R, I) {
        (
            NewScalar::new_scalar(r),
            NewScalar::new_scalar(i),
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
    #[inline] fn real(&self) -> Num { self[0].scalar() }
    #[inline] fn imaginary(&self) -> Num { self[1].scalar() }
}

impl<Num: Axis, S> NewComplex<Num> for [S; 2]
where 
    S: NewScalar<Num>,
{
    #[inline] fn new_complex(r: Num, i: Num) -> [S; 2] {
        [
            NewScalar::new_scalar(r),
            NewScalar::new_scalar(i),
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
    fn real(&self) -> Num { (*self).real() }
    fn imaginary(&self) -> Num { (*self).imaginary() }
}

// Vector impls

impl<Num: Axis> Vector<Num> for () {
    #[inline] fn x(&self) -> Num { Num::ZERO }
    #[inline] fn y(&self) -> Num { Num::ZERO }
    #[inline] fn z(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> Vector<Num> for [T; 0] {
    #[inline] fn x(&self) -> Num { Num::ZERO }
    #[inline] fn y(&self) -> Num { Num::ZERO }
    #[inline] fn z(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, X, Y, Z> Vector<Num> for (X, Y, Z)
where
    X: Scalar<Num>,
    Y: Scalar<Num>,
    Z: Scalar<Num>,
{
    #[inline] fn x(&self) -> Num { self.0.scalar() }
    #[inline] fn y(&self) -> Num { self.1.scalar() }
    #[inline] fn z(&self) -> Num { self.2.scalar() }
}

impl<Num: Axis, X, Y, Z> NewVector<Num> for (X, Y, Z)
where
    X: NewScalar<Num>,
    Y: NewScalar<Num>,
    Z: NewScalar<Num>,
{
    #[inline] fn new_vector(i: Num, j: Num, k: Num) -> (X, Y, Z) {
        (
            NewScalar::new_scalar(i),
            NewScalar::new_scalar(j),
            NewScalar::new_scalar(k),
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
    #[inline] fn x(&self) -> Num { self[0].scalar() }
    #[inline] fn y(&self) -> Num { self[1].scalar() }
    #[inline] fn z(&self) -> Num { self[2].scalar() }
}

impl<Num: Axis, S> NewVector<Num> for [S; 3]
where
    S: NewScalar<Num>,
{
    #[inline] fn new_vector(i: Num, j: Num, k: Num) -> [S; 3] {
        [
            NewScalar::new_scalar(i),
            NewScalar::new_scalar(j),
            NewScalar::new_scalar(k),
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
    fn x(&self) -> Num { (*self).x() }
    fn y(&self) -> Num { (*self).y() }
    fn z(&self) -> Num { (*self).z() }
}

// Rotation impls

impl<Num: Axis> Rotation<Num> for () {
    #[inline] fn roll(&self) -> Num { Num::ZERO }
    #[inline] fn pitch(&self) -> Num { Num::ZERO }
    #[inline] fn yaw(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, T> Rotation<Num> for [T; 0] {
    #[inline] fn roll(&self) -> Num { Num::ZERO }
    #[inline] fn pitch(&self) -> Num { Num::ZERO }
    #[inline] fn yaw(&self) -> Num { Num::ZERO }
}

impl<Num: Axis, X, Y, Z> Rotation<Num> for (X, Y, Z)
where
    X: Scalar<Num>,
    Y: Scalar<Num>,
    Z: Scalar<Num>,
{
    #[inline] fn roll(&self) -> Num { self.0.scalar() }
    #[inline] fn pitch(&self) -> Num { self.1.scalar() }
    #[inline] fn yaw(&self) -> Num { self.2.scalar() }
}

impl<Num: Axis, X, Y, Z> NewRotation<Num> for (X, Y, Z)
where
    X: NewScalar<Num>,
    Y: NewScalar<Num>,
    Z: NewScalar<Num>,
{
    #[inline] fn new_rotation(i: Num, j: Num, k: Num) -> (X, Y, Z) {
        (
            NewScalar::new_scalar(i),
            NewScalar::new_scalar(j),
            NewScalar::new_scalar(k),
        )
    }
}

impl<Num: Axis, S> Rotation<Num> for [S; 3]
where S: Scalar<Num>
{
    #[inline] fn roll(&self) -> Num { self[0].scalar() }
    #[inline] fn pitch(&self) -> Num { self[1].scalar() }
    #[inline] fn yaw(&self) -> Num { self[2].scalar() }
}

impl<Num: Axis, S> NewRotation<Num> for [S; 3]
where
    S: NewScalar<Num>,
{
    #[inline] fn new_rotation(i: Num, j: Num, k: Num) -> [S; 3] {
        [
            NewScalar::new_scalar(i),
            NewScalar::new_scalar(j),
            NewScalar::new_scalar(k),
        ]
    }
}

impl<Num: Axis, T> Rotation<Num> for &T
where T: Rotation<Num>
{
    fn roll(&self) -> Num { (*self).roll() }
    fn pitch(&self) -> Num { (*self).pitch() }
    fn yaw(&self) -> Num { (*self).yaw() }
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
    ( core $ty:ty $(: $( $trait:ident ),+ )? ) => {
        impl<Num: Axis, T> Quaternion<Num> for $ty
        where T: Quaternion<Num> $($( + $trait )+)?
        {
            fn r(&self) -> Num { (*(*self)).r() }
            fn i(&self) -> Num { (*(*self)).i() }
            fn j(&self) -> Num { (*(*self)).j() }
            fn k(&self) -> Num { (*(*self)).k() }
        }

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

        impl<Num: Axis, T> Rotation<Num> for $ty
        where T: Rotation<Num> $($( + $trait )+)?
        {
            fn roll(&self) -> Num { (*(*self)).roll() }
            fn pitch(&self) -> Num { (*(*self)).pitch() }
            fn yaw(&self) -> Num { (*(*self)).yaw() }
        }
    };
    ( $ty:ty $(: $( $trait:ident ),+ )? ) => {
        #[cfg(feature = "alloc")]
        impl<Num: Axis, T> Quaternion<Num> for $ty
        where T: Quaternion<Num> $($( + $trait )+)?
        {
            fn r(&self) -> Num { (*(*self)).r() }
            fn i(&self) -> Num { (*(*self)).i() }
            fn j(&self) -> Num { (*(*self)).j() }
            fn k(&self) -> Num { (*(*self)).k() }
        }

        #[cfg(feature = "alloc")]
        impl<Num: Axis, T> Vector<Num> for $ty
        where T: Vector<Num> $($( + $trait )+)?
        {
            fn x(&self) -> Num { (*(*self)).x() }
            fn y(&self) -> Num { (*(*self)).y() }
            fn z(&self) -> Num { (*(*self)).z() }
        }

        #[cfg(feature = "alloc")]
        impl<Num: Axis, T> Complex<Num> for $ty
        where T: Complex<Num> $($( + $trait )+)?
        {
            fn real(&self) -> Num { (*(*self)).real() }
            fn imaginary(&self) -> Num { (*(*self)).imaginary() }
        }

        #[cfg(feature = "alloc")]
        impl<Num: Axis, T> Rotation<Num> for $ty
        where T: Rotation<Num> $($( + $trait )+)?
        {
            fn roll(&self) -> Num { (*(*self)).roll() }
            fn pitch(&self) -> Num { (*(*self)).pitch() }
            fn yaw(&self) -> Num { (*(*self)).yaw() }
        }
    };
}

ref_impls!{Box<T>}
ref_impls!{Rc<T>}
ref_impls!{Arc<T>}
ref_impls!{Cow<'_, T>: ToOwned}

ref_impls!{core LazyCell<T>}
ref_impls!{core Ref<'_, T>}
ref_impls!{core RefMut<'_, T>}
ref_impls!{core ManuallyDrop<T>}
ref_impls!{core &mut T}

// Other impls

mod axis;

mod core_impls;

mod dep_impls;
