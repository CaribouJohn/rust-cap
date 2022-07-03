use crate::header::Header;
use std::fmt::Display;
use std::io::{Read, Seek};
use std::{
    fs::File,
    io::{self},
};

//fields are 32bit aligned
pub struct RawBlock {
    pub header: Header,
    data: Vec<u8>,
}

impl Display for RawBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Header {} Data {}",
            self.header,
            String::from_utf8_lossy(self.data.as_slice()).into_owned()
        )
        // self.data.iter().for_each(|c| {
        //     let part: Vec<u8> = escape_default(b).collect();
        //     visible.push_str(str::from_utf8(&part).unwrap());

        //     write!(f, "{}", c as char);
        // });
    }
}

impl RawBlock {
    pub fn new(h: Header, f: &mut File) -> io::Result<RawBlock> {
        let mut d: Vec<u8> = Vec::new();
        let mut cnt_to_end = f.stream_position()?;
        cnt_to_end -= h.start;
        cnt_to_end = h.len as u64 - cnt_to_end;
        f.take(cnt_to_end).read_to_end(&mut d)?;
        Ok(RawBlock { header: h, data: d })
    }
}
