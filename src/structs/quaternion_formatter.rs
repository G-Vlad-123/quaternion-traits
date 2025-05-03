
use crate::core::ops::{
    Add, Sub,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Not,
};

/// Settings for formatting in the [`display`](crate::quat::display) function.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuaternionFormat {
    /// Adds spacing for the first number.
    /// 
    /// `add_spacing_for_first_sign` = `false`: 
    /// `[-1, 2, 0, 0]` -> `"-1 + 2i"`
    /// 
    /// `add_spacing_for_first_sign` = `true`: 
    /// `[-1, 2, 0, 0]` -> `"- 1 + 2i"`
    pub add_spacing_for_first_sign: bool,
    /// Removes spacing from the non-first numbers.
    /// 
    /// `remove_spacing_for_nonfirst_signs` = `false`:
    /// `[-1, 2, 0, 0]` -> `"-1 + 2i"`
    /// 
    /// `remove_spacing_for_nonfirst_signs` = `true`:
    /// `[-1, 2, 0, 0]` -> `"-1+2i"`
    pub remove_spacing_for_nonfirst_signs: bool,
    /// Shows 1 for units of the imaginary axies.
    /// 
    /// `show_1x_for_unit_values_of_x` = `false`:
    /// `[0, 1, 2, -1]` -> `"i + 2j - k"`
    /// 
    /// `show_1x_for_unit_values_of_x` = `true`:
    /// `[0, 1, 2, -1]` -> `"1i + 2j - 1k"`
    pub show_1x_for_unit_values_of_x: bool,
    /// Adds r to represent the real unit.
    /// 
    /// `add_r_ro_real_axis` = `false`:
    /// `[3, 0, 2, 0]` -> `"3 + 2j"` and
    /// `[1, 0, 0,-1]` -> `"1 - k"`
    /// 
    /// `add_r_ro_real_axis` = `true`:
    /// `[3, 0, 2, 0]` -> `"3r + 2j"` and
    /// `[1, 0, 0,-1]` -> `"r - k"`
    pub add_r_ro_real_axis: bool,
    /// Adds `+` to the first number if it's positive.
    /// 
    /// `add_plus_signfor_first` = `false`:
    /// `[3, 0, -3, 0]` -> `"3 - 3j"`
    /// 
    /// `add_plus_signfor_first` = `true`:
    /// `[3, 0, -3, 0]` -> `"+3 - 3j"
    pub add_plus_sign_for_first: bool,
}

impl QuaternionFormat {
    /// The default formatter. All false.
    pub const DEFAULT: Self = QuaternionFormat {
        add_spacing_for_first_sign: false,
        remove_spacing_for_nonfirst_signs: false,
        show_1x_for_unit_values_of_x: false,
        add_r_ro_real_axis: false,
        add_plus_sign_for_first: false,
    };

    /// Adds spacing inbetween all the numbers.
    /// Has only `add_spacing_for_first_sign` set to true.
    pub const ALL_SPACING: Self = QuaternionFormat {
        add_spacing_for_first_sign: true,
        remove_spacing_for_nonfirst_signs: false,
        show_1x_for_unit_values_of_x: false,
        add_r_ro_real_axis: false,
        add_plus_sign_for_first: false,
    };

    /// Removes all spacing inbetween numbers.
    /// Has only `remove_spacing_for_nonfirst_signs` set to true.
    pub const NO_SPACING: Self = QuaternionFormat {
        add_spacing_for_first_sign: false,
        remove_spacing_for_nonfirst_signs: true,
        show_1x_for_unit_values_of_x: false,
        add_r_ro_real_axis: false,
        add_plus_sign_for_first: false,
    };

    /// Shows 1s for units on the imaginary axies.
    /// Has only `show_1x_for_unit_values_of_x` set to true.
    pub const SHOW_1S: Self = QuaternionFormat {
        add_spacing_for_first_sign: false,
        remove_spacing_for_nonfirst_signs: false,
        show_1x_for_unit_values_of_x: true,
        add_r_ro_real_axis: false,
        add_plus_sign_for_first: false,
    };

    /// Adds the `'r'` char to the end of the real part.
    /// Has only `add_r_ro_real_axis` set to true.
    pub const EXPLICIT_REAL_AXIS: Self = QuaternionFormat {
        add_spacing_for_first_sign: false,
        remove_spacing_for_nonfirst_signs: false,
        show_1x_for_unit_values_of_x: false,
        add_r_ro_real_axis: true,
        add_plus_sign_for_first: false,
    };

    /// Adds the `'+'` char to the start of the first number when positive.
    /// Has only `add_plus_sign_for_first` set to true.
    pub const EXPLICIT_PLUS_SIGN: Self = QuaternionFormat {
        add_spacing_for_first_sign: false,
        remove_spacing_for_nonfirst_signs: false,
        show_1x_for_unit_values_of_x: false,
        add_r_ro_real_axis: false,
        add_plus_sign_for_first: true,
    };

    #[inline]
    /// Compibes the two formats (like an or operation).
    pub const fn with(self, addon: Self) -> QuaternionFormat {
        QuaternionFormat {
            add_spacing_for_first_sign:
                self.add_spacing_for_first_sign
             || addon.add_spacing_for_first_sign,

            remove_spacing_for_nonfirst_signs:
                self.remove_spacing_for_nonfirst_signs
             || addon.remove_spacing_for_nonfirst_signs,

            show_1x_for_unit_values_of_x:
                self.show_1x_for_unit_values_of_x
             || addon.show_1x_for_unit_values_of_x,

            add_r_ro_real_axis:
                self.add_r_ro_real_axis
             || addon.add_r_ro_real_axis,
             
            add_plus_sign_for_first:
                self.add_plus_sign_for_first
             || addon.add_plus_sign_for_first,
        }
    }

