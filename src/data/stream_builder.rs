use super::*;

pub struct StreamBuilder {
    data: Vec<EOByte>,
}

impl StreamBuilder {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    pub fn add_byte(&mut self, number: EOByte) {
        self.data.push(number);
    }
    pub fn add_char(&mut self, number: EOChar) {
        let bytes = encode_number(number.into());
        self.data.push(bytes[0]);
    }
    pub fn add_short(&mut self, number: EOShort) {
        let bytes = encode_number(number.into());
        self.data.push(bytes[0]);
        self.data.push(bytes[1]);
    }
    pub fn add_three(&mut self, number: EOThree) {
        let bytes = encode_number(number);
        self.data.push(bytes[0]);
        self.data.push(bytes[1]);
        self.data.push(bytes[2]);
    }
    pub fn add_int(&mut self, number: EOInt) {
        let bytes = encode_number(number);
        self.data.push(bytes[0]);
        self.data.push(bytes[1]);
        self.data.push(bytes[2]);
        self.data.push(bytes[3]);
    }
    pub fn add_string(&mut self, string: &str) {
        self.data.extend_from_slice(string.as_bytes());
    }
    pub fn add_break_string(&mut self, string: &str) {
        self.add_string(string);
        self.data.push(EO_BREAK_CHAR);
    }
    pub fn add_prefix_string(&mut self, string: &str) {
        self.add_char(string.len() as EOChar);
        self.add_string(string);
    }
    pub fn get(self) -> Vec<EOByte> {
        self.data
    }
}

impl Default for StreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            builder.data,
            [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21]
        );
    }
    #[test]
    fn add_break_string() {
        let mut builder = StreamBuilder::new();
        builder.add_break_string("Hello, world!");
        assert_eq!(
            builder.data,
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
            builder.data,
            [0xE, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21]
        );
    }
}
