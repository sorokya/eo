use bytes::{BytesMut, BufMut, Bytes};

use super::{encode_number, EOByte, EOChar, EOInt, EOShort, EOThree, EO_BREAK_CHAR, encode_map_string};

/// used for building byte streams in EO format
///
/// [StreamBuilder] is generally used for writing data in the format
/// that EO uses in it's data (pub/data/map) files and server/client communications.
///
/// # Example
/// You might use a [StreamBuilder] in a client application to build the
/// Init request packet data like so
/// ```
/// use eo::data::StreamBuilder;
///
/// let hdid = String::from("2901132");
/// let mut builder = StreamBuilder::with_capacity(10 + hdid.len());
/// builder.add_three(42); // TODO: generate challenge
/// builder.add_char(0); // version major
/// builder.add_char(0); // version minor
/// builder.add_char(28); // version build
/// builder.add_char(112); // ?
/// builder.add_prefix_string(&hdid);
///
/// let buf = builder.get();
/// ```
///
pub struct StreamBuilder {
    data: BytesMut,
}

impl StreamBuilder {
    /// Creates a [StreamBuilder] with a default capacity
    ///
    /// default capacity of a [Vec] is 0, but changes to 4 when an item is
    /// pushed. Then it doubles every time you exceed that capacity.
    /// See [Capacity and reallocation](https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation)
    /// for more information
    pub fn new() -> Self {
        Self { data: BytesMut::new() }
    }
    /// Creates a [StreamBuilder] with a pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: BytesMut::with_capacity(capacity),
        }
    }
    /// Adds a single [EOByte] to the data stream
    pub fn add_byte(&mut self, number: EOByte) {
        self.data.put_u8(number);
    }
    /// Adds an [EOChar] encoded as a single byte to the data stream
    pub fn add_char(&mut self, number: EOChar) {
        let bytes = encode_number(number.into());
        self.data.put_slice(&bytes[0..1]);
    }
    /// Adds an [EOShort] encoded as two bytes to the data stream
    pub fn add_short(&mut self, number: EOShort) {
        let bytes = encode_number(number.into());
        self.data.put_slice(&bytes[0..2]);
    }
    /// Adds an [EOThree] encoded as three bytes to the data stream
    pub fn add_three(&mut self, number: EOThree) {
        let bytes = encode_number(number);
        self.data.put_slice(&bytes[0..3]);
    }
    /// Adds an [EOInt] encoded as four bytes to the data stream
    pub fn add_int(&mut self, number: EOInt) {
        let bytes = encode_number(number);
        self.data.put_slice(&bytes[0..4]);
    }
    /// Adds the UTF-8 encoded version of `string` to the data stream
    pub fn add_string(&mut self, string: &str) {
        self.data.put_slice(string.as_bytes());
    }
    /// Adds the UTF-8 encoded version of `string` + [EO_BREAK_CHAR] to
    /// the data stream
    pub fn add_break_string(&mut self, string: &str) {
        self.add_string(string);
        self.data.put_u8(EO_BREAK_CHAR);
    }
    /// Adds an [EOChar] of `string.len()` + the UTF-8 encoded version
    /// of `string` to the data stream
    pub fn add_prefix_string(&mut self, string: &str) {
        self.add_char(string.len() as EOChar);
        self.add_string(string);
    }
    /// Adds a fixed length UTF-8 encoded version of `string` to the data stream
    /// (padding with spaces if necessary)
    pub fn add_fixed_string(&mut self, string: &str, length: usize) {
        if string.len() > length {
            panic!("string is too long");
        }

        self.add_string(string);
        for _ in 0..length - string.len() {
            self.add_string(" ");
        }
    }
    pub fn add_emf_string(&mut self, string: &str, length: usize) {
        self.append(encode_map_string(string, length));
    }
    /// Appends data from other Vec to the end of this StreamBuilder
    pub fn append(&mut self, other: Bytes) {
        self.data.put(other);
    }
    /// Returns the data stream
    pub fn get(self) -> Bytes {
        self.data.freeze()
    }
}

