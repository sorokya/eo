use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default, Clone)]
pub struct Open {
    pub player_id: EOShort,
    pub message: String,
}

impl Open {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Open {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.message = reader.get_break_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.message.len() + 3);
        builder.add_short(self.player_id);
        builder.add_break_string(&self.message);
        builder.get()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn deserialize() {
//         todo!();
//         let data: Vec<EOByte> = vec![];
//         let mut open = Open::new();
//         let reader = StreamReader::new(&data);
//         open.deserialize(&reader);
//     }
//     #[test]
//     fn serialize() {
//         todo!();
//         let mut open = Open::new();
//         assert_eq!(open.serialize(), [])
//     }
// }
