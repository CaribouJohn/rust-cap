use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};
use std::{
    fs::File,
    io::{self},
};

#[derive(Debug, Clone)]
pub enum CapFileStatus {
    Valid(bool), //false = LittleEndian
    Invalid(String),
}

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

#[derive(Debug)]
pub struct SectionBlockHeader {
    status: CapFileStatus,
    blocktype: [u8; 4],
    rawblocklength: [u8; 4],
    byteordermagic: [u8; 4],
    blocklength: u32,
    minor: u16,
    major: u16,
    sectionlength: i64,
    //options here
    options: Vec<OptionValue>,
}

impl SectionBlockHeader {
    pub fn read_data<T: byteorder::ByteOrder>(
        &mut self,
        f: &mut File,
    ) -> io::Result<CapFileStatus> {
        let mut bl_cursor = Cursor::new(self.rawblocklength);
        self.blocklength = bl_cursor.read_u32::<T>()?;
        self.major = f.read_u16::<T>()?;
        self.minor = f.read_u16::<T>()?;
        self.sectionlength = f.read_i64::<T>()?;

        let mut latestoption = 1;
        while latestoption != 0 {
            let mut o = OptionValue::default();
            latestoption = o.read_option::<T>(f)?;
            self.options.push(o);
        }

        Ok(self.status.clone())
    }

    pub fn read_from_file(&mut self, f: &mut File) -> io::Result<CapFileStatus> {
        //Set up endian check
        let header = [0xA, 0xD, 0xD, 0xA];
        let magic = [0x1A, 0x2B, 0x3C, 0x4D];

        //read first 4 bytes as bytes
        f.read_exact(&mut self.blocktype)?;
        for (index, value) in self.blocktype.into_iter().enumerate() {
            if value != header[index] {
                self.status = CapFileStatus::Invalid("Bad Header".to_string());
            }
        }

        f.read_exact(&mut self.rawblocklength)?;
        f.read_exact(&mut self.byteordermagic)?;
        if self.byteordermagic[0] == magic[0]
            && self.byteordermagic[1] == magic[1]
            && self.byteordermagic[2] == magic[2]
            && self.byteordermagic[3] == magic[3]
        {
            self.status = CapFileStatus::Valid(false);
        } else if self.byteordermagic[3] == magic[0]
            && self.byteordermagic[1] == magic[2]
            && self.byteordermagic[2] == magic[1]
            && self.byteordermagic[3] == magic[0]
        {
            self.status = CapFileStatus::Valid(true);
        } else {
            self.status = CapFileStatus::Invalid("Byte order invalid".to_string());
        }

        //set up rawlength

        match self.status {
            CapFileStatus::Valid(false) => {
                println!("BigEndian");
                return self.read_data::<BigEndian>(f);
            }
            CapFileStatus::Valid(true) => {
                println!("LittleEndian");
                return self.read_data::<LittleEndian>(f);
            }
            _ => {
                println!("Invalid");
            }
        }

        return Ok(self.status.clone());
    }
}

impl Default for SectionBlockHeader {
    fn default() -> Self {
        Self {
            status: CapFileStatus::Invalid("Not read".to_string()),
            blocktype: Default::default(),
            rawblocklength: Default::default(),
            blocklength: Default::default(),
            byteordermagic: Default::default(),
            minor: Default::default(),
            major: Default::default(),
            sectionlength: Default::default(),
            options: Default::default(),
        }
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
