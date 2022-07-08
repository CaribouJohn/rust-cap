pub mod block;

use std::fs::File;
use std::io::{self};

use block::interface::InterfaceBlock;
use block::interfacestat::InterfaceStatBlock;
use block::packet::PacketBlock;
use block::rawblock::RawBlock;
use block::section::SectionBlock;

const _SECTION_BLOCK_TYPE: u32 = 0x0A0D0D0A;
const INTERFACE_BLOCK_TYPE: u32 = 0x1;
const INTERFACE_STATS_BLOCK_TYPE: u32 = 0x5;
const PACKET_BLOCK_TYPE: u32 = 0x6;

fn main() -> io::Result<()> {
    //let mut f = File::open("nfs4.1.pcapng")?;
    let mut f = File::open("nfs4.1.pcapng")?;

    //let mut stats = HashMap::new();
    //let mut list Vec<> = Vec::new();

    //
    // Read header - should be a section
    // to start assume 1 section, but can be 1+
    //
    let rb = RawBlock::from_file(&mut f, None)?;
    if let Ok(mut section) = SectionBlock::try_from(rb) {
        //
        // loop will read packets and block associated with the section.
        // the iface is for the section explicitly and the blocks contents
        // are interpreted with reference to the linktype on the iface block.
        //
        // println!("--------------Section--------------");
        // println!("{}", section);
        // println!("-------------------------------------");
        loop {
            let current = RawBlock::from_file(&mut f, section.header.endianness);
            println!("{:?}", &current);
            match current {
                Ok(rb) => match rb.blktype {
                    INTERFACE_BLOCK_TYPE => match InterfaceBlock::try_from(rb) {
                        Ok(iface) => {
                            println!("{}", iface);
                            section.iface = Some(iface)
                        }
                        Err(e) => {
                            println!(r#"failed to convert to Interface block: {}"#, e);
                            break;
                        }
                    },
                    INTERFACE_STATS_BLOCK_TYPE => match InterfaceStatBlock::try_from(rb) {
                        Ok(ifacestat) => {
                            println!("{}", ifacestat);
                        }
                        Err(e) => {
                            println!(r#"failed to convert to Interface stats block: {}"#, e);
                            break;
                        }
                    },
                    PACKET_BLOCK_TYPE => match PacketBlock::try_from(rb) {
                        Ok(packet) => {
                            println!("{}", packet);
                            section.packets.push(packet);
                        }
                        Err(e) => {
                            println!(r#"failed to convert to Packet block: {}"#, e);
                            break;
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
        }

        //final...
        println!("{}", section);
    }

    Ok(())
}
