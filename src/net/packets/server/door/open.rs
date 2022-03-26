use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    world::TinyCoords,
};

#[derive(Debug, Default)]
pub struct Open {
    pub coords: TinyCoords,
}

impl Open {
    pub fn new(x: EOChar, y: EOChar) -> Self {
        Self {
            coords: TinyCoords::new(x, y),
        }
    }
}

impl Serializeable for Open {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.coords = TinyCoords::new(reader.get_char(), reader.get_short() as EOChar)
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(3);
        builder.add_char(self.coords.x);
        builder.add_short(self.coords.y as EOShort);
        builder.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let buf = vec![37, 36, 254];
        let mut open = Open::default();
        let reader = StreamReader::new(&buf);
        open.deserialize(&reader);
        assert_eq!(open.coords, TinyCoords::new(36, 35));
    }
    #[test]
    fn serialize() {
        let open = Open::new(36, 35);
        assert_eq!(open.serialize(), [37, 36, 254]);
    }
}
