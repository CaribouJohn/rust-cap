pub mod block;
pub mod header;

use std::fs::File;
use std::io;

use crate::block::RawBlock;
//use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("mountpluscallback.pcapng")?;

    //Read header - should be a section
    println!("----------------------------");
    let hdr = header::Header::new(&mut f)?;
    if hdr.is_section() {
        //Use the header to create the block
        let rb = RawBlock::new(hdr, &mut f)?;
        //this starts the tree of blocks
        println!("{}", rb);
    }
    println!("----------------------------");

    loop {
        let nxt = header::Header::new(&mut f)?;
        let rb1 = RawBlock::new(nxt, &mut f);
        match rb1 {
            Ok(raw) => println!("{}", raw),
            _ => break,
        }
    }

    Ok(())
}
