//! MC Protocol UUID data type.

use std::io::ErrorKind::InvalidInput;
use std::io::prelude::*;
use std::io;
use std::str::FromStr;

use packet::Protocol;

use uuid::Uuid;

/// UUID read/write wrapper.
impl Protocol for Uuid {
    type Clean = Uuid;

    fn proto_len(_: &Uuid) -> usize { 16 }
    fn proto_encode(value: &Uuid, dst: &mut dyn Write) -> io::Result<()> {
        dst.write_all(value.as_bytes())
    }
    /// Reads 16 bytes from `src` and returns a `Uuid`
    fn proto_decode(src: &mut dyn Read) -> io::Result<Uuid> {
        let mut v = [0u8; 16];
        src.read_exact(&mut v)?;
        Ok(Uuid::from_bytes(v))
    }
}

pub struct UuidString;

impl Protocol for UuidString {
    type Clean = Uuid;

    fn proto_len(value: &Uuid) -> usize {
        <String as Protocol>::proto_len(&value.hyphenated().encode_lower(&mut Uuid::encode_buffer()).to_string())
    }

    fn proto_encode(value: &Uuid, dst: &mut dyn Write) -> io::Result<()> {
        <String as Protocol>::proto_encode(&value.hyphenated().encode_lower(&mut Uuid::encode_buffer()).to_string(), dst)
    }

    fn proto_decode(src: &mut dyn Read) -> io::Result<Uuid> {
        // Unfortunately we can't implement `impl FromError<ParseError> for io::Error`
        let s = <String as Protocol>::proto_decode(src)?;
        Ok(Uuid::from_str(&s).unwrap())
    }
}
