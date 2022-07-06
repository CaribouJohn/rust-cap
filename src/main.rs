pub mod option;
mod rawblock;
pub mod section;

use std::fs::File;
use std::io::{self};

use crate::rawblock::RawBlock;
use crate::section::SectionBlock;

fn main() -> io::Result<()> {
    //let mut f = File::open("nfs4.1.pcapng")?;
    let mut f = File::open("mountpluscallback.pcapng")?;

    //let mut stats = HashMap::new();
    //let mut list Vec<> = Vec::new();
    //Read header - should be a section
    let rb = RawBlock::from_file(&mut f, None)?;
    let section = SectionBlock::from(rb);
    println!("----------------------------");
    println!("{}", section);
    println!("----------------------------");

    //loop {
    println!("----------------------------");
    let current = RawBlock::from_file(&mut f, section.header.endianness);
    match current {
        Ok(rb) => {
            println!("{}", rb);
        }
        Err(e) => {
            println!("{:#?}", e);
            //break},
        }
    }
    println!("----------------------------");
    //}

    Ok(())
}
