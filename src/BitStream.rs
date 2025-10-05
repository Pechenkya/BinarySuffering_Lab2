use std::path::Path;
use std::fs::File;
use crate::util::*;

pub struct BitStream {
    file: String,
    buff: Vec<u8>,
    bit_pointer: usize,
    read_dir: bool,
}

impl BitStream {
    pub fn new(file_path: &str, read_dir: bool) -> Self {
        let buff: Vec<u8>;
        if read_dir {
            buff = read_file_binary(file_path).unwrap();
        }
        else {
            buff = Vec::new();
        }

        BitStream {
            file: file_path.to_string(),
            buff: buff,
            bit_pointer: 0,
            read_dir: read_dir,
        }
    }

    pub fn clear_output_file(&self) -> Result<(), std::io::Error> {
        if !self.read_dir {
            let _file = File::create(&self.file).expect(&format!("Unable to clear file {}", self.file));
            Ok(())
        }
        else {
            create_error("Cannot clear file in read mode")
        }
    }

    pub fn write_bit_sequence(&mut self, in_buff: &[u8], bit_len: usize) -> Result<(), std::io::Error> {
        if self.read_dir {
            return create_error("This BitStream is in read mode");
        }

        let basic_shift = self.bit_pointer % 8;
        let full_bytes_to_write = bit_len / 8;
        let remaining_bits = bit_len % 8;
        
        if basic_shift == 0 {
            // Move full bytes to the stream buffer
            for i in 0..full_bytes_to_write {
                self.buff.push(in_buff[i]);
            }
            
            // Handle remaining bits
            if remaining_bits != 0 {
                self.buff.push((in_buff[full_bytes_to_write] << (8 - remaining_bits)) >> (8 - remaining_bits));
            }
        }
        else {
            let mut last_byte_id = self.buff.len() - 1;

            // Move full bytes to the stream buffer
            for i in 0..full_bytes_to_write {
                // Append low bits to the last byte
                self.buff[last_byte_id] |= in_buff[i] << basic_shift;

                // Push high bits as a new byte
                self.buff.push(in_buff[i] >> (8 - basic_shift));
                last_byte_id += 1;
            }

            // Handle remaining bits
            if remaining_bits != 0 {
                if remaining_bits + basic_shift > 8 {
                    self.buff[last_byte_id] |= in_buff[in_buff.len() - 1] << basic_shift;
                    self.buff.push(in_buff[in_buff.len() - 1] >> (remaining_bits - basic_shift));
                }
                else {
                    self.buff[last_byte_id] |= (in_buff[in_buff.len() - 1] << (8 - remaining_bits)) >> (8 - remaining_bits - basic_shift);
                }
            }
        }
        self.bit_pointer += bit_len;

        print!("Buffer after write: ");
        for b in self.buff.iter() {
            print!("{:08b}", b)
        }
        println!("");

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        if self.read_dir {
            return create_error("BitStream cannot be flushed in read mode");
        }

        print!("Buffer on flush: ");
        for b in self.buff.iter() {
            print!("{:08b} ", b)
        }
        println!("");

        let _res = write_file_binary(&self.file, &self.buff);
        self.buff.clear();
        self.bit_pointer = 0;

        Ok(())
    }

    pub fn read_bit_sequence(&mut self) -> Result<Vec<u8>, std::io::Error> {
        if !self.read_dir {
            return Err(create_error("This BitStream is in write mode").err().unwrap());
        }

        let result: Vec<u8> = Vec::new();

        // TODO

        Ok(result)
    }


}