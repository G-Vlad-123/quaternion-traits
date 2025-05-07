
use super::*;

/**
Adds most functions in this crate as methods that take `self` and/or return `Self`.

Most methods have dry documentation and just link to their respective crate function.
If possible it's planned to have the same* documentation for both instances of each function
in the future.

\* With examples that match their respective function.
 */
pub trait QuaternionMethods<Num: Axis>: Quaternion<Num> + QuaternionConstructor<Num> + Sized {
    /// Adds two quaternions togheder.
    /// 
    /// Check [the add function](crate::quat::add) in the root for more info.
    #[inline] fn add(self, other: impl Quaternion<Num>) -> Self { quat::add(self, other) }
    /// Subtracts a quaternion from another one.
    /// 
    /// Check [the sub function](crate::quat::sub) in the root for more info.
    #[inline] fn sub(self, other: impl Quaternion<Num>) -> Self { quat::sub(self, other) }
    /// Muliplies a quaternion to another one.
    /// 
    /// Check [the mul function](crate::quat::mul) in the root for more info.
    #[inline] fn mul(self, other: impl Quaternion<Num>) -> Self { quat::mul(self, other) }
    /// Muliplies a quaternion to another one in a reversed order.
    /// 
    /// Check [the mul_reversed function](crate::quat::mul_reversed) in the root for more info.
    #[inline] fn mul_reversed(self, other: impl Quaternion<Num>) -> Self { quat::mul_reversed(self, other) }
    /// Divides a quaternion from another one.
    /// 
    /// Check [the div function](crate::quat::div) in the root for more info.
    #[inline] fn div(self, other: impl Quaternion<Num>) -> Self { quat::div(self, other) }
    /// Divides a quaternion from another one.
    /// 
    /// Check [the div_reversed function](crate::quat::div_reversed) in the root for more info.
    #[inline] fn div_reversed(self, other: impl Quaternion<Num>) -> Self { quat::div_reversed(self, other) }
    /// Scales a quaternion.
    /// 
    /// Equivalent to multiplying a quaternion by a scalar quaternion.
    /// 
    /// Check [the scale function](crate::quat::scale) in the root for more info.
    #[inline] fn scale(self, other: impl Scalar<Num>) -> Self { quat::scale(self, other) }
    /// Scales a quaternion by the inverse of the scalar.
    /// 
    /// Equivalent to dividing a quaternion by a scalar quaternion.
    /// 
    /// Check [the unscale function](crate::quat::unscale) in the root for more info.
    #[inline] fn unscale(self, other: impl Scalar<Num>) -> Self { quat::unscale(self, other) }
    /// Checks if a quaternion is a scalar value.
    /// 
    /// Check [the is_scalar function](crate::quat::is_scalar) in the root for more info.
    #[inline] fn is_scalar(self) -> bool { quat::is_scalar(self) }
    /// Checks if a quaternion is a complex value.
    /// 
    /// Check [the is_complex function](crate::quat::is_complex) in the root for more info.
    #[inline] fn is_complex(self) -> bool { quat::is_complex(self) }
    /// Checks if a quaternion is a vector value.
    /// 
    /// Check [the is_vector function](crate::quat::is_vector) in the root for more info.
    #[inline] fn is_vector(self) -> bool { quat::is_vector(self) }
    /// Checks if a quaternion is on an axis plane.
    /// 
    /// Check [the is_on_axis_plane function](crate::quat::is_on_axis_plane) in the root for more info.
    #[inline] fn is_on_axis_plane(self) -> bool { quat::is_on_axis_plane(self) }
    /// Checks if two quaternion represent the same value.
    /// 
    /// Check [the eq function](crate::quat::eq) in the root for more info.
    #[inline] fn eq(self, other: impl Quaternion<Num>) -> bool { quat::eq(self, other) }
    /// Gets the absolute value of a quaternion. (Also knows as it's "length")
    /// 
    /// Check [the abs function](crate::quat::abs) in the root for more info.
    #[inline] fn abs(self) -> Num { quat::abs(self) }
    /// Gets the squared absolute value of a quaternion. (Also knows as it's squared "length")
    /// 
    /// Check [the abs_squared function](crate::quat::abs_squared) in the root for more info.
    #[inline] fn abs_squared(self) -> Num { quat::abs_squared(self) }
    /// Gets the angle from a quaternion's polar form.
    /// 
    /// Check [the angle function](crate::quat::angle) in the root for more info.
    #[inline] fn angle(self) -> Num { quat::angle(self) }
    /// Gets the cosine of the angle from a quaternion's polar form.
    /// 
    /// Check [the angle_cos function](crate::quat::angle_cos) in the root for more info.
    #[inline] fn angle_cos(self) -> Num { quat::angle_cos(self) }
    /// Gets the dot product of two quaternions.
    /// 
    /// Check [the dot function](crate::quat::dot) in the root for more info.
    #[inline] fn dot(self, other: impl Quaternion<Num>) -> Num { quat::dot(self, other) }
    /// Normalizes a quaternion.
    /// 
    /// Check [the normalize function](crate::quat::normalize) in the root for more info.
    #[inline] fn norm(self) -> Self { quat::normalize(self) }
    /// Gets the conjugate of a quaternion.
    /// 
    /// Check [the conj function](crate::quat::conj) in the root for more info.
    #[inline] fn conj(self) -> Self { quat::conj(self) }
    /// Gets the negative of a quaternion.
    /// 
    /// Check [the neg function](crate::quat::neg) in the root for more info.
    #[inline] fn neg(self) -> Self { quat::neg(self) }
    /// Gets the inverse of a quaternion.
    /// 
    /// Check [the inv function](crate::quat::inv) in the root for more info.
    #[inline] fn inv(self) -> Self { quat::inv(self) }
    /// Checks if the distance inbetween two quaternions is less then [`Num::ERROR`](Axis::ERROR).
    /// 
    /// Check [the is_near function](crate::quat::is_near) in the root for more info.
    #[inline] fn is_near(self, other: impl Quaternion<Num>) -> bool { quat::is_near(self, other) }
    /// Checks if the distance inbetween two quaternions is less then error.
    /// 
    /// Check [the is_near_by function](crate::quat::is_near_by) in the root for more info.
    #[inline] fn is_near_by(self, other: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool { quat::is_near_by(self, other, error) }
    /// Checks if the ratio inbetween the absolute values of each quaternion
    /// are near [Num::ONE](Axis::ONE) by a margin of [Num::ERROR](Axis::ERROR).
    /// 
    /// Check [the is_close function](crate::quat::is_close) in the root for more info.
    #[inline] fn is_close(self, other: impl Quaternion<Num>) -> bool { quat::is_close(self, other) }
    /// Checks if the ratio inbetween the absolute values of each quaternion
    /// are near [Num::ONE](Axis::ONE) by a margin of `error`.
    /// 
    /// Check [the is_close_by function](crate::quat::is_close_by) in the root for more info.
    #[inline] fn is_close_by(self, other: impl Quaternion<Num>, error: impl Scalar<Num>) -> bool { quat::is_close_by(self, other, error) }
    /// Gets the distance inbetween the coordonates of two quaternions.
    /// 
    /// Check [the dist_euclid function](crate::quat::dist_euclid) in the root for more info.
    #[inline] fn dist_euclid(self, other: impl Quaternion<Num>) -> Num { quat::dist_euclid(self, other) }
    /// Calculates the cosine distance between two quaternions.
    #[inline] fn dist_cosine(self, other: impl Quaternion<Num>) -> Num { quat::dist_cosine(self, other) }
    /// Gets the square root of a quaternion.
    /// 
    /// Check [the sqrt function](crate::quat::sqrt) in the root for more info.
    #[inline] fn sqrt(self) -> Self { quat::sqrt(self) }
    /// Raises a quaternion to an integer power.
    /// 
    /// Check [the pow_i function](crate::quat::pow_i) in the root for more info.
    #[inline] fn pow_i(self, exp: i32) -> Self { quat::pow_i(self, exp) }
    /// Raises a quaternion to a positive integer power.
    /// 
    /// Check [the pow_u function](crate::quat::pow_u) in the root for more info.
    #[inline] fn pow_u(self, exp: u32) -> Self { quat::pow_u(self, exp) }
    /// Raises a quaternion to a scalar power.
    /// 
    /// Check [the pow_f function](crate::quat::pow_f) in the root for more info.
    #[inline] fn pow_f(self, exp: impl Scalar<Num>) -> Self { quat::pow_f(self, exp) }
    /// Raises a quaternion to a quaternion power.
    /// 
    /// Check [the pow_q function](crate::quat::pow_q) in the root for more info.
    #[cfg(feature = "unstable")]
    #[inline] fn pow_q(self, exp: impl Quaternion<Num>) -> Self { quat::pow_q(self, exp) }
    /// Raises the number e to a quaternion power.
    /// 
    /// Check [the exp function](crate::quat::exp) in the root for more info.
    #[inline] fn exp(self) -> Self { quat::exp(self) }
    /// Gets the natural logarithm of a quaternion.
    /// 
    /// Check [the ln function](crate::quat::ln) in the root for more info.
    #[inline] fn ln(self) -> Self { quat::ln(self) }
    /// Gets the logarithm of a quaternion.
    /// 
    /// Check [the log function](crate::quat::log) in the root for more info.
    #[cfg(feature = "unstable")]
    #[inline] fn log(self, base: impl Quaternion<Num>) -> Self { quat::log(self, base) }
    /// Gets the sinus of a quaternion.
    #[inline] fn sin(self) -> Self { quat::sin(self) }
    /// Gets the hyperbolic sinus of a quaternion.
    #[inline] fn sinh(self) -> Self { quat::sinh(self) }
    /// Gets the secant of a quaternion.
    #[inline] fn sec(self) -> Self { quat::sec(self) }
    /// Gets the cosinus of a quaternion.
    #[inline] fn cos(self) -> Self { quat::cos(self) }
    /// Gets the hyperbolic cosinus of a quaternion.
    #[inline] fn cosh(self) -> Self { quat::cosh(self) }
    /// Gets the cosecant of a quaternion.
    #[inline] fn csc(self) -> Self { quat::csc(self) }
    /// Gets the tangent of a quaternion.
    #[inline] fn tan(self) -> Self { quat::tan(self) }
    /// Gets the hyperbolic tangent of a quaternion.
    #[inline] fn tanh(self) -> Self { quat::tanh(self) }
    /// Gets the tangent of a quaternion.
    #[inline] fn cot(self) -> Self { quat::cot(self) }
    /// Gets the hyperbolic tangent of a quaternion.
    #[inline] fn coth(self) -> Self { quat::coth(self) }
    /// Gets the arcsinus of a quaternion.
    #[inline] fn asin(self) -> Self { quat::asin(self) }
    /// Gets the inverse hyperbolic sinus of a quaternion.
    #[inline] fn asinh(self) -> Self { quat::asinh(self) }
    /// Gets the arcsecant of a quaternion.
    #[inline] fn asec(self) -> Self { quat::asec(self) }
    /// Gets the arccosinus of a quaternion.
    #[inline] fn acos(self) -> Self { quat::acos(self) }
    /// Gets the inverse hyperbolic cosinus of a quaternion.
    #[inline] fn acosh(self) -> Self { quat::acosh(self) }
    /// Gets the arccosecant of a quaternion.
    #[inline] fn acsc(self) -> Self { quat::acsc(self) }
    /// Gets the arctangent of a quaternion.
    #[inline] fn atan(self) -> Self { quat::atan(self) }
    /// Gets the inverse hyperbolic tangent of a quaternion.
    #[inline] fn atanh(self) -> Self { quat::atanh(self) }
    /// Gets the arctangent of a quaternion.
    #[inline] fn acot(self) -> Self { quat::acot(self) }
    /// Gets the inverse hyperbolic tangent of a quaternion.
    #[inline] fn acoth(self) -> Self { quat::acoth(self) }
    /// Gets the vector part of a quaternion.
    /// 
    /// Check [the vector_part function](crate::quat::vector_part) in the root for more info.
    #[inline] fn vector_part(self) -> Self { quat::vector_part(self) }
    /// Gets the complex part of a quaternion.
    /// 
    /// Check [the complex_part function](crate::quat::complex_part) in the root for more info.
    #[inline] fn complex_part(self) -> Self { quat::complex_part(self) }
    /// Gets the scalar part of a quaternion.
    /// 
    /// Check [the scalar_part function](crate::quat::scalar_part) in the root for more info.
    #[inline] fn scalar_part(self) -> Self { quat::scalar_part(self) }
    /// Turns a quaternion representation into a vector value representation.
    /// 
    /// Check [the to_vector function](crate::quat::to_vector) in the root for more info.
    #[inline] fn to_vector<V: VectorConstructor<Num>>(self) -> V { quat::to_vector(self) }
    /// Turns a quaternion representation into a complex number representation.
    /// 
    /// Check [the to_complex function](crate::quat::to_complex) in the root for more info.
    #[inline] fn to_complex<C: ComplexConstructor<Num>>(self) -> C { quat::to_complex(self) }
    /// Turns a quaternion representation into a scalar representation.
    /// 
    /// Check [the to_scalar function](crate::quat::to_scalar) in the root for more info.
    #[inline] fn to_scalar<S: ScalarConstructor<Num>>(self) -> S { quat::to_scalar(self) }
    /// Turns a quaternion representation into a rotation.
    /// 
    /// Check [the to_rotation function](crate::quat::to_rotation) in the root for more info.
    #[inline] fn to_rotation<R: RotationConstructor<Num>>(self) -> R { quat::to_rotation(self) }
    /// Gets the polar form of a quaternion.
    /// 
    /// Check [the to_polar_form function](crate::quat::to_polar_form) in the root for more info.
    #[inline] fn to_polar_form<Abs, Angle, UnitVec>(self) -> (Abs, Angle, UnitVec)
    where 
        Abs: ScalarConstructor<Num>,
        Angle: ScalarConstructor<Num>,
        UnitVec: VectorConstructor<Num>,
    { quat::to_polar_form(self) }
    /// Turns a quaternion representation into a 2x2 complex matrix.
    #[inline] fn to_matrix_2<C: ComplexConstructor<Num>, M: MatrixConstructor<C, 2>>(self) -> M { quat::to_matrix_2(self) }
    /// Turns a quaternion representation into a 3x3 matrix (DCM).
    #[inline] fn to_matrix_3<M: MatrixConstructor<Num, 3>>(self) -> M { quat::to_matrix_3(self) }
    /// Turns a quaternion representation into a 4x4 matrix.
    #[inline] fn to_matrix_4<M: MatrixConstructor<Num, 4>>(self) -> M { quat::to_matrix_4(self) }
    /// Constructs a quaternion representation from a vector.
    /// 
    /// Check [the from_vector function](crate::quat::from_vector) in the root for more info.
    #[inline] fn from_vector<V: Vector<Num>>(vector: V) -> Self { quat::from_vector(vector) }
    /// Constructs a quaternion representation from a complex number.
    /// 
    /// Check [the from_complex function](crate::quat::from_complex) in the root for more info.
    #[inline] fn from_complex<C: Complex<Num>>(complex: C) -> Self { quat::from_complex(complex) }
    /// Constructs a quaternion representation from a scalar value.
    /// 
    /// Check [the from_scalar function](crate::quat::from_scalar) in the root for more info.
    #[inline] fn from_scalar<S: Scalar<Num>>(scalar: S) -> Self { quat::from_scalar(scalar) }
    /// Constructs a unit quaternion representation from a rotation.
    /// 
    /// Check [the from_rotation function](crate::quat::from_rotation) in the root for more info.
    #[inline] fn from_rotation<R: Rotation<Num>>(rotation: R) -> Self { quat::from_rotation(rotation) }
    /// Constructs a unit quaternion representation from a rotation.
    /// 
    /// Check [the from_polar_form function](crate::quat::from_polar_form) in the root for more info.
    #[inline] fn from_polar_form<Abs, Angle, UnitVec>(abs: Abs, angle: Angle, unit_vec: UnitVec) -> Option<Self>
    where 
        Abs: Scalar<Num>,
        Angle: Scalar<Num>,
        UnitVec: Vector<Num>,
    { quat::from_polar_form(abs, angle, unit_vec) }
    /// Constructs a unit quaternion representation from a rotation.
    /// 
    /// Check [the from_matrix_2 function](crate::quat::from_matrix_2) in the root for more info.
    #[inline] fn from_matrix_2<M: Matrix<Elem, 2>, Elem: Complex<Num>>(matrix: M) -> Option<Self> { quat::from_matrix_2(matrix) }
    /// Constructs a unit quaternion representation from a rotation.
    /// 
    /// Check [the from_matrix_3 function](crate::quat::from_matrix_3) in the root for more info.
    #[inline] fn from_matrix_3<M: Matrix<Elem, 3>, Elem: Scalar<Num>>(matrix: M) -> Self { quat::from_matrix_3(matrix) }
    /// Constructs a unit quaternion representation from a rotation.
    /// 
    /// Check [the from_matrix_4 function](crate::quat::from_matrix_4) in the root for more info.
    #[inline] fn from_matrix_4<M: Matrix<Elem, 4>, Elem: Scalar<Num>>(matrix: M) -> Self { quat::from_matrix_4::<Num, Elem, Self>(matrix) }
}
