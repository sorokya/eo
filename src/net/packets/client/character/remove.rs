use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

const REMOVE_SIZE: usize = 4;

#[derive(Debug, Default)]
pub struct Remove {
    pub session_id: EOShort,
    pub character_id: EOShort,
}

impl Remove {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Remove {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.session_id = reader.get_short();
        self.character_id = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REMOVE_SIZE);
        builder.add_short(self.session_id);
        builder.add_short(self.character_id);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{EOByte, StreamReader};

    #[test]
    fn serialize() {
        let mut remove = Remove::new();
        remove.session_id = 49077;
        remove.character_id = 10;
        assert_eq!(remove.serialize(), [249, 194, 11, 254])
    }

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![249, 194, 11, 254];
        let mut remove = Remove::new();
        let reader = StreamReader::new(&data);
        remove.deserialize(&reader);
        assert_eq!(remove.session_id, 49077);
        assert_eq!(remove.character_id, 10);
    }
}
