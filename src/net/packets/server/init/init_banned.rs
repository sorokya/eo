use num_traits::FromPrimitive;

use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};
use crate::net::InitBanType;

pub const INIT_BANNED_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct InitBanned {
    pub ban_type: InitBanType,
    /// remaining ban duration in minutes
    pub duration: EOByte,
}

impl InitBanned {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for InitBanned {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        let ban_type_byte = reader.get_byte();
        self.ban_type = match InitBanType::from_u8(ban_type_byte) {
            Some(ban_type) => ban_type,
            _ => panic!("Failed to convert byte: {} to InitBanType", ban_type_byte),
        };
        self.duration = match self.ban_type {
            InitBanType::Temporary => reader.get_byte(),
            _ => 0,
        };
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(INIT_BANNED_SIZE);
        builder.add_byte(self.ban_type as EOByte);
        if self.ban_type == InitBanType::Temporary {
            builder.add_byte(self.duration);
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, InitBanType, InitBanned, Serializeable, StreamReader};

    #[test]
    fn serialize_temporary() {
        let mut init_ban = InitBanned::new();
        init_ban.ban_type = InitBanType::Temporary;
        init_ban.duration = 30;

        assert_eq!(init_ban.serialize(), [0, 30]);
    }
    #[test]
    fn deserialize_temporary() {
        let data: Vec<EOByte> = vec![0, 30];
        let mut init_ban = InitBanned::new();
        let mut reader = StreamReader::new(&data);
        init_ban.deserialize(&mut reader);
        assert_eq!(init_ban.ban_type, InitBanType::Temporary);
        assert_eq!(init_ban.duration, 30);
    }
    #[test]
    fn serialize_permanent() {
        let mut init_ban = InitBanned::new();
        init_ban.ban_type = InitBanType::Permanent;

        assert_eq!(init_ban.serialize(), [2]);
    }
    #[test]
    fn deserialize_permanent() {
        let data: Vec<EOByte> = vec![2];
        let mut init_ban = InitBanned::new();
        let mut reader = StreamReader::new(&data);
        init_ban.deserialize(&mut reader);
        assert_eq!(init_ban.ban_type, InitBanType::Permanent);
        assert_eq!(init_ban.duration, 0);
    }
}
