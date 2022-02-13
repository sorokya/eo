use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default)]
pub struct Request {
    pub name: String,
    pub password: String,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.password = reader.get_break_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.name.len());
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.password);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![97, 100, 109, 105, 110, 255, 116, 101, 115, 116, 255];
        let mut packet = Request::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.name, "admin");
        assert_eq!(packet.password, "test");
    }
    #[test]
    fn serialize() {
        let mut packet = Request::new();
        packet.name = "admin".to_string();
        packet.password = "test".to_string();
        assert_eq!(
            packet.serialize(),
            [97, 100, 109, 105, 110, 255, 116, 101, 115, 116, 255]
        )
    }
}
