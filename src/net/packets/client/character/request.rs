use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};

pub const REQUEST_SIZE: usize = 4;

#[derive(Debug, Default)]
pub struct Request {}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, _reader: &StreamReader) {}
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REQUEST_SIZE);
        builder.add_break_string("NEW");
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, Serializeable};

    #[test]
    fn serialize() {
        let packet = Request::new();
        assert_eq!(packet.serialize(), [78, 69, 87, 255])
    }
}
