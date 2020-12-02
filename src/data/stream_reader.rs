use super::*;
use std::cmp;

pub struct StreamReader<'a> {
    data: &'a [EOByte],
    position: usize,
}

impl<'a> StreamReader<'a> {
    pub fn new(data: &'a [EOByte]) -> Self {
        Self { data, position: 0 }
    }
    pub fn length(&self) -> usize {
        self.data.len()
    }
    pub fn remaining(&self) -> usize {
        self.length() - self.position
    }
    pub fn eof(&self) -> bool {
        self.position >= self.data.len()
    }
    pub fn get_byte(&mut self) -> EOByte {
        let number = self.data[self.position];
        self.position += 1;
        number
    }
    pub fn get_char(&mut self) -> EOChar {
        let number = decode_number(&self.data[self.position..self.position + 1]);
        self.position += 1;
        number as EOChar
    }
    pub fn get_short(&mut self) -> EOShort {
        let number = decode_number(&self.data[self.position..self.position + 2]);
        self.position += 2;
        number as EOShort
    }
    pub fn get_three(&mut self) -> EOThree {
        let number = decode_number(&self.data[self.position..self.position + 3]);
        self.position += 3;
        number as EOThree
    }
    pub fn get_int(&mut self) -> EOInt {
        let number = decode_number(&self.data[self.position..self.position + 4]);
        self.position += 4;
        number as EOInt
    }
    pub fn get_fixed_string(&mut self, length: usize) -> String {
        if self.remaining() >= length {
            let bytes_to_read =
                &self.data[self.position..cmp::min(self.length(), self.position + length)];
            self.position += length;
            String::from_utf8(bytes_to_read.to_vec())
                .expect("Failed to convert byte array to string")
        } else {
            String::from("")
        }
    }
    pub fn get_break_string(&mut self) -> String {
        let position_of_break_char = {
            let mut position = 0;
            for i in self.position..self.data.len() {
                if self.data[i] == EO_BREAK_CHAR {
                    position = i;
                    break;
                }
            }
            position
        };

        let string = self.get_fixed_string(position_of_break_char - self.position);
        self.position += 1;
        string
    }
    pub fn get_prefix_string(&mut self) -> String {
        let length = self.get_char() as usize;
        let string = self.get_fixed_string(length);
        self.position += length;
        string
    }
    pub fn get_end_string(&mut self) -> String {
        self.get_fixed_string(self.remaining())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_byte() {
        let bytes = [0];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_byte(), bytes[0]);
    }
    #[test]
    fn get_char() {
        let bytes = [1];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_char(), 0);
    }
    #[test]
    fn get_short_one_byte() {
        let bytes = [2, 254];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_short(), 1);
    }
    #[test]
    fn get_short_two_bytes() {
        let bytes = [1, 2];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_short(), 253);
    }
    #[test]
    fn get_three_one_byte() {
        let bytes = [2, 254, 254];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_three(), 1);
    }
    #[test]
    fn get_three_two_bytes() {
        let bytes = [1, 2, 254];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_three(), 253);
    }
    #[test]
    fn get_three_three_bytes() {
        let bytes = [1, 1, 2];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_three(), 64009);
    }
    #[test]
    fn get_int_one_byte() {
        let bytes = [2, 254, 254, 254];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_int(), 1);
    }
    #[test]
    fn get_int_two_bytes() {
        let bytes = [1, 2, 254, 254];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_int(), 253);
    }
    #[test]
    fn get_int_three_bytes() {
        let bytes = [1, 1, 2, 254];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_int(), 64009);
    }
    #[test]
    fn get_int_four_bytes() {
        let bytes = [1, 1, 1, 2];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_int(), 16194277);
    }
    #[test]
    fn get_fixed_string() {
        let bytes = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_fixed_string(13), "Hello, world!");
    }
    #[test]
    fn get_break_string() {
        let bytes = [
            0x48,
            0x65,
            0x6C,
            0x6C,
            0x6F,
            0x2C,
            0x20,
            0x77,
            0x6F,
            0x72,
            0x6C,
            0x64,
            0x21,
            EO_BREAK_CHAR,
        ];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_break_string(), "Hello, world!");
    }
    #[test]
    fn get_prefix_string() {
        let bytes = [
            0xE, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ];
        let mut reader = StreamReader::new(&bytes);
        assert_eq!(reader.get_prefix_string(), "Hello, world!");
    }
}
