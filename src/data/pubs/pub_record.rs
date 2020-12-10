use super::super::EOByte;

pub trait PubRecord: Clone {
    fn serialize(&self) -> Vec<EOByte>;
    fn deserialize(&mut self, buf: &[EOByte]);
}
