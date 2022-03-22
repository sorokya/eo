use crate::data::{EOByte, Serializeable, StreamBuilder, StreamReader};
use crate::net::BanType;

pub const INIT_BANNED_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct ReplyBanned {
    pub ban_type: BanType,
    /// remaining ban duration in minutes
    pub duration: EOByte,
}

impl ReplyBanned {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_duration(duration: EOByte) -> Self {
        Self {
            ban_type: BanType::Temporary,
            duration,
        }
    }
}

impl Serializeable for ReplyBanned {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.ban_type = BanType::from_byte(reader.get_byte());
        self.duration = match self.ban_type {
            BanType::Temporary => reader.get_byte(),
            _ => 0,
        };
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(INIT_BANNED_SIZE);
        builder.add_byte(self.ban_type as EOByte);
        if self.ban_type == BanType::Temporary {
            builder.add_byte(self.duration);
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{BanType, EOByte, ReplyBanned, Serializeable, StreamReader};

    #[test]
    fn serialize_temporary() {
        let mut init_ban = ReplyBanned::new();
        init_ban.ban_type = BanType::Temporary;
        init_ban.duration = 30;

        assert_eq!(init_ban.serialize(), [0, 30]);
    }
    #[test]
    fn deserialize_temporary() {
        let data: Vec<EOByte> = vec![0, 30];
        let mut init_ban = ReplyBanned::new();
        let reader = StreamReader::new(&data);
        init_ban.deserialize(&reader);
        assert_eq!(init_ban.ban_type, BanType::Temporary);
        assert_eq!(init_ban.duration, 30);
    }
    #[test]
    fn serialize_permanent() {
        let mut init_ban = ReplyBanned::new();
        init_ban.ban_type = BanType::Permanent;

        assert_eq!(init_ban.serialize(), [2]);
    }
    #[test]
    fn deserialize_permanent() {
        let data: Vec<EOByte> = vec![2];
        let mut init_ban = ReplyBanned::new();
        let reader = StreamReader::new(&data);
        init_ban.deserialize(&reader);
        assert_eq!(init_ban.ban_type, BanType::Permanent);
        assert_eq!(init_ban.duration, 0);
    }
}
