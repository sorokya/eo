use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const REQUEST_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Request {
    pub player_id: EOShort,
}

impl Request {
    pub fn new(player_id: EOShort) -> Self {
        Self {
            player_id,
        }
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REQUEST_SIZE);
        builder.add_short(self.player_id);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, EOByte, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 254];

        let mut packet = Request::default();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_id, 1);
    }
    #[test]
    fn serialize() {
        let packet = Request::new(1);
        assert_eq!(packet.serialize(), [2, 254]);
    }
}
