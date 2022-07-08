use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::fmt::Display;
use std::io::{self, Cursor};

use crate::interface::InterfaceBlock;
use crate::option::OptionValue;
use crate::packet::PacketBlock;
use crate::rawblock::RawBlock;

#[derive(Debug, Default)]
pub struct SectionBlock {
    pub header: RawBlock,
    minor: u16,
    major: u16,
    sectionlength: i64,
    //options here
    options: Vec<OptionValue>,
    pub iface: Option<InterfaceBlock>,
    pub packets: Vec<PacketBlock>,
}

impl TryFrom<RawBlock> for SectionBlock {
    type Error = io::Error;

    fn try_from(value: RawBlock) -> Result<Self, Self::Error> {
        let mut sb = Self::default();
        sb.header = value.clone(); //have to clone
        let mut bl_cursor = Cursor::new(value.data);
        if let Some(true) = sb.header.endianness {
            sb.major = bl_cursor.read_u16::<BigEndian>()?;
            sb.minor = bl_cursor.read_u16::<BigEndian>()?;
            sb.sectionlength = bl_cursor.read_i64::<BigEndian>()?;
            sb.extract_options::<BigEndian>(&mut bl_cursor);
        } else {
            sb.major = bl_cursor.read_u16::<LittleEndian>()?;
            sb.minor = bl_cursor.read_u16::<LittleEndian>()?;
            sb.sectionlength = bl_cursor.read_i64::<LittleEndian>()?;
            sb.extract_options::<LittleEndian>(&mut bl_cursor);
        }
        Ok(sb)
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

impl Display for SectionBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--------------Section--------------\n")?;
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

        if let Some(i) = &self.iface {
            write!(f, "{}\n", i)?;
        }

        for p in self.packets.iter() {
            write!(f, "{}\n", p)?;
        }
        write!(f, "--------------End Section--------------\n")?;

        Ok(())
    }
}
