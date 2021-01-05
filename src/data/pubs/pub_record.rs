use super::super::{EOByte, StreamReader};

/// provides record types with serialization functions
pub trait PubRecord {
    /// serializes a pub record into a byte array
    fn serialize(&self) -> Vec<EOByte>;
    /// deserializes a byte array into a pub record
    fn deserialize(&mut self, reader: &mut StreamReader);
}
