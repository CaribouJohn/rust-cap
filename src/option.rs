use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};
use std::{
    fs::File,
    io::{self},
};

#[derive(Debug)]
pub struct OptionValue {
    optcode: u16,
    optlen: u16,
    optval: Vec<u8>,
}

impl OptionValue {
    pub fn read_option<T: byteorder::ByteOrder>(&mut self, f: &mut File) -> io::Result<u16> {
        self.optcode = f.read_u16::<T>()?;
        self.optlen = f.read_u16::<T>()?;
        if self.optcode != 0 {
            f.take(self.optlen as u64).read_to_end(&mut self.optval)?;
            //            println!("{:#?}", str::from_utf8(&self.optval))
        }
        Ok(self.optcode)
    }
}
