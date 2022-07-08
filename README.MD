# rust-cap

##  Rust application to read and search PCAPNG files
The current code opens a `pcapng` file, parses the input once and then translates the bytes read into the fields of the Block Type.

Currently reads 
* SectionBlock `0x0A0D0D0A` (only the first currently)
* InterfaceBlock `0x1`
* (Enhanced) PacketBlock `0x6`
* InterfaceStats `0x5`  
* Options when specified in above block

## Todo
* Need to decompose the packet 
    * network layers (E.G ethernet frame, fields)
    * payload and potentially further decomposition

add filtering, command line parameters to provide a query mechanism. Should support

* by address source/target
* field/values
* arbitrary text
* payload type

I'd like to provide a way to specify the Block structure/payload structure In a general manner so this could be extended. I am not sure what mechanism I should use to specify this currently 

Other block types Not implemented - skipped if they occur in the file currently.

See https://www.ietf.org/staging/draft-tuexen-opsawg-pcapng-02.html 

I may extend the blocks to include the types below, but captures I have looked at so far only included the implemented types.

* 0x00000002	Packet Block (Appendix A)
* 0x00000003	Simple Packet Block (Section 4.4)
* 0x00000004	Name Resolution Block (Section 4.5)
* 0x00000009	systemd Journal Export Block 
* 0x0000000A	Decryption Secrets Block 


## Lisense and Use
The purpose of this code is to 

* learn Rust and write code to practice
* implement a well defined spec (pcap file format)
* create something that will aid me in my work. 

I don't use Rust at work but I am keen to learn the language as I believe it has some useful and modern features. 

I am not intending that this code is any more than an aid at the moment and so if you find it useful fill your boots...

This code will be released under the Apache 2.0 lisence if I ever finish it... 