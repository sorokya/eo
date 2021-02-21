use super::{EOByte, StreamReader};

/// provides types with serialization functions
pub trait Serializeable {
    /// deserializes a byte array into a type
    fn deserialize(&mut self, reader: &mut StreamReader);
    /// serializes a type into a byte array
    fn serialize(&self) -> Vec<EOByte>;
}
