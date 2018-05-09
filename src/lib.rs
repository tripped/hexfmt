//! # `hexfmt`
//!
//! Format-controlled hexadecimal output.

#![no_std]
use core::fmt::{Formatter, LowerHex, UpperHex, Result};

pub struct HexFmt<'a>(&'a [u8]);

impl<'a> HexFmt<'a> {
    fn fmt(&self, fmtr: &mut Formatter, uppercase: bool) -> Result {
        let octets_per_row = fmtr.width().unwrap_or(0);
        let octets_per_group = fmtr.precision().unwrap_or(0);
        let mut in_row = 0;
        let mut in_group = 0;

        for (i, octet) in self.0.iter().enumerate() {
            if uppercase {
                write!(fmtr, "{:02X}", octet)?;
            } else {
                write!(fmtr, "{:02x}", octet)?;
            }

            if i+1 < self.0.len() {
                in_row += 1;
                in_group += 1;
                if 0 < octets_per_row && in_row == octets_per_row {
                    write!(fmtr, "\n")?;
                    in_row = 0;
                    in_group = 0;
                }
                if 0 < octets_per_group && in_group == octets_per_group {
                    write!(fmtr, " ")?;
                    in_group = 0;
                }
            }
        }
        Ok(())
    }
}

impl<'a> LowerHex for HexFmt<'a> {
    fn fmt(&self, fmtr: &mut Formatter) -> Result {
        self.fmt(fmtr, false)
    }
}

impl<'a> UpperHex for HexFmt<'a> {
    fn fmt(&self, fmtr: &mut Formatter) -> Result {
        self.fmt(fmtr, true)
    }
}

impl<'a, T> From<&'a T> for HexFmt<'a>
        where T: ?Sized + AsRef<[u8]> + 'a {
    fn from(t: &'a T) -> Self { HexFmt(t.as_ref()) }
}

pub fn hex<'a, T>(t: &'a T) -> HexFmt<'a>
        where T: ?Sized + AsRef<[u8]> + 'a {
    HexFmt::from(t)
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
    use ::hex;

    #[test]
    fn empty_slice_produces_empty_string() {
        assert_eq!("", format!("{:x}", hex(&[])));
    }

    #[test]
    fn single_octet() {
        assert_eq!("01", format!("{:x}", hex(&[0x01])));
    }

    #[test]
    fn several_octets() {
        assert_eq!("01020f", format!("{:x}", hex(&[0x01, 0x02, 0x0f])));
    }

    #[test]
    fn uppercase_octet() {
        assert_eq!("FF", format!("{:X}", hex(&[0xff])));
    }

    #[test]
    fn breaks_between_lines_1() {
        assert_eq!("AA\nBB", format!("{:1X}", hex(&[0xaa, 0xbb])));
    }

    #[test]
    fn four_octets_rowlimit_2_makes_2_lines() {
        assert_eq!(
            "AABB\nCCDD",
            format!("{:2X}", hex(&[0xaa, 0xbb, 0xcc, 0xdd])));
    }

    #[test]
    fn five_octets_rowlimit_2_makes_3_lines() {
        assert_eq!(
            "AABB\nCCDD\nEE",
            format!("{:2X}", hex(&[0xaa, 0xbb, 0xcc, 0xdd, 0xee])));
    }

    #[test]
    fn two_octets_grouplimit_1_makes_2_groups() {
        assert_eq!("11 22", format!("{:.1x}", hex(&[0x11, 0x22])));
    }

    #[test]
    fn five_octets_grouplimit_2_makes_3_groups() {
        assert_eq!(
            "1122 3344 55",
            format!("{:.2x}", hex(&[0x11, 0x22, 0x33, 0x44, 0x55])));
    }

    #[test]
    fn groups_and_rows_play_nice() {
        assert_eq!(
            "1122 3344 55\na0b0 c0",
             format!("{:5.2x}",
                     hex(&[0x11, 0x22, 0x33, 0x44, 0x55, 0xa0, 0xb0, 0xc0])));
    }
}