    #[inline]
    /// Sets to false all the flags that have true in `remove`.
    pub const fn without(self, remove: Self) -> QuaternionFormat {
        QuaternionFormat {
            add_spacing_for_first_sign:
                self.add_spacing_for_first_sign
             && !remove.add_spacing_for_first_sign,

            remove_spacing_for_nonfirst_signs:
                self.remove_spacing_for_nonfirst_signs
             && !remove.remove_spacing_for_nonfirst_signs,

            show_1x_for_unit_values_of_x:
                self.show_1x_for_unit_values_of_x
             && !remove.show_1x_for_unit_values_of_x,

            add_r_ro_real_axis:
                self.add_r_ro_real_axis
             && !remove.add_r_ro_real_axis,
             
            add_plus_sign_for_first:
                self.add_plus_sign_for_first
             && !remove.add_plus_sign_for_first,
        }
    }
}

impl BitAnd for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn bitand(self, other: Self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first_sign:
                self.add_spacing_for_first_sign
             && other.add_spacing_for_first_sign,

            remove_spacing_for_nonfirst_signs:
                self.remove_spacing_for_nonfirst_signs
             && other.remove_spacing_for_nonfirst_signs,

            show_1x_for_unit_values_of_x:
                self.show_1x_for_unit_values_of_x
             && other.show_1x_for_unit_values_of_x,

            add_r_ro_real_axis:
                self.add_r_ro_real_axis
             && other.add_r_ro_real_axis,
             
            add_plus_sign_for_first:
                self.add_plus_sign_for_first
             && other.add_plus_sign_for_first,
        }
    }
}

impl BitOr for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first_sign:
                self.add_spacing_for_first_sign
             || other.add_spacing_for_first_sign,

            remove_spacing_for_nonfirst_signs:
                self.remove_spacing_for_nonfirst_signs
             || other.remove_spacing_for_nonfirst_signs,

            show_1x_for_unit_values_of_x:
                self.show_1x_for_unit_values_of_x
             || other.show_1x_for_unit_values_of_x,

            add_r_ro_real_axis:
                self.add_r_ro_real_axis
             || other.add_r_ro_real_axis,
             
            add_plus_sign_for_first:
                self.add_plus_sign_for_first
             || other.add_plus_sign_for_first,
        }
    }
}

impl BitXor for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn bitxor(self, other: Self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first_sign:
                self.add_spacing_for_first_sign
              ^ other.add_spacing_for_first_sign,

            remove_spacing_for_nonfirst_signs:
                self.remove_spacing_for_nonfirst_signs
              ^ other.remove_spacing_for_nonfirst_signs,

            show_1x_for_unit_values_of_x:
                self.show_1x_for_unit_values_of_x
              ^ other.show_1x_for_unit_values_of_x,

            add_r_ro_real_axis:
                self.add_r_ro_real_axis
              ^ other.add_r_ro_real_axis,
             
            add_plus_sign_for_first:
                self.add_plus_sign_for_first
              ^ other.add_plus_sign_for_first,
        }
    }
}

impl Add for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.with(other)
    }
}

impl Sub for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.without(other)
    }
}

impl BitAndAssign for QuaternionFormat {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.add_spacing_for_first_sign &= other.add_spacing_for_first_sign;
        self.remove_spacing_for_nonfirst_signs &= other.remove_spacing_for_nonfirst_signs;
        self.show_1x_for_unit_values_of_x &= other.show_1x_for_unit_values_of_x;
        self.add_r_ro_real_axis &= other.add_r_ro_real_axis;
        self.add_plus_sign_for_first &= other.add_plus_sign_for_first;
    }
}

impl BitOrAssign for QuaternionFormat {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.add_spacing_for_first_sign |= other.add_spacing_for_first_sign;
        self.remove_spacing_for_nonfirst_signs |= other.remove_spacing_for_nonfirst_signs;
        self.show_1x_for_unit_values_of_x |= other.show_1x_for_unit_values_of_x;
        self.add_r_ro_real_axis |= other.add_r_ro_real_axis;
        self.add_plus_sign_for_first |= other.add_plus_sign_for_first;
    }
}

impl BitXorAssign for QuaternionFormat {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.add_spacing_for_first_sign ^= other.add_spacing_for_first_sign;
        self.remove_spacing_for_nonfirst_signs ^= other.remove_spacing_for_nonfirst_signs;
        self.show_1x_for_unit_values_of_x ^= other.show_1x_for_unit_values_of_x;
        self.add_r_ro_real_axis ^= other.add_r_ro_real_axis;
        self.add_plus_sign_for_first ^= other.add_plus_sign_for_first;
    }
}

impl Not for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn not(self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first_sign:        !self.add_spacing_for_first_sign,
            remove_spacing_for_nonfirst_signs: !self.remove_spacing_for_nonfirst_signs,
            show_1x_for_unit_values_of_x:      !self.show_1x_for_unit_values_of_x,
            add_r_ro_real_axis:                !self.add_r_ro_real_axis,
            add_plus_sign_for_first:           !self.add_plus_sign_for_first
        }
    }
}
