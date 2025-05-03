
use crate::{Axis, Quaternion};
use crate::core::result::Result;

#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Writes a quaternion representation to a formatter/string.
/// 
/// This function gives each number in order.
/// With it's axis represented by the character that comes after it.
/// The real axis does not have a character.
/// `[1, 2, 3, 4]` -> `"1 + 2i + 3j + 4k"`
/// 
/// Negatives change the sign of the number shown inbetween.
/// `[1, -2, 3, -4]` -> `"1 - 2i + 3j - 4k"`
/// 
/// Except for the first number.
/// `[-1, 2, -3, 4]` -> `"-1 + 2i - 3j + 4k"`
/// 
/// Does not show the number if it's [`Num::ONE`](Axis::ONE)
/// `[1, 1, -1, 1]` -> `"1 + i - j + k"`
/// 
/// If it's [`Num::ZERO`](Axis::ZERO)
/// it's skipped.
/// `[0, 2, 0, 1]` -> `"2i + k"`
/// 
/// This affects how negatives are shown.
/// `[0, -2, 2, -2]` -> `"-2i + 2j - 2k"`
/// 
/// If the quaternion is the origin, [`Num::ZERO`](Axis::ZERO) is shown.
/// `[0, 0, 0, 0]` -> `"0"`
/// 
/// # Examples
/// ```
/// use quaternion_traits::quat::display;
/// use quaternion_traits::structs::QuaternionFormat as QF;
/// 
/// let quat: [i8; 4] = [2, 0, -3, 1];
/// 
/// let mut string = String::new();
/// 
/// display::<f32>(&mut string, quat, QF::DEFAULT);
/// 
/// assert_eq!(string.as_str(), "2 - 3j + k");
/// ```
/// 
/// ```
/// use quaternion_traits::quat::display;
/// use quaternion_traits::structs::QuaternionFormat as QF;
/// 
/// let mut string = String::new();
/// 
/// display::<f32>(&mut string, [0, 0, 0, 0], QF::DEFAULT);
/// 
/// assert_eq!(string.as_str(), "0");
/// ```
/// 
/// ```
/// use quaternion_traits::quat::display;
/// use quaternion_traits::structs::QuaternionFormat as QF;
/// use quaternion_traits::quat::add;
/// 
/// let a: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
/// let b: [f32; 4] = [3.0, -3.0, -3.0, 10.0];
/// let c: [f32; 4] = add::<f32, [f32; 4]>(a, b);
/// 
/// let mut out: String = String::new();
/// 
/// display::<f32>(&mut out, a, QF::DEFAULT);
/// out.push_str(" + ");
/// display::<f32>(&mut out, b, QF::DEFAULT);
/// out.push_str(" = ");
/// display::<f32>(&mut out, c, QF::DEFAULT);
/// 
/// assert_eq!(
///     out.as_str(),
///     "1 + 2i + 3j + 4k + 3 - 3i - 3j + 10k = 4 - i + 14k"
/// )
/// ```
/// 
/// # Note
/// This function assumes that axis is displayed as the primitive number types (i32, f64, etc)
pub fn display<Num: Axis + crate::core::fmt::Display>(
    target: &mut impl crate::core::fmt::Write,
    quaternion: impl Quaternion<Num>,
    format: crate::structs::QuaternionFormat, // TODO add functionality to show 0s
) -> crate::core::fmt::Result {
    use crate::core::write;

    #[inline]
    fn write_first<Num: Axis + crate::core::fmt::Display, const AXIS: char>(target: &mut impl crate::core::fmt::Write, num: Num, format: crate::structs::QuaternionFormat) -> crate::core::fmt::Result {
        if (num != Num::ONE && num != -Num::ONE) || format.show_1x_for_unit_values_of_x {
            if num < Num::ZERO {
                if format.add_spacing_for_first_sign {
                    write!(target, "- {}{AXIS}", -num)
                } else {
                    write!(target, "{}{AXIS}", num)
                }
            } else {
                match (format.add_plus_sign_for_first, format.add_spacing_for_first_sign) {
                    (false, _) => write!(target, "{}{AXIS}", num),
                    (true, false) => write!(target, "+{}{AXIS}", num),
                    (true, true) => write!(target, "+ {}{AXIS}", num),
                }
            }
        } else if num == Num::ONE {
            match (format.add_plus_sign_for_first, format.add_spacing_for_first_sign) {
                (false, _) => write!(target, "{AXIS}"),
                (true, false) => write!(target, "+{AXIS}"),
                (true, true) => write!(target, "+ {AXIS}"),
            }
        } else {
            if format.add_spacing_for_first_sign {
                write!(target, "- {AXIS}")
            } else {
                write!(target, "-{AXIS}")
            }
        }
    }

    #[inline]
    fn write_number<Num: Axis + crate::core::fmt::Display, const AXIS: char>(target: &mut impl crate::core::fmt::Write, num: Num, format: crate::structs::QuaternionFormat) -> crate::core::fmt::Result {
        if num > Num::ZERO {
            if num != Num::ONE || format.show_1x_for_unit_values_of_x {
                if format.remove_spacing_for_nonfirst_signs {
                    write!(target, "+{}{AXIS}", num)
                } else {
                    write!(target, " + {}{AXIS}", num)
                }
            } else {
                if format.remove_spacing_for_nonfirst_signs {
                    write!(target, "+{AXIS}")
                } else {
                    write!(target, " + {AXIS}")
                }
            }
        } else if num < Num::ZERO {
            if num != -Num::ONE || format.show_1x_for_unit_values_of_x {
                if format.remove_spacing_for_nonfirst_signs {
                    write!(target, "{}{AXIS}", -num)
                } else {
                    write!(target, " - {}{AXIS}", -num)
                }
            } else {
                if format.remove_spacing_for_nonfirst_signs {
                    write!(target, "-{AXIS}")
                } else {
                    write!(target, " - {AXIS}")
                }
            }
        } else if format.show_0x_for_zero_values {
            if format.remove_spacing_for_nonfirst_signs {
                write!(target, "+0{AXIS}")
            } else {
                write!(target, " + 0{AXIS}")
            }
        } else { Result::Ok(()) }
    }

    if format.show_0x_for_zero_values {
        if format.add_r_ro_real_axis {
            write_first::<Num, 'r'>(target, quaternion.r(), format)?;
        } else if format.add_spacing_for_first_sign {
            if quaternion.r() >= Num::ZERO {
                write!(target, "{}", quaternion.r())?;
            } else {
                write!(target, "- {}", -quaternion.r())?;
            }
        } else {
            write!(target, "{}", quaternion.r())?;
        }

        write_number::<Num, 'i'>(target, quaternion.i(), format)?;
        write_number::<Num, 'j'>(target, quaternion.j(), format)?;
        write_number::<Num, 'k'>(target, quaternion.k(), format)?;

        return Result::Ok(());
    }

    if quaternion.r() != Num::ZERO {
        if format.add_r_ro_real_axis {
            write_first::<Num, 'r'>(target, quaternion.r(), format)?;
        } else if format.add_spacing_for_first_sign {
            if quaternion.r() > Num::ZERO {
                write!(target, "{}", quaternion.r())?;
            } else {
                write!(target, "- {}", -quaternion.r())?;
            }
        } else {
            write!(target, "{}", quaternion.r())?;
        }

        write_number::<Num, 'i'>(target, quaternion.i(), format)?;
        write_number::<Num, 'j'>(target, quaternion.j(), format)?;
        write_number::<Num, 'k'>(target, quaternion.k(), format)?;

        return Result::Ok(());
    }


    if quaternion.i() != Num::ZERO {
        write_first::<Num, 'i'>(target, quaternion.i(), format)?;

        write_number::<Num, 'j'>(target, quaternion.j(), format)?;
        write_number::<Num, 'k'>(target, quaternion.k(), format)?;
        
        return Result::Ok(());
    }


    if quaternion.j() != Num::ZERO {
        write_first::<Num, 'j'>(target, quaternion.i(), format)?;

        write_number::<Num, 'k'>(target, quaternion.k(), format)?;

        return Result::Ok(());
    }

    if quaternion.k() != Num::ZERO {
        return write_first::<Num, 'k'>(target, quaternion.i(), format);
    }

    write!(target, "{}", Num::ZERO)
}

#[cfg(feature = "alloc")]
use crate::alloc::string::String;

#[cfg(feature = "alloc")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Turns a quaternion representation into a [String].
pub fn to_string<Num: Axis + crate::core::fmt::Display>(quaternion: impl Quaternion<Num>, format: crate::structs::QuaternionFormat) -> Result<String, crate::core::fmt::Error> {
    let mut string = String::new();
    display(&mut string, quaternion, format)?;
    Result::Ok(string)
}
