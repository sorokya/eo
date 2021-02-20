use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR};

#[derive(Debug, Default)]
pub struct InitFriendListPlayers {
    pub names: Vec<String>,
}

impl InitFriendListPlayers {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for InitFriendListPlayers {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        let total_names = reader.get_short();
        self.names = Vec::with_capacity(total_names as usize);

        if reader.get_byte() != EO_BREAK_CHAR {
            panic!("Expected break char in packet");
        }

        for _ in 0..total_names {
            self.names.push(reader.get_break_string());
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.names.len() as EOShort);
        builder.add_byte(EO_BREAK_CHAR);
        for name in &self.names {
            builder.add_break_string(name);
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, InitFriendListPlayers, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![0x02, 0xFE, 0xFF, 0x61, 0x64, 0x6D, 0x69, 0x6E, 0xFF];
        let mut init_friend_list_players = InitFriendListPlayers::new();
        let mut reader = StreamReader::new(&buf);
        init_friend_list_players.deserialize(&mut reader);
        assert_eq!(init_friend_list_players.names.len(), 1);
        assert_eq!(init_friend_list_players.names[0], "admin");
    }

    #[test]
    fn serialize() {
        let mut init_friend_list_players = InitFriendListPlayers::new();
        init_friend_list_players.names = Vec::with_capacity(1);
        init_friend_list_players.names.push("admin".to_string());
        assert_eq!(
            init_friend_list_players.serialize(),
            [0x02, 0xFE, 0xFF, 0x61, 0x64, 0x6D, 0x69, 0x6E, 0xFF]
        );
    }
}
