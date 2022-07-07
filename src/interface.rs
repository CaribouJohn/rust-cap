use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::fmt::Display;
use std::io::{self, Cursor};

use crate::option::OptionValue;
use crate::rawblock::RawBlock;

#[derive(Debug, Default)]
pub struct InterfaceBlock {
    pub header: RawBlock,
    linktype: u16,
    reserved: u16,
    snaplen: u32,
    //options here
    options: Vec<OptionValue>,
}

impl TryFrom<RawBlock> for InterfaceBlock {
    type Error = io::Error;

    fn try_from(value: RawBlock) -> Result<Self, Self::Error> {
        let mut sb = Self::default();
        sb.header = value.clone(); //have to clone
        let mut bl_cursor = Cursor::new(value.data);
        if let Some(true) = sb.header.endianness {
            sb.linktype = bl_cursor.read_u16::<BigEndian>()?;
            sb.reserved = bl_cursor.read_u16::<BigEndian>()?;
            sb.snaplen = bl_cursor.read_u32::<BigEndian>()?;
            sb.extract_options::<BigEndian>(&mut bl_cursor);
        } else {
            sb.linktype = bl_cursor.read_u16::<LittleEndian>()?;
            sb.reserved = bl_cursor.read_u16::<LittleEndian>()?;
            sb.snaplen = bl_cursor.read_u32::<LittleEndian>()?;
            sb.extract_options::<LittleEndian>(&mut bl_cursor);
        }
        Ok(sb)
    }
}

impl InterfaceBlock {
    fn extract_options<T: ByteOrder>(&mut self, source: &mut Cursor<Vec<u8>>) {
        let mut latestoption = 1;
        //println!("before options : {}", self);
        while latestoption != 0 {
            let mut o = OptionValue::default();
            latestoption = o.read_option::<T>(source).unwrap();
            self.options.push(o);
        }
    }
}

impl Display for InterfaceBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#010x?} ({} for {} bytes) (v{}.{}) max packet size = {}\n",
            self.header.blktype,
            self.header.filepos,
            self.header.blklen,
            self.linktype,
            self.reserved,
            self.snaplen
        )?;

        for o in self.options.iter() {
            write!(f, "\t{}\n", o)?;
        }
        Ok(())
    }
}