impl Default for StreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BytesMut, BufMut};

    use super::{StreamBuilder, EO_BREAK_CHAR};
    #[test]
    fn add_byte() {
        let mut builder = StreamBuilder::with_capacity(1);
        builder.add_byte(1);
        assert_eq!(builder.data[0], 1);
    }
    #[test]
    fn add_char() {
        let mut builder = StreamBuilder::with_capacity(1);
        builder.add_char(1);
        assert_eq!(builder.data[0], 2);
    }
    #[test]
    fn add_short_one_byte() {
        let mut builder = StreamBuilder::with_capacity(2);
        builder.add_short(1);
        assert_eq!(builder.data[0], 2);
        assert_eq!(builder.data[1], 254);
    }
    #[test]
    fn add_short_two_bytes() {
        let mut builder = StreamBuilder::with_capacity(2);
        builder.add_short(253);
        assert_eq!(builder.data[0], 1);
        assert_eq!(builder.data[1], 2);
    }
    #[test]
    fn add_three_one_byte() {
        let mut builder = StreamBuilder::with_capacity(3);
        builder.add_three(1);
        assert_eq!(builder.data[0], 2);
        assert_eq!(builder.data[1], 254);
        assert_eq!(builder.data[2], 254);
    }
    #[test]
    fn add_three_two_bytes() {
        let mut builder = StreamBuilder::with_capacity(3);
        builder.add_three(253);
        assert_eq!(builder.data[0], 1);
        assert_eq!(builder.data[1], 2);
        assert_eq!(builder.data[2], 254);
    }
    #[test]
    fn add_three_three_bytes() {
        let mut builder = StreamBuilder::with_capacity(3);
        builder.add_three(64009);
        assert_eq!(builder.data[0], 1);
        assert_eq!(builder.data[1], 1);
        assert_eq!(builder.data[2], 2);
    }
    #[test]
    fn add_int_one_byte() {
        let mut builder = StreamBuilder::with_capacity(4);
        builder.add_int(1);
        assert_eq!(builder.data[0], 2);
        assert_eq!(builder.data[1], 254);
        assert_eq!(builder.data[2], 254);
        assert_eq!(builder.data[3], 254);
    }
    #[test]
    fn add_int_two_bytes() {
        let mut builder = StreamBuilder::with_capacity(4);
        builder.add_int(253);
        assert_eq!(builder.data[0], 1);
        assert_eq!(builder.data[1], 2);
        assert_eq!(builder.data[2], 254);
        assert_eq!(builder.data[3], 254);
    }
    #[test]
    fn add_int_three_bytes() {
        let mut builder = StreamBuilder::with_capacity(4);
        builder.add_int(64009);
        assert_eq!(builder.data[0], 1);
        assert_eq!(builder.data[1], 1);
        assert_eq!(builder.data[2], 2);
        assert_eq!(builder.data[3], 254);
    }
    #[test]
    fn add_int_four_bytes() {
        let mut builder = StreamBuilder::with_capacity(4);
        builder.add_int(16194277);
        assert_eq!(builder.data[0], 1);
        assert_eq!(builder.data[1], 1);
        assert_eq!(builder.data[2], 1);
        assert_eq!(builder.data[3], 2);
    }
    #[test]
    fn add_string() {
        let mut builder = StreamBuilder::new();
        builder.add_string("Hello, world!");
        assert_eq!(
            &builder.get()[..],
            [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21]
        );
    }
    #[test]
    fn add_break_string() {
        let mut builder = StreamBuilder::new();
        builder.add_break_string("Hello, world!");
        assert_eq!(
            &builder.get()[..],
            [
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
                EO_BREAK_CHAR
            ]
        );
    }
    #[test]
    fn add_prefix_string() {
        let mut builder = StreamBuilder::new();
        builder.add_prefix_string("Hello, world!");
        assert_eq!(
            &builder.get()[..],
            [0xE, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21]
        );
    }
    #[test]
    fn append() {
        let mut builder = StreamBuilder::new();
        builder.add_short(42);

        let mut vec = BytesMut::with_capacity(6);
        vec.put_slice(&[1, 2, 3, 4, 5, 6]);
        builder.append(vec.freeze());

        assert_eq!(&builder.get()[..], [43, 254, 1, 2, 3, 4, 5, 6]);
    }
}
