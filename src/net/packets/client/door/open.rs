use crate::{
    data::{EOByte, Serializeable, StreamReader, EOChar},
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
        self.coords.deserialize(reader);
    }
    fn serialize(&self) -> Vec<EOByte> {
        self.coords.serialize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let buf = vec![37, 36];
        let mut open = Open::default();
        let reader = StreamReader::new(&buf);
        open.deserialize(&reader);
        assert_eq!(open.coords, TinyCoords::new(36, 35));
    }
    #[test]
    fn serialize() {
        let open = Open::new(36, 35);
        assert_eq!(open.serialize(), [37, 36]);
    }
}