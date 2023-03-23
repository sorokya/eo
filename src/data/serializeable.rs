use super::{StreamReader, StreamBuilder};

/// provides types with serialization functions
pub trait Serializeable {
    /// deserializes a byte array into a type
    fn deserialize(&mut self, reader: &StreamReader);
    /// serializes a type into a byte array
    fn serialize(&self, builder: &mut StreamBuilder);
}
