#![allow(dead_code, non_snake_case, unused_imports, unused_variables)]
mod util;
mod BitStream;


fn main() {
    let mut BS1 = BitStream::BitStream::new("text.bin", false);
    let a1: [u8; 2] = [0xE1, 0x01];
    let a2: [u8; 2] = [0xEE, 0x00];
    BS1.write_bit_sequence(&a1, 9).unwrap();
    BS1.write_bit_sequence(&a2, 9).unwrap();
    BS1.flush().unwrap();
}
