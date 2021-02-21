use crate::data::{EOByte, EOChar, EOThree, Serializeable, StreamBuilder, StreamReader};

const SIZE: usize = 6;

#[derive(Debug, Default)]
pub struct Init {
    pub challenge: EOThree,
    pub version: [EOChar; 3],
    pub hdid: String,
}

impl Init {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Init {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.challenge = reader.get_three();
        for i in 0..3 {
            self.version[i] = reader.get_char();
        }
        reader.get_char();
        reader.get_char();
        self.hdid = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SIZE + self.hdid.len());
        builder.add_three(self.challenge);
        for version_char in self.version.iter() {
            builder.add_char(*version_char);
        }
        builder.add_char(112); // always 112?
        builder.add_prefix_string(&self.hdid);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Init, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![
            222, 108, 4, 1, 1, 30, 113, 11, 52, 48, 55, 49, 54, 54, 54, 52, 48, 50,
        ];

        let mut packet = Init::new();
        let mut reader = StreamReader::new(&data);
        packet.deserialize(&mut reader);
        assert_eq!(packet.challenge, 219319);
        assert_eq!(packet.version, [0, 0, 29]);
        assert_eq!(packet.hdid, "4071666402");
    }
    #[test]
    fn serialize() {
        let mut packet = Init::new();
        packet.challenge = 219319;
        packet.version = [0, 0, 29];
        packet.hdid = "4071666402".to_string();
        assert_eq!(
            packet.serialize(),
            [222, 108, 4, 1, 1, 30, 113, 11, 52, 48, 55, 49, 54, 54, 54, 52, 48, 50,]
        );
    }
}
