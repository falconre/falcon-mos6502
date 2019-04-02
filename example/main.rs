extern crate falcon_mos6502;

fn main() {

    let bytes = [
        0xa0, 0x00,
        0xb1, 0x80,
        0xf0, 0x11,
        0xc9, 0x41,
        0x90, 0x06,
        0xc9, 0x5b,
        0xb0, 0x02,
        0x09, 0x20,
        0x91, 0x82,
        0xc8,
        0xd0, 0xed,
        0x38,
        0x60,
        0x91, 0x82,
        0x18,
        0x60
    ];


    let mut offset = 0;
    loop {
        let slice = match bytes.get(offset..bytes.len()) {
            Some(slice) => slice,
            None => { break; }
        };

        if let Some(instruction) = falcon_mos6502::decode(slice) {
            let opbytes =
                (0..instruction.size())
                    .into_iter()
                    .map(|i| format!("{:02x}", bytes[offset + i]))
                    .collect::<Vec<String>>()
                    .join("");

            println!("{:04x} {:<6} {}", offset, opbytes, instruction);
            offset += instruction.size();
        }
        else {
            break;
        }
    }
}