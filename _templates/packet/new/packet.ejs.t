---
to: src/net/packets/<%= type %>/<%= family %>/<%= action %>.rs
---

use crate::data::{Serializeable, StreamBuilder, StreamReader};

const SIZE: usize = todo!();

#[derive(Debug, Default, Clone)]
pub struct <%= h.capitalize(action) %> {
}

impl <%= h.capitalize(action) %> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for <%= h.capitalize(action) %> {
    fn deserialize(&mut self, reader: &StreamReader) {
        todo!();
    }
    fn serialize(&self) -> Vec<EOByte> {
        todo!();
        let mut builder = StreamBuilder::with_capacity(SIZE);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        todo!();
        let data: Vec<EOByte> = vec![];
        let mut <%= action %> = <%= h.capitalize(action) %>::new();
        let reader = StreamReader::new(&data);
        <%= action %>.deserialize(&reader);
    }
    #[test]
    fn serialize() {
        todo!();
        let mut <%= action %> = <%= h.capitalize(action) %>::new();
        assert_eq!(
            <%= action %>.serialize(),
            []
        )
    }
}
