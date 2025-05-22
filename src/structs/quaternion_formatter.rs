
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
    /// `add_spacing_for_first` = `false`: 
    /// `[-1, 2, 0, 0]` -> `"-1 + 2i"`
    /// 
    /// `add_spacing_for_first` = `true`: 
    /// `[-1, 2, 0, 0]` -> `"- 1 + 2i"`
    pub add_spacing_for_first: bool,
    /// Removes spacing from the non-first numbers.
    /// 
    /// `remove_spacing` = `false`:
    /// `[-1, 2, 0, 0]` -> `"-1 + 2i"`
    /// 
    /// `remove_spacing` = `true`:
    /// `[-1, 2, 0, 0]` -> `"-1+2i"`
    pub remove_spacing: bool,
    /// Shows 1 for units of the imaginary axies.
    /// 
    /// `show_1s` = `false`:
    /// `[0, 1, 2, -1]` -> `"i + 2j - k"`
    /// 
    /// `show_1s` = `true`:
    /// `[0, 1, 2, -1]` -> `"1i + 2j - 1k"`
    pub show_1s: bool,
    /// Adds r to represent the real unit.
    /// 
    /// `explicit_real_axis` = `false`:
    /// `[3, 0, 2, 0]` -> `"3 + 2j"` and
    /// `[1, 0, 0,-1]` -> `"1 - k"`
    /// 
    /// `explicit_real_axis` = `true`:
    /// `[3, 0, 2, 0]` -> `"3r + 2j"` and
    /// `[1, 0, 0,-1]` -> `"r - k"`
    pub explicit_real_axis: bool,
    /// Adds `+` to the first number if it's positive.
    /// 
    /// `explicit_plus_sign` = `false`:
    /// `[3, 0, -3, 0]` -> `"3 - 3j"`
    /// 
    /// `explicit_plus_sign` = `true`:
    /// `[3, 0, -3, 0]` -> `"+3 - 3j"
    pub explicit_plus_sign: bool,
    /// Disables skipping for axies that are 0.
    /// 
    /// `show_0s` = `false`:
    /// `[0, 1, 0, -2]` -> `"i - 2k"`
    /// 
    /// `show_0s` = `true`:
    /// `[0, 1, 0, -2]` -> `"0 + i + 0j - 2k"`
    pub show_0s: bool,
}

impl QuaternionFormat {
    /// The default formatter. All false.
    pub const DEFAULT: Self = QuaternionFormat {
        add_spacing_for_first: false,
        remove_spacing: false,
        show_1s: false,
        explicit_real_axis: false,
        explicit_plus_sign: false,
        show_0s: false,
    };

    /// Adds spacing inbetween all the numbers.
    /// Has only `add_spacing_for_first_sign` set to true.
    pub const ADD_SPACING_FOR_FIRST: Self = QuaternionFormat {
        add_spacing_for_first: true,
        remove_spacing: false,
        show_1s: false,
        explicit_real_axis: false,
        explicit_plus_sign: false,
        show_0s: false,
    };

    /// Removes all spacing inbetween numbers.
    /// Has only `remove_spacing_for_nonfirst_signs` set to true.
    pub const REMOVE_SPACING: Self = QuaternionFormat {
        add_spacing_for_first: false,
        remove_spacing: true,
        show_1s: false,
        explicit_real_axis: false,
        explicit_plus_sign: false,
        show_0s: false,
    };

    /// Shows 1s for units on the imaginary axies.
    /// Has only `show_1x_for_unit_values_of_x` set to true.
    pub const SHOW_1S: Self = QuaternionFormat {
        add_spacing_for_first: false,
        remove_spacing: false,
        show_1s: true,
        explicit_real_axis: false,
        explicit_plus_sign: false,
        show_0s: false,
    };

    /// Adds the `'r'` char to the end of the real part.
    /// Has only `add_r_ro_real_axis` set to true.
    pub const EXPLICIT_REAL_AXIS: Self = QuaternionFormat {
        add_spacing_for_first: false,
        remove_spacing: false,
        show_1s: false,
        explicit_real_axis: true,
        explicit_plus_sign: false,
        show_0s: false,
    };

    /// Adds the `'+'` char to the start of the first number when positive.
    /// Has only `add_plus_sign_for_first` set to true.
    pub const EXPLICIT_PLUS_SIGN: Self = QuaternionFormat {
        add_spacing_for_first: false,
        remove_spacing: false,
        show_1s: false,
        explicit_real_axis: false,
        explicit_plus_sign: true,
        show_0s: false,
    };

    /// Shows 0s for axieses instead of skipping them.
    /// Has only `show_0x_for_zero_values` set to true.
    pub const SHOW_0S: Self = QuaternionFormat {
        add_spacing_for_first: false,
        remove_spacing: false,
        show_1s: false,
        explicit_real_axis: false,
        explicit_plus_sign: false,
        show_0s: true,
    };

