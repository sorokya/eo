use crate::data::{EOByte, EOChar, EOThree, Serializeable, StreamBuilder, StreamReader};

const SIZE: usize = 6;

#[derive(Debug, Default)]
pub struct Request {
    pub challenge: EOThree,
    pub version: [EOChar; 3],
    pub hdid: String,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.challenge = reader.get_three();
        for i in 0..3 {
            self.version[i] = reader.get_char();
        }
        reader.get_char();
        reader.get_char();
        self.hdid = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SIZE + self.hdid.len());
        builder.add_three(self.challenge);
        for version_char in self.version.iter() {
            builder.add_char(*version_char);
        }
        builder.add_char(112); // always 112?
        builder.add_prefix_string(&self.hdid);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![
            222, 108, 4, 1, 1, 30, 113, 11, 52, 48, 55, 49, 54, 54, 54, 52, 48, 50,
        ];

        let mut request = Request::new();
        let reader = StreamReader::new(&data);
        request.deserialize(&reader);
        assert_eq!(request.challenge, 219319);
        assert_eq!(request.version, [0, 0, 29]);
        assert_eq!(request.hdid, "4071666402");
    }
    #[test]
    fn serialize() {
        let mut request = Request::new();
        request.challenge = 219319;
        request.version = [0, 0, 29];
        request.hdid = "4071666402".to_string();
        assert_eq!(
            request.serialize(),
            [222, 108, 4, 1, 1, 30, 113, 11, 52, 48, 55, 49, 54, 54, 54, 52, 48, 50,]
        );
    }
}
