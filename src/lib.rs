//! # `hexfmt`
//!
//! Format-controlled hexadecimal output.

#![no_std]
use core::fmt::{Formatter, LowerHex, Result};

pub struct HexFmt<'a>(&'a [u8]);

impl<'a> LowerHex for HexFmt<'a> {
    fn fmt(&self, fmtr: &mut Formatter) -> Result {
        for octet in self.0.iter() {
            write!(fmtr, "{:02x}", octet)?;
        }
        Ok(())
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
}
