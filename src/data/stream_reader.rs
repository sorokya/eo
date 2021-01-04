use super::{decode_number, EOByte, EOChar, EOInt, EOShort, EOThree, EO_BREAK_CHAR};
use std::cmp;

/// used for reading byte streams
///
/// This struct is for parsing numbers and string out of an EO
/// encoded data stream such as a data file or a data packet.
///
/// # Example
/// The following example shows a server application reading the
/// Init request data.
/// ```
/// use eo::data::StreamReader;
///
/// // EO Init Request
/// let buf = [
///     162, 190, 2, 1, 1, 29, 113, 8, 50, 57, 48, 49, 49, 51, 50,
/// ];
/// let mut reader = StreamReader::new(&buf);
/// let challenge = reader.get_three();
/// let version_major = reader.get_char();
/// let version_minor = reader.get_char();
/// let version_build = reader.get_char();
/// reader.seek(1); // unknown char
/// let hdid = reader.get_prefix_string();
///
/// println!(
///     "Client Request\nChallenge: {}\nVersion: {}.{}.{}\nHDID: {}",
///     challenge, version_major, version_minor, version_build, hdid
/// );
/// ```
pub struct StreamReader<'a> {
    data: &'a [EOByte],
    position: usize,
}

impl<'a> StreamReader<'a> {
    /// Creates a [StreamReader] for an existing `&[EOByte]`
    pub fn new(data: &'a [EOByte]) -> Self {
        Self { data, position: 0 }
    }
    /// returns the length of the data stream
    pub fn length(&self) -> usize {
        self.data.len()
    }
    /// returns the amount of bytes left in the data stream
    pub fn remaining(&self) -> usize {
        self.length() - self.position
    }
    /// returns a true if you have reached the end of the data stream
    ///
    /// # Example
    /// ```
    /// use eo::data::StreamReader;
    ///
    /// let buf = [
    ///     255, 255, 255, 255, 255, 255
    /// ];
    /// let mut reader = StreamReader::new(&buf);
    /// while !reader.eof() {
    ///     println!("{}", reader.get_byte());
    /// }
    /// ```
    pub fn eof(&self) -> bool {
        self.position >= self.data.len()
    }
    /// returns a single [EOByte] from the data stream
    ///
    /// increases the read position by 1
    pub fn get_byte(&mut self) -> EOByte {
        let number = self.data[self.position];
        self.position += 1;
        number
    }
    /// returns a single byte from the data stream decoded into an [EOChar]
    ///
    /// increases the read position by 1
    pub fn get_char(&mut self) -> EOChar {
        let number = decode_number(&self.data[self.position..self.position + 1]);
        self.position += 1;
        number as EOChar
    }
    /// returns two bytes from the data stream decoded into an [EOShort]
    ///
    /// increases the read position by 2
    /// decodes two bytes using the [decode_number] method
    pub fn get_short(&mut self) -> EOShort {
        let number = decode_number(&self.data[self.position..self.position + 2]);
        self.position += 2;
        number as EOShort
    }
    /// returns three bytes from the data stream decoded into an [EOThree]
    pub fn get_three(&mut self) -> EOThree {
        let number = decode_number(&self.data[self.position..self.position + 3]);
        self.position += 3;
        number as EOThree
    }
    /// returns four bytes from the data stream decoded into an [EOInt]
    pub fn get_int(&mut self) -> EOInt {
        let number = decode_number(&self.data[self.position..self.position + 4]);
        self.position += 4;
        number as EOInt
    }
    /// returns a UTF-8 encoded string of length `length` from the data stream
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
    /// returns a UTF-8 encoded string from the current read position to the next "break string" (0xff)
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
    /// reads the next byte as a length and uses that length to get a fixed string in UTF-8 encoding
    pub fn get_prefix_string(&mut self) -> String {
        let length = self.get_char() as usize;
        self.get_fixed_string(length)
    }
    /// returns a UTF-8 encoded string from the current read position to the end of the data stream
    pub fn get_end_string(&mut self) -> String {
        self.get_fixed_string(self.remaining())
    }
    /// moves the read position forward by `length`
    pub fn seek(&mut self, length: usize) {
        self.position += length;
    }
}

#[cfg(test)]
mod tests {
    use super::{StreamReader, EO_BREAK_CHAR};
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
    #[test]
    fn length() {
        let reader = StreamReader::new(&[255]);
        assert_eq!(reader.length(), 1);

        let reader = StreamReader::new(&[255, 255, 255]);
        assert_eq!(reader.length(), 3);
    }
}
