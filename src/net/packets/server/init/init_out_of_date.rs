use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

pub const INIT_VERSION_SIZE: usize = 3;

#[derive(Debug, Default)]
pub struct InitOutOfDate {
    pub version: [EOChar; 3],
}

impl InitOutOfDate {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for InitOutOfDate {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        for char in self.version.iter_mut() {
            *char = reader.get_char();
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(INIT_VERSION_SIZE);
        for char in self.version.iter() {
            builder.add_char(*char);
        }

        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, InitOutOfDate, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![1, 1, 30];
        let mut init_out_of_date = InitOutOfDate::new();
        let mut reader = StreamReader::new(&buf);
        init_out_of_date.deserialize(&mut reader);
        assert_eq!(init_out_of_date.version, [0, 0, 29]);
    }

    #[test]
    fn serialize() {
        let mut init_out_of_date = InitOutOfDate::new();
        init_out_of_date.version = [0, 0, 29];
        assert_eq!(init_out_of_date.serialize(), [1, 1, 30]);
    }
}
