use crate::data::{EOByte, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader};

pub const INIT_OK_SIZE: usize = 9;

#[derive(Debug, Default)]
pub struct InitOk {
    pub sequence_bytes: [EOByte; 2],
    pub encoding_multiples: [EOByte; 2],
    pub player_id: EOShort,
    pub challenge_response: EOThree,
}

impl InitOk {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for InitOk {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        for byte in self.sequence_bytes.iter_mut() {
            *byte = reader.get_byte();
        }

        for byte in self.encoding_multiples.iter_mut() {
            *byte = reader.get_byte();
        }

        self.player_id = reader.get_short();
        self.challenge_response = reader.get_three();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(INIT_OK_SIZE);
        for byte in self.sequence_bytes.iter() {
            builder.add_byte(*byte);
        }

        for byte in self.encoding_multiples.iter() {
            builder.add_byte(*byte);
        }

        builder.add_short(self.player_id);
        builder.add_three(self.challenge_response);

        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, InitOk, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![7, 10, 5, 9, 2, 254, 2, 254, 254];
        let mut init_ok = InitOk::new();
        let mut reader = StreamReader::new(&buf);
        init_ok.deserialize(&mut reader);

        assert_eq!(init_ok.sequence_bytes, [7, 10]);
        assert_eq!(init_ok.encoding_multiples, [5, 9]);
        assert_eq!(init_ok.player_id, 1);
        assert_eq!(init_ok.challenge_response, 1);
    }

    #[test]
    fn serialize() {
        let mut init_ok = InitOk::new();
        init_ok.sequence_bytes = [7, 10];
        init_ok.encoding_multiples = [5, 9];
        init_ok.player_id = 1;
        init_ok.challenge_response = 1;

        assert_eq!(init_ok.serialize(), [7, 10, 5, 9, 2, 254, 2, 254, 254]);
    }
}
