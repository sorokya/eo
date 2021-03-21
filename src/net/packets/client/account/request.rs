use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default)]
pub struct Request {
    pub name: String,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.name = reader.get_end_string().to_lowercase();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.name.len());
        builder.add_string(&self.name.to_lowercase());
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![97, 100, 109, 105, 110];
        let mut packet = Request::new();
        let mut reader = StreamReader::new(&data);
        packet.deserialize(&mut reader);
        assert_eq!(packet.name, "admin");
    }
    #[test]
    fn serialize() {
        let mut packet = Request::new();
        packet.name = "admin".to_string();
        assert_eq!(packet.serialize(), [97, 100, 109, 105, 110])
    }
}
