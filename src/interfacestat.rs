use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::fmt::Display;
use std::io::{self, Cursor};

use crate::option::OptionValue;
use crate::rawblock::RawBlock;

#[derive(Debug, Default)]
pub struct InterfaceStatBlock {
    pub header: RawBlock,
    interfaceid: u32,
    timestamp_high: u32,
    timestamp_low: u32,
    //options here
    options: Vec<OptionValue>,
}

impl TryFrom<RawBlock> for InterfaceStatBlock {
    type Error = io::Error;

    fn try_from(value: RawBlock) -> Result<Self, Self::Error> {
        let mut sb = Self::default();
        sb.header = value.clone(); //have to clone
        let mut bl_cursor = Cursor::new(value.data);
        if let Some(true) = sb.header.endianness {
            sb.interfaceid = bl_cursor.read_u32::<BigEndian>()?;
            sb.timestamp_high = bl_cursor.read_u32::<BigEndian>()?;
            sb.timestamp_low = bl_cursor.read_u32::<BigEndian>()?;
            sb.extract_options::<BigEndian>(&mut bl_cursor)?;
        } else {
            sb.interfaceid = bl_cursor.read_u32::<LittleEndian>()?;
            sb.timestamp_high = bl_cursor.read_u32::<LittleEndian>()?;
            sb.timestamp_low = bl_cursor.read_u32::<LittleEndian>()?;
            sb.extract_options::<LittleEndian>(&mut bl_cursor)?;
        }
        Ok(sb)
    }
}

impl InterfaceStatBlock {
    fn extract_options<T: ByteOrder>(&mut self, source: &mut Cursor<Vec<u8>>) -> io::Result<()> {
        let mut latestoption = 1;
        //println!("before options : {}", self);
        while latestoption != 0 {
            let mut o = OptionValue::default();
            latestoption = o.read_option::<T>(source)?;
            self.options.push(o);
        }
        Ok(())
    }
}

impl Display for InterfaceStatBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#010x?} ({} for {} bytes) iface={} ({}.{})\n",
            self.header.blktype,
            self.header.filepos,
            self.header.blklen,
            self.interfaceid,
            self.timestamp_high,
            self.timestamp_low
        )?;

        for o in self.options.iter() {
            write!(f, "\t{}\n", o)?;
        }
        Ok(())
    }
}
