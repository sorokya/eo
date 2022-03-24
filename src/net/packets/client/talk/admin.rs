use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default, Clone)]
pub struct Admin {
    pub message: String,
}

impl Admin {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Admin {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.message = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.message.len());
        builder.add_string(&self.message);
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
//         let mut admin = Admin::new();
//         let reader = StreamReader::new(&data);
//         admin.deserialize(&reader);
//     }
//     #[test]
//     fn serialize() {
//         todo!();
//         let mut admin = Admin::new();
//         assert_eq!(admin.serialize(), [])
//     }
// }
