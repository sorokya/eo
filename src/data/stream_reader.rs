use bytes::{Bytes, BytesMut, BufMut};

use super::{decode_number, EOByte, EOChar, EOInt, EOShort, EOThree, EO_BREAK_CHAR, decode_map_string};
use std::{cell::Cell, cmp};

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
/// use bytes::Bytes;
///
/// // EO Init Request
/// let buf: Vec<u8> = [
///     162, 190, 2, 1, 1, 29, 113, 8, 50, 57, 48, 49, 49, 51, 50,
/// ].to_vec();
/// let buf = Bytes::from(buf);
/// let reader = StreamReader::new(buf);
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
pub struct StreamReader {
    data: Bytes,
    position: Cell<usize>,
}

impl StreamReader {
    /// Creates a [StreamReader] for an existing `&[EOByte]`
    pub fn new(data: Bytes) -> Self {
        Self {
            data,
            position: Cell::new(0),
        }
    }
    pub fn has_break(&self) -> bool {
        self.data.iter().any(|b| *b == EO_BREAK_CHAR)
    }
    /// returns the length of the data stream
    pub fn length(&self) -> usize {
        self.data.len()
    }
    /// returns the amount of bytes left in the data stream
    pub fn remaining(&self) -> usize {
        self.length() - self.position.get()
    }
    /// returns a true if you have reached the end of the data stream
    ///
    /// # Example
    /// ```
    /// use eo::data::StreamReader;
    /// use bytes::Bytes;
    ///
    /// let buf: Vec<u8> = [
    ///     255, 255, 255, 255, 255, 255
    /// ].to_vec();
    /// let buf = Bytes::from(buf);
    /// let reader = StreamReader::new(buf);
    /// while !reader.eof() {
    ///     println!("{}", reader.get_byte());
    /// }
    /// ```
    pub fn eof(&self) -> bool {
        self.position.get() >= self.data.len()
    }
    /// returns a single [EOByte] from the data stream
    ///
    /// increases the read position by 1
    pub fn get_byte(&self) -> EOByte {
        let position = self.position.get();
        let number = self.data[position];
        self.position.set(position + 1);
        number
    }
    /// returns a single byte from the data stream decoded into an [EOChar]
    ///
    /// increases the read position by 1
    pub fn get_char(&self) -> EOChar {
        let position = self.position.get();
        let bytes_to_copy = cmp::min(self.data.len() - position, 1);
        let number = decode_number(&self.data[position..position + bytes_to_copy]);

        self.position.set(position + bytes_to_copy);
        number as EOChar
    }
    /// returns two bytes from the data stream decoded into an [EOShort]
    ///
    /// increases the read position by 2
    /// decodes two bytes using the [decode_number] method
    pub fn get_short(&self) -> EOShort {
        let position = self.position.get();
        let bytes_to_copy = cmp::min(self.data.len() - position, 2);
        let number = decode_number(&self.data[position..position + bytes_to_copy]);

        self.position.set(position + bytes_to_copy);
        number as EOShort
    }
    /// returns three bytes from the data stream decoded into an [EOThree]
    pub fn get_three(&self) -> EOThree {
        let position = self.position.get();
        let bytes_to_copy = cmp::min(self.data.len() - position, 3);
        let number = decode_number(&self.data[position..position + bytes_to_copy]);

        self.position.set(position + bytes_to_copy);
        number as EOThree
    }
    /// returns four bytes from the data stream decoded into an [EOInt]
    pub fn get_int(&self) -> EOInt {
        let position = self.position.get();
        let bytes_to_copy = cmp::min(self.data.len() - position, 4);
        let number = decode_number(&self.data[position..position + bytes_to_copy]);

        self.position.set(position + bytes_to_copy);
        number as EOInt
    }
    /// returns a UTF-8 encoded string of length `length` from the data stream
    pub fn get_fixed_string(&self, length: usize) -> String {
        if self.remaining() >= length {
            let bytes_to_read = self.get_vec(length);
            String::from_utf8_lossy(&bytes_to_read).to_string()
        } else {
            String::from("")
        }
    }
    /// returns a UTF-8 encoded string of length `length` from the data stream (decodes map strings)
    pub fn get_emf_string(&self, length: usize) -> String {
        let raw_string = self.get_vec(length);
        decode_map_string(raw_string)
    }
    /// returns a UTF-8 encoded string from the current read position to the next "break string" (0xff)
    pub fn get_break_string(&self) -> String {
        let position_of_break_char = {
            let mut temp_position = 0;
            for i in self.position.get()..self.data.len() {
                if self.data[i] == EO_BREAK_CHAR {
                    temp_position = i;
                    break;
                }
            }

            if temp_position == 0 {
                temp_position = self.data.len();
            }

            temp_position
        };

        let string = self.get_fixed_string(position_of_break_char - self.position.get());
        self.position.set(self.position.get() + 1);
        string
    }
    /// reads the next byte as a length and uses that length to get a fixed string in UTF-8 encoding
    pub fn get_prefix_string(&self) -> String {
        let length = self.get_char() as usize;
        self.get_fixed_string(length)
    }
    /// returns a UTF-8 encoded string from the current read position to the end of the data stream
    pub fn get_end_string(&self) -> String {
        self.get_fixed_string(self.remaining())
    }
    /// moves the read position forward by `length`
    pub fn seek(&self, length: usize) {
        self.position.set(self.position.get() + length);
    }
    pub fn reset(&self) {
        self.position.set(0);
    }
    /// returns a Vec<EOByte> of the desired length
    pub fn get_vec(&self, length: usize) -> Bytes {
        let position = self.position.get();
        let mut buf = BytesMut::with_capacity(length);
        buf.put_slice(&self.data[position..cmp::min(self.length(), position + length)]);
        self.position.set(position + length);
        buf.freeze()
    }

