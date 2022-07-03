pub mod block;
pub mod header;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

use crate::block::RawBlock;

fn main() -> io::Result<()> {
    let mut f = File::open("nfs4.1.pcapng")?;

    let mut stats = HashMap::new();

    //Read header - should be a section
    println!("----------------------------");
    let hdr = header::Header::new(&mut f)?;
    if hdr.is_section() {
        //Use the header to create the block
        let rb = RawBlock::new(hdr, &mut f)?;
        //this starts the tree of blocks
        println!("{}", rb);
        let cnt = stats.entry(rb.header.blktype).or_insert(0u16);
        *cnt += 1u16;
    }

    //let mut current = f.stream_position();
    loop {
        println!("----------------------------");
        let nxtres = header::Header::new(&mut f);
        match nxtres {
            Ok(nxt) => {
                let rb1 = RawBlock::new(nxt, &mut f);
                match rb1 {
                    Ok(raw) => {
                        println!("{}", raw);
                        let cnt = stats.entry(raw.header.blktype).or_insert(0u16);
                        *cnt += 1;
                        //current = f.stream_position();
                    }
                    _ => {
                        stats.iter().for_each(|entry| {
                            println!("{:?}", entry);
                        });
                        break;
                    }
                }
            }
            _ => {
                stats.iter().for_each(|entry| {
                    println!("{:#010X} : {}", entry.0, entry.1);
                });
                break;
            }
        }
    }

    Ok(())
}
