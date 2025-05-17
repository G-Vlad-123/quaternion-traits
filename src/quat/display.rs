
use crate::{Axis, Quaternion, QuaternionConstructor};
use crate::structs::QuaternionFormat;
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
/// Display a random quaternion.
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
/// Display origin.
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
/// Display multiple quaternions.
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
    format: QuaternionFormat,
) -> crate::core::fmt::Result {
    use crate::core::write;

    #[inline]
    fn write_first<Num: Axis + crate::core::fmt::Display, const AXIS: char>(target: &mut impl crate::core::fmt::Write, num: Num, format: crate::structs::QuaternionFormat) -> crate::core::fmt::Result {
        if (num != Num::ONE && num != -Num::ONE) || format.show_1s {
            if num < Num::ZERO {
                if format.add_spacing_for_first {
                    write!(target, "- {}{AXIS}", -num)
                } else {
                    write!(target, "{}{AXIS}", num)
                }
            } else {
                match (format.explicit_plus_sign, format.add_spacing_for_first) {
                    (false, _) => write!(target, "{}{AXIS}", num),
                    (true, false) => write!(target, "+{}{AXIS}", num),
                    (true, true) => write!(target, "+ {}{AXIS}", num),
                }
            }
        } else if num == Num::ONE {
            match (format.explicit_plus_sign, format.add_spacing_for_first) {
                (false, _) => write!(target, "{AXIS}"),
                (true, false) => write!(target, "+{AXIS}"),
                (true, true) => write!(target, "+ {AXIS}"),
            }
        } else {
            if format.add_spacing_for_first {
                write!(target, "- {AXIS}")
            } else {
                write!(target, "-{AXIS}")
            }
        }
    }

    #[inline]
    fn write_number<Num: Axis + crate::core::fmt::Display, const AXIS: char>(target: &mut impl crate::core::fmt::Write, num: Num, format: crate::structs::QuaternionFormat) -> crate::core::fmt::Result {
        if num > Num::ZERO {
            if num != Num::ONE || format.show_1s {
                if format.remove_spacing {
                    write!(target, "+{}{AXIS}", num)
                } else {
                    write!(target, " + {}{AXIS}", num)
                }
            } else {
                if format.remove_spacing {
                    write!(target, "+{AXIS}")
                } else {
                    write!(target, " + {AXIS}")
                }
            }
        } else if num < Num::ZERO {
            if num != -Num::ONE || format.show_1s {
                if format.remove_spacing {
                    write!(target, "{}{AXIS}", -num)
                } else {
                    write!(target, " - {}{AXIS}", -num)
                }
            } else {
                if format.remove_spacing {
                    write!(target, "-{AXIS}")
                } else {
                    write!(target, " - {AXIS}")
                }
            }
        } else if format.show_0s {
            if format.remove_spacing {
                write!(target, "+0{AXIS}")
            } else {
                write!(target, " + 0{AXIS}")
            }
        } else { Result::Ok(()) }
    }

    if quaternion.r() != Num::ZERO || format.show_0s {
        if format.explicit_real_axis {
            write_first::<Num, 'r'>(target, quaternion.r(), format)?;
        } else if quaternion.r() < Num::ZERO {
            if format.add_spacing_for_first {
                write!(target, "- {}", -quaternion.r())?;
            } else {
                write!(target, "{}", quaternion.r())?;
            }
        } else {
            match (format.explicit_plus_sign, format.add_spacing_for_first) {
                (false, _) => write!(target, "{}", quaternion.r()),
                (true, false) => write!(target, "+{}", quaternion.r()),
                (true, true) => write!(target, "+ {}", quaternion.r()),
            }?;
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

/// Alias for `display(target, quaternion, QuaternionFormat::DEFAULT)`
#[inline]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
pub fn default_display<Num: Axis + crate::core::fmt::Display>(
    target: &mut impl crate::core::fmt::Write,
    quaternion: impl Quaternion<Num>,
) -> crate::core::fmt::Result {
    display(target, quaternion, QuaternionFormat::DEFAULT)
}

#[cfg(feature = "alloc")]
use crate::alloc::string::String;

#[cfg(feature = "alloc")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Turns a quaternion representation into a [`String`].
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_string;
/// use quaternion_traits::structs::QuaternionFormat as QF;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, -2.0, 0.0];
/// 
/// let string: String = to_string::<f32>(quat, QF::DEFAULT).unwrap();
/// 
/// assert_eq!(
///     string,
///     String::from("i - 2j")
/// );
/// ```
pub fn to_string<Num: Axis + crate::core::fmt::Display>(quaternion: impl Quaternion<Num>, format: QuaternionFormat) -> Result<String, crate::core::fmt::Error> {
    let mut string = String::new();
    display(&mut string, quaternion, format)?;
    Result::Ok(string)
}

#[inline]
#[cfg(feature = "alloc")]
#[cfg_attr(all(test, panic = "abort"), no_panic::no_panic)]
/// Alias for `to_string(_, QuaternionFormat::DEFAULT)`.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::to_default_string;
/// 
/// let quat: [f32; 4] = [0.0, 1.0, -2.0, 0.0];
/// 
/// let string: String = to_default_string::<f32>(quat).unwrap();
/// 
/// assert_eq!(
///     string,
///     String::from("i - 2j")
/// );
/// ```
pub fn to_default_string<Num: Axis + crate::core::fmt::Display>(quaternion: impl Quaternion<Num>) -> Result<String, crate::core::fmt::Error> {
    to_string::<Num>(quaternion, QuaternionFormat::DEFAULT)
}

use crate::core::str::FromStr;

/// Parses a [`str`] into a quaternion representation.
/// 
/// # Example
/// ```
/// use quaternion_traits::quat::from_str;
/// 
/// let mut quat: [f32; 4];
/// 
/// // Parsing the real part
/// quat = from_str::<f32, _>("17.5").unwrap();
/// assert_eq!(quat, [17.5, 0.0, 0.0, 0.0]);
/// 
/// // Parsing the imaginary part
/// quat = from_str::<f32, _>("3i + j").unwrap();
/// assert_eq!(quat, [0.0, 3.0, 1.0, 0.0]);
/// 
/// // Explicit real part
/// quat = from_str::<f32, _>("4r - 17k").unwrap();
/// assert_eq!(quat, [4.0, 0.0, 0.0, -17.0]);
/// 
/// // No + required (just in case you want to make it look nice)
/// quat = from_str::<f32, _>("1 2i 3j 4k").unwrap();
/// assert_eq!(quat, [1.0, 2.0, 3.0, 4.0]);
/// 
/// // No spaces required! Numbers are read after they see an:
/// //    i, j, k, -, + along side ' ', '\\t', '\\n'
/// quat = from_str::<f32, _>("ijk").unwrap();
/// assert_eq!(quat, [0.0, 1.0, 1.0, 1.0]);
/// 
/// // Double negative
/// quat = from_str::<f32, _>(" - - 3j").unwrap();
/// assert_eq!(quat, [0.0, 0.0, 3.0, 0.0]);
/// 
/// // Uppercase
/// quat = from_str::<f32, _>("1R - 2I + 3J - 4K").unwrap();
/// assert_eq!(quat, [1.0, -2.0, 3.0, -4.0]);
/// 
/// // Any ordering
/// quat = from_str::<f32, _>("1j + 2i + 3 + 4k").unwrap();
/// assert_eq!(quat, [3.0, 2.0, 1.0, 4.0]);
/// ```
pub fn from_str<Num: Axis + FromStr, Out: QuaternionConstructor<Num>>(s: &str) -> Result<Out, <Num as FromStr>::Err> {
    use crate::core::option::Option::{*, self};
    
    let mut quat: [Num; 4] = [Num::ZERO; 4];
    let mut sign: Num = Num::ONE;
    let mut num: Option<(usize, usize)> = None;

    #[inline] fn read<Num: FromStr>(s: &str) -> Result<Num, <Num as FromStr>::Err> {
        s.parse::<Num>()
    }

    for (index, c) in s.char_indices() {
        match c {
            ' ' | '\t' | '\n' | '-' | '+' => {
                if let Some(n) = num {
                    quat[0] = quat[0] + sign * read(&s[n.0..=(n.0 + n.1)])?;
                    num = None;
                    sign = Num::ONE;
                }
                if c == '-' {sign = -sign}
            },
            'r' | 'R' => {
                if let Some(n) = num {
                    quat[0] = quat[0] + sign * read(&s[n.0..=(n.0 + n.1)])?;
                    num = None;
                    sign = Num::ONE;
                } else {
                    quat[0] = quat[0] + Num::ONE;
                }
            },
            'i' | 'I' => {
                if let Some(n) = num {
                    quat[1] = quat[1] + sign * read(&s[n.0..=(n.0 + n.1)])?;
                    num = None;
                    sign = Num::ONE;
                } else {
                    quat[1] = quat[1] + Num::ONE;
                }
            },
            'j' | 'J' => {
                if let Some(n) = num {
                    quat[2] = quat[2] + sign * read(&s[n.0..=(n.0 + n.1)])?;
                    num = None;
                    sign = Num::ONE;
                } else {
                    quat[2] = quat[2] + Num::ONE;
                }
            },
            'k' | 'K' => {
                if let Some(n) = num {
                    quat[3] = quat[3] + sign * read(&s[n.0..=(n.0 + n.1)])?;
                    num = None;
                    sign = Num::ONE;
                } else {
                    quat[3] = quat[3] + Num::ONE;
                }
            },
            _ => match num {
                Some((_, ref mut len)) => *len = *len + 1,
                None => num = Some((index, 0))
            },
        }
    }

    if let Some(n) = num {
        quat[0] = quat[0] + sign * read(&s[n.0..])?;
    }

    Result::Ok(Out::from_quat(quat))
}