    /// returns a single [EOByte] from the data stream
    ///
    /// without increasing the read position by 1
    pub fn peek_byte(&self) -> EOByte {
        self.data[self.position.get()]
    }
}

#[cfg(test)]
mod tests {
    use super::{StreamReader, EO_BREAK_CHAR, BytesMut, BufMut};
    #[test]
    fn get_byte() {
        let mut bytes = BytesMut::with_capacity(1);
        bytes.put_u8(1);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_byte(), 1);
    }
    #[test]
    fn get_char() {
        let mut bytes = BytesMut::with_capacity(1);
        bytes.put_u8(1);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_char(), 0);
    }
    #[test]
    fn get_short_one_byte() {
        let mut bytes = BytesMut::with_capacity(2);
        bytes.put_u8(2);
        bytes.put_u8(254);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_short(), 1);
    }
    #[test]
    fn get_short_two_bytes() {
        let mut bytes = BytesMut::with_capacity(2);
        bytes.put_u8(1);
        bytes.put_u8(2);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_short(), 253);
    }
    #[test]
    fn get_three_one_byte() {
        let mut bytes = BytesMut::with_capacity(3);
        bytes.put_u8(2);
        bytes.put_u8(254);
        bytes.put_u8(254);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_three(), 1);
    }
    #[test]
    fn get_three_two_bytes() {
        let mut bytes = BytesMut::with_capacity(3);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(254);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_three(), 253);
    }
    #[test]
    fn get_three_three_bytes() {
        let mut bytes = BytesMut::with_capacity(3);
        bytes.put_u8(1);
        bytes.put_u8(1);
        bytes.put_u8(2);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_three(), 64009);
    }
    #[test]
    fn get_int_one_byte() {
        let mut bytes = BytesMut::with_capacity(4);
        bytes.put_u8(2);
        bytes.put_u8(254);
        bytes.put_u8(254);
        bytes.put_u8(254);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_int(), 1);
    }
    #[test]
    fn get_int_two_bytes() {
        let mut bytes = BytesMut::with_capacity(4);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(254);
        bytes.put_u8(254);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_int(), 253);
    }
    #[test]
    fn get_int_three_bytes() {
        let mut bytes = BytesMut::with_capacity(4);
        bytes.put_u8(1);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(254);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_int(), 64009);
    }
    #[test]
    fn get_int_four_bytes() {
        let mut bytes = BytesMut::with_capacity(4);
        bytes.put_u8(1);
        bytes.put_u8(1);
        bytes.put_u8(1);
        bytes.put_u8(2);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_int(), 16194277);
    }
    #[test]
    fn get_fixed_string() {
        let buf: Vec<u8> = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ].to_vec();

        let mut bytes = BytesMut::with_capacity(buf.len());
        bytes.put_slice(&buf);

        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_fixed_string(13), "Hello, world!");
    }
    #[test]
    fn get_break_string() {
        let buf: Vec<u8> = [
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
        ].to_vec();

        let mut bytes = BytesMut::with_capacity(buf.len());
        bytes.put_slice(&buf);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_break_string(), "Hello, world!");
    }
    #[test]
    fn get_prefix_string() {
        let buf: Vec<u8> = [
            0xE, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ].to_vec();

        let mut bytes = BytesMut::with_capacity(buf.len());
        bytes.put_slice(&buf);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.get_prefix_string(), "Hello, world!");
    }
    #[test]
    fn length() {
        let mut bytes = BytesMut::with_capacity(1);
        bytes.put_u8(EO_BREAK_CHAR);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.length(), 1);

        let mut bytes = BytesMut::with_capacity(3);
        bytes.put_u8(EO_BREAK_CHAR);
        bytes.put_u8(EO_BREAK_CHAR);
        bytes.put_u8(EO_BREAK_CHAR);

        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.length(), 3);
    }
    #[test]
    fn seek() {
        let mut bytes = BytesMut::with_capacity(2);
        bytes.put_u8(1);
        bytes.put_u8(2);
        let reader = StreamReader::new(bytes.freeze());
        assert_eq!(reader.position.get(), 0);
        reader.seek(2);
        assert_eq!(reader.position.get(), 2);
    }
    #[test]
    fn eof() {
        let mut bytes = BytesMut::with_capacity(1);
        bytes.put_u8(1);
        let reader = StreamReader::new(bytes.freeze());
        assert!(!reader.eof());
        reader.get_byte();
        assert!(reader.eof());
    }
    #[test]
    fn get_vec() {
        let mut bytes = BytesMut::with_capacity(5);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(4);
        bytes.put_u8(5);
        let reader = StreamReader::new(bytes.freeze());
        let buf = reader.get_vec(3);
        assert_eq!(buf[..], vec![1, 2, 3]);
        let buf = reader.get_vec(2);
        assert_eq!(buf[..], vec![4, 5]);
    }
}
