#![no_std]
//! This crate allows the formatting of integer types as superscripts or subscripts.
//! It consists of two traits, [FormatSuperscript] & [FormatSubscript] with which the integers can
//! be formatted.
//!
//! ```
//! use indexing_fmt::*;
//!
//! let index = 12;
//! let name = format!("Ship{}", index.to_superscript());
//!
//! assert_eq!(name, "Ship¹²");
//! ```
//! ```
//! use indexing_fmt::*;
//!
//! let index = 840;
//! let name = format!("Docking-Bay{}", index.to_subscript());
//!
//! assert_eq!(name, "Docking-Bay₈₄₀");
//! ```

use core::fmt::Write;

const ESCAPES_SUPERSCRIPTS: [char; 10] = [
    '\u{2070}', '\u{00B9}', '\u{00B2}', '\u{00B3}', '\u{2074}', '\u{2075}', '\u{2076}', '\u{2077}',
    '\u{2078}', '\u{2079}',
];

const ESCAPES_SUBSCRIPTS: [char; 10] = [
    '\u{2080}', '\u{2081}', '\u{2082}', '\u{2083}', '\u{2084}', '\u{2085}', '\u{2086}', '\u{2087}',
    '\u{2088}', '\u{2089}',
];

/// This type should probably not be used directly.
///
/// See the [crate] level documentation and [FormatSuperscript::to_superscript].
#[doc(hidden)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Superscript<T>(pub T);

/// Responsible for converting to superscripts¹²³.
///
/// See the [crate] level documentation.
pub trait FormatSuperscript
where
    Self: Sized,
{
    fn to_superscript(&self) -> Superscript<Self>;
}

macro_rules! impl_superscript(
    ($ty_unsigned:ty, $ty_signed:ty) => {
        impl core::fmt::Display for Superscript<$ty_unsigned> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if self.0 == 0 {
                    f.write_char(ESCAPES_SUPERSCRIPTS[0])?;
                } else {
                    let mut value = self.0;
                    let max_base = value.ilog10();
                    for base in (0..max_base + 1).rev() {
                        let b = (10 as $ty_unsigned).pow(base);
                        let digit = value / b;
                        f.write_char(ESCAPES_SUPERSCRIPTS[digit as usize])?;
                        value %= b;
                    }
                }
                Ok(())
            }
        }

        impl core::fmt::Display for Superscript<$ty_signed> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if self.0 < 0 {
                    f.write_char('\u{207b}')?;
                }
                let new_value = Superscript(self.0.unsigned_abs());
                <Superscript<$ty_unsigned> as core::fmt::Display>::fmt(&new_value, f)
            }
        }

        impl FormatSuperscript for $ty_signed {
            fn to_superscript(&self) -> Superscript<$ty_signed> {
                Superscript(*self)
            }
        }

        impl FormatSuperscript for $ty_unsigned {
            fn to_superscript(&self) -> Superscript<$ty_unsigned> {
                Superscript(*self)
            }
        }
    };
);

impl_superscript!(usize, isize);
impl_superscript!(u64, i64);
impl_superscript!(u32, i32);
impl_superscript!(u16, i16);
impl_superscript!(u8, i8);

/// This type should probably not be used directly.
///
/// See the [crate] level documentation and [FormatSubscript::to_subscript].
#[doc(hidden)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subscript<T>(pub T);

/// Responsible for converting to subscripts₁₂₃.
///
/// See the [crate] level documentation.
pub trait FormatSubscript
where
    Self: Sized,
{
    fn to_subscript(&self) -> Subscript<Self>;
}

macro_rules! impl_subscript(
    ($ty_unsigned:ty, $ty_signed:ty) => {
        impl core::fmt::Display for Subscript<$ty_unsigned> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // If zero, insert only one entry
                if self.0 == 0 {
                    f.write_char(ESCAPES_SUBSCRIPTS[0])?;
                } else {
                    let mut value = self.0;
                    let max_base = value.ilog10();
                    for base in (0..max_base + 1).rev() {
                        let b = (10 as $ty_unsigned).pow(base);
                        let digit = value / b;
                        f.write_char(ESCAPES_SUBSCRIPTS[digit as usize])?;
                        value %= b;
                    }
                }
                Ok(())
            }
        }

        impl core::fmt::Display for Subscript<$ty_signed> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if self.0 < 0 {
                    f.write_char('\u{208b}')?;
                }
                let new_value = Subscript(self.0.unsigned_abs());
                <Subscript<$ty_unsigned> as core::fmt::Display>::fmt(&new_value, f)
            }
        }

        impl FormatSubscript for $ty_unsigned {
            fn to_subscript(&self) -> Subscript<Self> {
                Subscript(*self)
            }
        }

        impl FormatSubscript for $ty_signed {
            fn to_subscript(&self) -> Subscript<$ty_signed> {
                Subscript(*self)
            }
        }
    };
);

impl_subscript!(usize, isize);
impl_subscript!(u64, i64);
impl_subscript!(u32, i32);
impl_subscript!(u16, i16);
impl_subscript!(u8, i8);

#[cfg(test)]
mod test {
    use super::*;
    extern crate std;

    #[test]
    fn superscript_single_digit() {
        let res = std::format!("value{}", 1.to_superscript());
        assert_eq!(res, "value¹");

        let res = std::format!("value{}", 2.to_superscript());
        assert_eq!(res, "value²");

        let res = std::format!("value{}", 3.to_superscript());
        assert_eq!(res, "value³");
    }

    #[test]
    fn superscript_negative() {
        let res = std::format!("U{}", (-1isize).to_superscript());
        assert_eq!(res, "U⁻¹");
    }

    #[test]
    fn superscript_multi_digit() {
        let res = std::format!("b{}", 87.to_superscript());
        assert_eq!(res, "b⁸⁷");

        let res = std::format!("b{}", 73_287.to_superscript());
        assert_eq!(res, "b⁷³²⁸⁷");

        let res = std::format!("b{}", 145_690.to_superscript());
        assert_eq!(res, "b¹⁴⁵⁶⁹⁰");
    }

    #[test]
    fn subscript_single_digit() {
        let res = std::format!("r{}", 0.to_subscript());
        assert_eq!(res, "r₀");

        let res = std::format!("r{}", 1.to_subscript());
        assert_eq!(res, "r₁");

        let res = std::format!("r{}", 2.to_subscript());
        assert_eq!(res, "r₂");
    }

    #[test]
    fn subscript_multi_digit() {
        let res = std::format!("gh{}", 23948.to_subscript());
        assert_eq!(res, "gh₂₃₉₄₈");

        let res = std::format!("gh{}", 15670.to_subscript());
        assert_eq!(res, "gh₁₅₆₇₀");
    }
}
