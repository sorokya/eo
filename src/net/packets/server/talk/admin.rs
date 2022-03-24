use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default, Clone)]
pub struct Admin {
    pub name: String,
    pub message: String,
}

impl Admin {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Admin {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.message = reader.get_break_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.name.len() + self.message.len() + 2);
        builder.add_break_string(&self.name);
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
//         let mut admin = Admin::new();
//         let reader = StreamReader::new(&data);
//         admin.deserialize(&reader);
//     }
//     #[test]
//     fn serialize() {
//         todo!();
//         let mut admin = Admin::new();
//         assert_eq!(
//             admin.serialize(),
//             []
//         )
//     }
// }
