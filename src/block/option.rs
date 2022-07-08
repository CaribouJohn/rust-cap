use byteorder::ReadBytesExt;
use std::fmt::Display;
use std::io::{self};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct OptionValue {
    pub optcode: u16,
    pub optlen: u16,
    pub optval: Vec<u8>,
}

impl Display for OptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#010x?} ({} bytes) {}",
            self.optcode,
            self.optlen,
            String::from_utf8_lossy(self.optval.as_slice()).into_owned()
        )
    }
}

impl Default for OptionValue {
    fn default() -> Self {
        Self {
            optcode: Default::default(),
            optlen: Default::default(),
            optval: Default::default(),
        }
    }
}

impl OptionValue {
    pub fn read_option<T: byteorder::ByteOrder>(
        &mut self,
        c: &mut Cursor<Vec<u8>>,
    ) -> io::Result<u16> {
        self.optcode = c.read_u16::<T>()?;
        self.optlen = c.read_u16::<T>()?;
        if self.optcode != 0 {
            c.take(self.optlen as u64).read_to_end(&mut self.optval)?;
            //            println!("{:#?}", str::from_utf8(&self.optval))
        }
        Ok(self.optcode)
    }
}
