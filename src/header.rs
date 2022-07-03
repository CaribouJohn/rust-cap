//
// We need to determine the actual block type from the first
// 4 bytes of the data being read.
//

use std::{
    fmt::{self, Display},
    fs::File,
    io::{self, Cursor, Read, Seek},
};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Header {
    pub start: u64,
    pub blktype: u32,
    pub len: u32,
    section: bool,
    little_endian: bool,
    magicbytes: [u8; 4],
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_section() {
            write!(
                f,
                "Pos = {:x} type = 0x{:x} , len = {:x}, section = {}, little_endian = {} , magic = ({:x},{:x},{:x},{:x})",
                self.start,
                self.blktype,
                self.len,
                self.section,
                self.little_endian,
                self.magicbytes[0],
                self.magicbytes[1],
                self.magicbytes[2],
                self.magicbytes[3],
            )
        } else {
            write!(
                f,
                "Pos = {:x} type = 0x{:x} , len = {}, section = {}, little_endian = {} ",
                self.start, self.blktype, self.len, self.section, self.little_endian,
            )
        }
    }
}

impl Header {
    //We need to read in the header from the source
    pub fn new(f: &mut File) -> io::Result<Header> {
        //Set up endian check
        let header = [0xA, 0xD, 0xD, 0xA];
        let magic = [0x1A, 0x2B, 0x3C, 0x4D];
        let mut sec = true;
        let mut le = true;

        let blkstart = f.stream_position()?;
        let mut h = [0u8; 4];
        let mut l = [0u8; 4];
        let mut m = [0u8; 4];
        f.read_exact(&mut h)?;
        f.read_exact(&mut l)?;

        //Check for section
        for (index, value) in h.into_iter().enumerate() {
            if value != header[index] {
                sec = false;
            }
        }

        if sec {
            f.read_exact(&mut m)?;
            if m[0] == magic[0] && m[1] == magic[1] && m[2] == magic[2] && m[3] == magic[3] {
                le = false;
            }
        }

        let mut h_cursor = Cursor::new(h);
        let hval: u32;
        let mut l_cursor = Cursor::new(l);
        let lval: u32;
        if le {
            hval = h_cursor.read_u32::<LittleEndian>()?;
            lval = l_cursor.read_u32::<LittleEndian>()?;
        } else {
            hval = h_cursor.read_u32::<BigEndian>()?;
            lval = l_cursor.read_u32::<BigEndian>()?;
        }

        Ok(Header {
            start: blkstart,
            blktype: hval,
            len: lval,
            section: sec,
            little_endian: le,
            magicbytes: m,
        })
    }

    pub fn is_section(&self) -> bool {
        self.section
    }
}
