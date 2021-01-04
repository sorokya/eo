use super::super::{EOByte, StreamReader};

pub trait PubRecord: Clone {
    fn serialize(&self) -> Vec<EOByte>;
    fn deserialize(&mut self, reader: &mut StreamReader);
}
