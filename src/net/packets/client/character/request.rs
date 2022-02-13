use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default)]
pub struct Request {
    pub message: String,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.message = reader.get_break_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.message.len() + 1);
        builder.add_break_string(&self.message);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, Serializeable};
    use crate::data::{EOByte, StreamReader};

    #[test]
    fn serialize() {
        let mut packet = Request::new();
        packet.message = "NEW".to_string();
        assert_eq!(packet.serialize(), [78, 69, 87, 255])
    }

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![78, 69, 87, 255];
        let mut packet = Request::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.message, "NEW");
    }
}
