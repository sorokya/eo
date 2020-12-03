use num_traits::FromPrimitive;

use super::{Action, Family};
use crate::data::{EOByte, StreamReader};

/// Represents an EO network packet
///
/// Each packet has an [Action], and [Family].
///
/// Packets can be created via [Packet::deserialize], [Packet::from_slice], and [Packet::from_vec].
pub struct Packet {
    data: Vec<EOByte>,
    pub action: Action,
    pub family: Family,
}

impl Packet {
    /// creates a new [Packet] from a byte array
    ///
    /// `data` is expected to be structured like this
    ///
    /// [Family, Action, Remaining data...]
    ///
    /// With the first two bytes being the packet action, and family and the
    /// remaining bytes being the rest of the packet data.
    pub fn deserialize(data: &[EOByte]) -> Self {
        Self {
            action: match Action::from_u8(data[0]) {
                Some(a) => a,
                None => panic!("Invalid packet action: {}", data[0]),
            },
            family: match Family::from_u8(data[1]) {
                Some(f) => f,
                None => panic!("Invalid packet family: {}", data[1]),
            },
            data: data[2..].to_vec(),
        }
    }
    /// creates a new [Packet] with the provided action, family, and data
    ///
    /// data is Copied into a new Vec using [to_vec](https://doc.rust-lang.org/std/primitive.slice.html#method.to_vec)
    pub fn from_slice(action: Action, family: Family, data: &[EOByte]) -> Self {
        Self {
            action,
            family,
            data: data.to_vec(),
        }
    }
    /// creates a new [Packet] with the provided action, family, and data
    ///
    /// data is Moved instead of Copied
    pub fn from_vec(action: Action, family: Family, data: Vec<EOByte>) -> Self {
        Self {
            action,
            family,
            data,
        }
    }
    /// serialized packet into a byte array
    pub fn serialize(&mut self) -> Vec<EOByte> {
        let mut buf: Vec<EOByte> = Vec::with_capacity(self.data.len() + 2);
        buf.push(self.action as EOByte);
        buf.push(self.family as EOByte);
        buf.append(&mut self.data);
        buf
    }
    /// creates a [StreamReader] off the packet data
    pub fn get_reader(&self) -> StreamReader {
        StreamReader::new(&self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::StreamBuilder;

    const INIT_PACKET_BYTES: [EOByte; 17] = [
        255, 255, 190, 2, 254, 1, 1, 29, 113, 8, 50, 57, 48, 49, 49, 51, 50,
    ];

    fn get_stream_builder() -> StreamBuilder {
        let hdid = String::from("2901132");
        let mut builder = StreamBuilder::with_capacity(10 + hdid.len());
        builder.add_three(442); // TODO: generate challenge
        builder.add_char(0); // version major
        builder.add_char(0); // version minor
        builder.add_char(28); // version build
        builder.add_char(112); // ?
        builder.add_prefix_string(&hdid);
        builder
    }

    #[test]
    fn deserialize() {
        let packet = Packet::deserialize(&INIT_PACKET_BYTES);
        assert_eq!(packet.action, Action::Init);
        assert_eq!(packet.family, Family::Init);
        assert_eq!(packet.data.as_slice(), &INIT_PACKET_BYTES[2..]);
    }
    #[test]
    fn serialize() {
        let builder = get_stream_builder();
        let mut packet = Packet::from_vec(Action::Init, Family::Init, builder.get());
        assert_eq!(packet.serialize().as_slice(), &INIT_PACKET_BYTES);
    }
    #[test]
    fn from_slice() {
        let packet = Packet::from_slice(Action::Init, Family::Init, &INIT_PACKET_BYTES[2..]);
        assert_eq!(packet.action, Action::Init);
        assert_eq!(packet.family, Family::Init);
        assert_eq!(packet.data.as_slice(), &INIT_PACKET_BYTES[2..]);
    }
    #[test]
    fn from_vec() {
        let builder = get_stream_builder();
        let packet = Packet::from_vec(Action::Init, Family::Init, builder.get());
        assert_eq!(packet.action, Action::Init);
        assert_eq!(packet.family, Family::Init);
        assert_eq!(packet.data.as_slice(), &INIT_PACKET_BYTES[2..]);
    }
    #[test]
    fn get_reader() {
        let builder = get_stream_builder();
        let packet = Packet::from_vec(Action::Init, Family::Init, builder.get());
        let mut reader = packet.get_reader();
        assert_eq!(reader.get_three(), 442);
        assert_eq!(reader.get_char(), 0);
        assert_eq!(reader.get_char(), 0);
        assert_eq!(reader.get_char(), 28);
    }
    #[test]
    #[should_panic]
    fn invalid_action() {
        Packet::deserialize(&[0, 1]);
    }
    #[test]
    #[should_panic]
    fn invalid_family() {
        Packet::deserialize(&[1, 0]);
    }
}
