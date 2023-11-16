//!
#![warn(missing_docs)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

/// Errors generated from the implementations
pub mod error;
pub use error::Error;

/// EncodeInto trait
pub mod enc_into;

/// TryDecodeFrom trait
pub mod try_decode_from;

/// one-stop shop for all exported symbols
pub mod prelude {
    pub use super::{enc_into::*, try_decode_from::*};
}

#[cfg(test)]
mod test {
    use super::prelude::*;

    #[test]
    fn test_u8() {
        let buf = 0xff_u8.encode_into();
        let (num, _) = u8::try_decode_from(&buf).unwrap();
        assert_eq!(0xff_u8, num);
    }

    #[test]
    fn test_u16() {
        let buf = 0xffee_u16.encode_into();
        let (num, _) = u16::try_decode_from(&buf).unwrap();
        assert_eq!(0xffee_u16, num);
    }

    #[test]
    fn test_u32() {
        let buf = 0xffeeddcc_u32.encode_into();
        let (num, _) = u32::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_u32, num);
    }

    #[test]
    fn test_u64() {
        let buf = 0xffeeddcc_bbaa9988_u64.encode_into();
        let (num, _) = u64::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_u64, num);
    }

    #[test]
    fn test_u128() {
        let buf = 0xffeeddcc_bbaa9988_77665544_33221100_u128.encode_into();
        let (num, _) = u128::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_77665544_33221100_u128, num);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_usize() {
        let buf = 0xffeeddcc_bbaa9988_usize.encode_into();
        let (num, _) = usize::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_bbaa9988_usize, num);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_usize() {
        let buf = 0xffeeddcc_usize.encode_into();
        let (num, _) = usize::try_decode_from(&buf).unwrap();
        assert_eq!(0xffeeddcc_usize, num);
    }
}
