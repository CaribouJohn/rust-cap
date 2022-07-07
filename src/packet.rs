use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use std::fmt::Display;
use std::io::{self, Cursor, Read};

use crate::option::OptionValue;
use crate::rawblock::RawBlock;

#[derive(Debug, Default)]
pub struct PacketBlock {
    pub header: RawBlock,
    interfaceid: u32,
    timestamp_high: u32,
    timestamp_low: u32,
    cap_packet_len: u32,
    orig_packet_len: u32,
    pub data: Vec<u8>, //may have padding after
    //options here
    options: Vec<OptionValue>,
}

impl TryFrom<RawBlock> for PacketBlock {
    type Error = io::Error;

    fn try_from(value: RawBlock) -> Result<Self, Self::Error> {
        let mut sb = Self::default();
        sb.header = value.clone(); //have to clone
        let bl_cursor = &mut Cursor::new(value.data);
        if let Some(true) = sb.header.endianness {
            sb.interfaceid = bl_cursor.read_u32::<BigEndian>()?;
            sb.timestamp_high = bl_cursor.read_u32::<BigEndian>()?;
            sb.timestamp_low = bl_cursor.read_u32::<BigEndian>()?;
            sb.cap_packet_len = bl_cursor.read_u32::<BigEndian>()?;
            sb.orig_packet_len = bl_cursor.read_u32::<BigEndian>()?;
            let padding = sb.cap_packet_len % 4;
            bl_cursor
                .take((sb.cap_packet_len + padding).into())
                .read_to_end(&mut sb.data)?;

            let mut buffer = Vec::new();
            // read the whole file
            bl_cursor.read_to_end(&mut buffer)?;
            if buffer.len() > 4 {
                let o_cursor = &mut Cursor::new(buffer);
                sb.extract_options::<BigEndian>(o_cursor);
            }
        } else {
            sb.interfaceid = bl_cursor.read_u32::<LittleEndian>()?;
            sb.timestamp_high = bl_cursor.read_u32::<LittleEndian>()?;
            sb.timestamp_low = bl_cursor.read_u32::<LittleEndian>()?;
            sb.cap_packet_len = bl_cursor.read_u32::<LittleEndian>()?;
            sb.orig_packet_len = bl_cursor.read_u32::<LittleEndian>()?;
            let padding = sb.cap_packet_len % 4;
            bl_cursor
                .take((sb.cap_packet_len + padding).into())
                .read_to_end(&mut sb.data)?;

            let mut buffer = Vec::new();
            // read the whole file
            bl_cursor.read_to_end(&mut buffer)?;
            if buffer.len() > 4 {
                let o_cursor = &mut Cursor::new(buffer);
                sb.extract_options::<LittleEndian>(o_cursor);
            }
        }
        Ok(sb)
    }
}

impl PacketBlock {
    fn extract_options<T: ByteOrder>(&mut self, source: &mut Cursor<Vec<u8>>) {
        let latestoption = 1;
        while latestoption != 0 {
            let mut o = OptionValue::default();
            if let Ok(_latestoption) = o.read_option::<T>(source) {
                self.options.push(o);
            }
        }
    }
}

impl Display for PacketBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#010x?} ({} for {} bytes) iface={} ({}.{}) packet size = {} (of {})\n{}\n",
            self.header.blktype,
            self.header.filepos,
            self.header.blklen,
            self.interfaceid,
            self.timestamp_high,
            self.timestamp_low,
            self.cap_packet_len,
            self.orig_packet_len,
            String::from_utf8_lossy(self.data.as_slice()).into_owned()
        )?;

        for o in self.options.iter() {
            write!(f, "\t{}\n", o)?;
        }
        Ok(())
    }
}
