use crate::{
    data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::replies::TalkReply,
};

const SIZE: usize = 2;

#[derive(Debug, Default, Clone)]
pub struct Reply {
    pub reply: TalkReply,
    pub name: String,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.reply = TalkReply::from_short(reader.get_short());
        self.name = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SIZE + self.name.len());
        builder.add_short(self.reply as EOShort);
        builder.add_string(&self.name);
        builder.get()
    }
}

// TODO: tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn deserialize() {}
//     #[test]
//     fn serialize() {}
// }
