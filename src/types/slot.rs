//! Minecraft item stack (inventory slot) data type

use std::io;
use std::io::prelude::*;

use nbt;

use packet::Protocol;

#[derive(Debug)]
pub struct Slot {
    id: u16,
    count: u8,
    damage: i16,
    tag: nbt::Blob
}

impl Protocol for Option<Slot> {
    type Clean = Option<Slot>;

    fn proto_len(value: &Option<Slot>) -> usize {
        match *value {
            Some(ref slot) => 2 + 1 + 2 + <nbt::Blob as Protocol>::proto_len(&slot.tag), // id, count, damage, tag
            None => 2
        }
    }

    fn proto_encode(value: &Option<Slot>, dst: &mut dyn Write) -> io::Result<()> {
        match *value {
            Some(Slot { id, count, damage, ref tag }) => {
                <i16 as Protocol>::proto_encode(&(id as i16), dst)?;
                <u8 as Protocol>::proto_encode(&count, dst)?;
                <i16 as Protocol>::proto_encode(&damage, dst)?;
                <nbt::Blob as Protocol>::proto_encode(tag, dst)?;
            }
            None => { <i16 as Protocol>::proto_encode(&-1, dst)? }
        }
        Ok(())
    }

    fn proto_decode(src: &mut dyn Read) -> io::Result<Option<Slot>> {
        let id = <i16 as Protocol>::proto_decode(src)?;
        Ok(if id == -1 {
            None
        } else {
            Some(Slot {
                id: id as u16,
                count: <u8 as Protocol>::proto_decode(src)?,
                damage: <i16 as Protocol>::proto_decode(src)?,
                tag: <nbt::Blob as Protocol>::proto_decode(src)?
            })
        })
    }
}
