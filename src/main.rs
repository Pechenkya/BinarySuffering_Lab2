#![allow(dead_code, non_snake_case, unused_imports, unused_variables)]
mod BitStream;
use std::{fs::File, io::Read};

use BitStream::bin_string_LSBF;


fn main() {
    println!("------------------WRITE TEST------------------\n");
    let mut BS1 = BitStream::BitStream::new("text.bin", false);
    let a1: [u8; 2] = [0xE1, 0x01];
    let a2: [u8; 2] = [0xEE, 0x00];
    println!("Writing (9 bits): {a1:02x?}, bin (LSB-F): {}", bin_string_LSBF(&a1));
    BS1.write_bit_sequence(&a1, 9).unwrap();
    println!("Writing (9 bits): {a2:02x?}, bin (LSB-F): {}", bin_string_LSBF(&a2));
    BS1.write_bit_sequence(&a2, 9).unwrap();
    BS1.flush().unwrap();
    println!("");
    let b1: [u8; 2] = [0xDB, 0xFF];
    let b2: [u8; 5] = [0xAE, 0x26, 0x42, 0x5F, 0x19];
    println!("Writing (12 bits): {b1:02x?}, bin (LSB-F): {}", bin_string_LSBF(&b1));
    BS1.write_bit_sequence(&b1, 12).unwrap();
    println!("Writing (39 bits): {b2:02x?}, bin (LSB-F): {}", bin_string_LSBF(&b2));
    BS1.write_bit_sequence(&b2, 39).unwrap();
    BS1.flush().unwrap();

    println!("\n\n");
    let mut file = File::open("text.bin").unwrap();
    let mut check_buff = Vec::new();
    file.read_to_end(&mut check_buff).unwrap();
    println!("File contents binary (LSB-F): {}", bin_string_LSBF(&check_buff));
    println!("\n\n");

    
    println!("-------------------READ TEST------------------\n");
    let mut BS2 = BitStream::BitStream::new("text.bin", true);
    
    let vec1 = BS2.read_bit_sequence(11).unwrap();
    let vec2 = BS2.read_bit_sequence(7).unwrap();
    println!("Vec1 (read 11 bits): {vec1:02x?}, bin (LSB-F): {}", bin_string_LSBF(&vec1));
    println!("Vec2 (read 7 bits): {vec2:02x?}, bin (LSB-F): {}", bin_string_LSBF(&vec2));
    let vec3 = BS2.read_bit_sequence(24).unwrap();
    println!("Vec3 (read 24 bits): {vec3:02x?}, bin (LSB-F): {}", bin_string_LSBF(&vec3));
    let vec4 = BS2.read_bit_sequence(33).unwrap();
    println!("Vec4 (read 33 bits): {vec4:02x?}, bin (LSB-F): {}", bin_string_LSBF(&vec4));
}
