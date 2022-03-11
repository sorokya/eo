use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const REQUEST_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Request {
    pub player_ids: Vec<EOShort>,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_ids = Vec::new();
        while !reader.eof() {
            self.player_ids.push(reader.get_short());
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REQUEST_SIZE * self.player_ids.len());
        for player_id in &self.player_ids {
            builder.add_short(*player_id);
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 254];

        let mut packet = Request::default();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_ids.len(), 1);
        assert_eq!(packet.player_ids[0], 1);
    }
    #[test]
    fn serialize() {
        let mut packet = Request::new();
        packet.player_ids.push(1);
        assert_eq!(packet.serialize(), [2, 254]);
    }
}
