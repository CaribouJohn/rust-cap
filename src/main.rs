pub mod interface;
pub mod interfacestat;
pub mod option;
pub mod packet;
pub mod rawblock;
pub mod section;

use std::fs::File;
use std::io::{self};

use crate::interface::InterfaceBlock;
use crate::interfacestat::InterfaceStatBlock;
use crate::packet::PacketBlock;
use crate::rawblock::RawBlock;
use crate::section::SectionBlock;

fn main() -> io::Result<()> {
    //let mut f = File::open("nfs4.1.pcapng")?;
    let mut f = File::open("mountpluscallback.pcapng")?;

    //let mut stats = HashMap::new();
    //let mut list Vec<> = Vec::new();
    //Read header - should be a section
    let rb = RawBlock::from_file(&mut f, None)?;
    if let Ok(section) = SectionBlock::try_from(rb) {
        println!("----------------------------");
        println!("{}", section);
        println!("----------------------------");

        //loop {
        println!("----------------------------");
        loop {
            let current = RawBlock::from_file(&mut f, section.header.endianness);
            match current {
                Ok(rb) => match rb.blktype {
                    1 => match InterfaceBlock::try_from(rb) {
                        Ok(iface) => {
                            println!("{}", iface);
                        }
                        Err(e) => {
                            println!(r#"failed to convert to Interface block: {}"#, e);
                        }
                    },
                    5 => match InterfaceStatBlock::try_from(rb) {
                        Ok(iface) => {
                            println!("{}", iface);
                        }
                        Err(e) => {
                            println!(r#"failed to convert to Interface block: {}"#, e);
                        }
                    },
                    6 => match PacketBlock::try_from(rb) {
                        Ok(iface) => {
                            println!("{}", iface);
                        }
                        Err(e) => {
                            println!(r#"failed to convert to Interface block: {}"#, e);
                        }
                    },
                    _ => {
                        println!("{}", rb);
                    }
                },
                Err(e) => {
                    println!("{:#?}", e);
                    break;
                }
            }
            println!("----------------------------");
        }
    }
    //}

    Ok(())
}
