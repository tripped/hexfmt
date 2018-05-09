//! # `hexfmt`
//!
//! Format-controlled hexadecimal output.

#![no_std]
use core::fmt::{Formatter, LowerHex, Result};

pub struct HexFmt<'a>(&'a [u8]);

impl<'a> LowerHex for HexFmt<'a> {
    fn fmt(&self, _fmtr: &mut Formatter) -> Result {
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
}
