use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use std::fmt::Display;
use std::io::{Cursor, Read, Seek};
use std::{
    fs::File,
    io::{self},
};

#[derive(Debug, Default, Clone)]
//Fields are a concept for other structs
pub struct RawBlock {
    //raw data of a block.
    //other types will use the buffer here
    pub filepos: u64,
    pub blktype: u32,
    pub blklen: u32,
    pub endianness: Option<bool>,
    pub data: Vec<u8>,
}

impl Display for RawBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#010x?} ({} for {} bytes) {}",
            self.blktype,
            self.filepos,
            self.blklen,
            String::from_utf8_lossy(self.data.as_slice()).into_owned()
        )
    }
}

impl RawBlock {
    //This will be done only for the first Section block to dtermine the
    //Endianness of the data in that section. All further blocks will depend
    //on the first.
    fn read_raw_section(&mut self, file: &mut File) -> io::Result<Option<bool>> {
        //Set up endian check
        let header = [0xA, 0xD, 0xD, 0xA];
        let magic = [0x1A, 0x2B, 0x3C, 0x4D];

        self.filepos = file.stream_position()?;
        let mut h = [0u8; 4];
        let mut l = [0u8; 4];
        let mut m = [0u8; 4];
        file.read_exact(&mut h)?;
        file.read_exact(&mut l)?;

        //Check for section
        for (index, value) in h.into_iter().enumerate() {
            if value != header[index] {
                return Ok(None); //fail
            }
        }

        //magic
        file.read_exact(&mut m)?;
        if m[0] == magic[0] && m[1] == magic[1] && m[2] == magic[2] && m[3] == magic[3] {
            self.endianness = Some(true);
        } else {
            self.endianness = Some(false);
        }

        let mut h_cursor = Cursor::new(h);
        let mut l_cursor = Cursor::new(l);
        match self.endianness {
            Some(false) => {
                self.blktype = h_cursor.read_u32::<LittleEndian>()?;
                self.blklen = l_cursor.read_u32::<LittleEndian>()?;
            }
            Some(true) => {
                self.blktype = h_cursor.read_u32::<BigEndian>()?;
                self.blklen = l_cursor.read_u32::<BigEndian>()?;
            }
            _ => {
                println!("NONE?: {:#?}", self);
                return Ok(None); //fail
            }
        }
        //println!("Self: {:#?}", self);
        return Ok(self.endianness);
    }

    fn read_raw_block<T: ByteOrder>(&mut self, file: &mut File) -> io::Result<Option<bool>> {
        self.filepos = file.stream_position()?;
        self.blktype = file.read_u32::<T>()?;
        self.blklen = file.read_u32::<T>()?;
        return Ok(self.endianness);
    }

    // public interface
    pub fn from_file(file: &mut File, endianness: Option<bool>) -> io::Result<RawBlock> {
        let mut rb = RawBlock::default();

        //Is this the first block? Endianness == None
        match endianness {
            Some(true) => {
                rb.read_raw_block::<BigEndian>(file)?;
            }
            Some(false) => {
                rb.read_raw_block::<LittleEndian>(file)?;
            }
            None => {
                rb.read_raw_section(file)?;
            }
        }

        //now we have length
        let mut cnt_to_end = file.stream_position()?; //current read position
        cnt_to_end -= rb.filepos; //sub position of start of block to get how many bytes we read already
        cnt_to_end = rb.blklen as u64 - cnt_to_end; //now get how many left
        file.take(cnt_to_end).read_to_end(&mut rb.data)?;

        Ok(rb)
    }
}