    #[inline]
    /// Compibes the two formats (like an or operation).
    pub const fn with(self, addon: Self) -> QuaternionFormat {
        QuaternionFormat {
            add_spacing_for_first:
                self.add_spacing_for_first
             || addon.add_spacing_for_first,

            remove_spacing:
                self.remove_spacing
             || addon.remove_spacing,

            show_1s:
                self.show_1s
             || addon.show_1s,

            explicit_real_axis:
                self.explicit_real_axis
             || addon.explicit_real_axis,
             
             explicit_plus_sign:
                 self.explicit_plus_sign
              || addon.explicit_plus_sign,
             
            show_0s:
                self.show_0s
             || addon.show_0s,
        }
    }

    #[inline]
    /// Sets to false all the flags that have true in `remove`.
    pub const fn without(self, remove: Self) -> QuaternionFormat {
        QuaternionFormat {
            add_spacing_for_first:
                self.add_spacing_for_first
             && !remove.add_spacing_for_first,

            remove_spacing:
                self.remove_spacing
             && !remove.remove_spacing,

            show_1s:
                self.show_1s
             && !remove.show_1s,

            explicit_real_axis:
                self.explicit_real_axis
             && !remove.explicit_real_axis,
             
            explicit_plus_sign:
                self.explicit_plus_sign
             && !remove.explicit_plus_sign,
             
            show_0s:
                self.show_0s
             && !remove.show_0s,
        }
    }
}

impl BitAnd for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn bitand(self, other: Self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first:
                self.add_spacing_for_first
             && other.add_spacing_for_first,

            remove_spacing:
                self.remove_spacing
             && other.remove_spacing,

            show_1s:
                self.show_1s
             && other.show_1s,

            explicit_real_axis:
                self.explicit_real_axis
             && other.explicit_real_axis,
             
            explicit_plus_sign:
                self.explicit_plus_sign
             && other.explicit_plus_sign,

            show_0s:
                self.show_0s
             && other.show_0s,
        }
    }
}

impl BitOr for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first:
                self.add_spacing_for_first
             || other.add_spacing_for_first,

            remove_spacing:
                self.remove_spacing
             || other.remove_spacing,

            show_1s:
                self.show_1s
             || other.show_1s,

            explicit_real_axis:
                self.explicit_real_axis
             || other.explicit_real_axis,
             
            explicit_plus_sign:
                self.explicit_plus_sign
             || other.explicit_plus_sign,

            show_0s:
                self.show_0s
             || other.show_0s,
        }
    }
}

impl BitXor for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn bitxor(self, other: Self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first:
                self.add_spacing_for_first
              ^ other.add_spacing_for_first,

            remove_spacing:
                self.remove_spacing
              ^ other.remove_spacing,

            show_1s:
                self.show_1s
              ^ other.show_1s,

            explicit_real_axis:
                self.explicit_real_axis
              ^ other.explicit_real_axis,
             
            explicit_plus_sign:
                self.explicit_plus_sign
              ^ other.explicit_plus_sign,

            show_0s:
                self.show_0s
              ^ other.show_0s,
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
        self.add_spacing_for_first &= other.add_spacing_for_first;
        self.remove_spacing &= other.remove_spacing;
        self.show_1s &= other.show_1s;
        self.explicit_real_axis &= other.explicit_real_axis;
        self.explicit_plus_sign &= other.explicit_plus_sign;
        self.show_0s &= other.show_0s;
    }
}

impl BitOrAssign for QuaternionFormat {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.add_spacing_for_first |= other.add_spacing_for_first;
        self.remove_spacing |= other.remove_spacing;
        self.show_1s |= other.show_1s;
        self.explicit_real_axis |= other.explicit_real_axis;
        self.explicit_plus_sign |= other.explicit_plus_sign;
        self.show_0s |= other.show_0s;
    }
}

impl BitXorAssign for QuaternionFormat {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.add_spacing_for_first ^= other.add_spacing_for_first;
        self.remove_spacing ^= other.remove_spacing;
        self.show_1s ^= other.show_1s;
        self.explicit_real_axis ^= other.explicit_real_axis;
        self.explicit_plus_sign ^= other.explicit_plus_sign;
        self.show_0s ^= other.show_0s;
    }
}

impl Not for QuaternionFormat {
    type Output = QuaternionFormat;

    #[inline]
    fn not(self) -> Self::Output {
        QuaternionFormat {
            add_spacing_for_first:        !self.add_spacing_for_first,
            remove_spacing: !self.remove_spacing,
            show_1s:      !self.show_1s,
            explicit_real_axis:                !self.explicit_real_axis,
            explicit_plus_sign:           !self.explicit_plus_sign,
            show_0s:           !self.show_0s,
        }
    }
}
