use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::fmt::Display;
use std::io::Cursor;

use crate::option::OptionValue;
use crate::rawblock::RawBlock;

#[derive(Debug)]
pub struct SectionBlock {
    pub header: RawBlock,
    minor: u16,
    major: u16,
    sectionlength: i64,
    //options here
    options: Vec<OptionValue>,
}

impl From<RawBlock> for SectionBlock {
    fn from(rb: RawBlock) -> Self {
        let mut sb = Self::default();
        sb.header = rb.clone(); //have to clone 
        let mut bl_cursor = Cursor::new(rb.data);
        if let Some(true) = sb.header.endianness {
            sb.major = bl_cursor.read_u16::<BigEndian>().unwrap();
            sb.minor = bl_cursor.read_u16::<BigEndian>().unwrap();
            sb.sectionlength = bl_cursor.read_i64::<BigEndian>().unwrap();
            sb.extract_options::<BigEndian>(&mut bl_cursor);
        } else {
            sb.major = bl_cursor.read_u16::<LittleEndian>().unwrap();
            sb.minor = bl_cursor.read_u16::<LittleEndian>().unwrap();
            sb.sectionlength = bl_cursor.read_i64::<LittleEndian>().unwrap();
            sb.extract_options::<LittleEndian>(&mut bl_cursor);
        }
        sb
    }
}

impl SectionBlock {
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

impl Default for SectionBlock {
    fn default() -> Self {
        Self {
            header: RawBlock::default(),
            minor: Default::default(),
            major: Default::default(),
            sectionlength: Default::default(),
            options: Default::default(),
        }
    }
}

impl Display for SectionBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#010x?} ({} for {} bytes) (v{}.{}) optionlen = {}\n",
            self.header.blktype,
            self.header.filepos,
            self.header.blklen,
            self.major,
            self.minor,
            self.sectionlength
        )?;

        for o in self.options.iter() {
            write!(f, "\t{}\n", o)?;
        }
        Ok(())
    }
}
