pub mod capfile;

use std::fs::File;
use std::io;
//use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("mountpluscallback.pcapng")?;
    let mut sbh1: capfile::SectionBlockHeader = capfile::SectionBlockHeader::default();
    sbh1.read_from_file(&mut f)?;
    //let mut sbh2: capfile::SectionBlockHeader = ;

    // read up to 10 bytes
    //let n = f.read(&mut buffer)?;
    println!("{:?}", sbh1);
    Ok(())
}
