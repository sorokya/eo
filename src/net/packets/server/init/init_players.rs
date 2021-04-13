use crate::{
    data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR},
    net::OnlineEntry,
};

#[derive(Debug, Default)]
pub struct InitPlayers {
    pub players: Vec<OnlineEntry>,
}

impl InitPlayers {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for InitPlayers {
    fn deserialize(&mut self, reader: &StreamReader) {
        let total_players = reader.get_short();
        self.players = Vec::with_capacity(total_players as usize);

        if reader.get_byte() != EO_BREAK_CHAR {
            panic!("Expected break char in packet");
        }

        for _ in 0..total_players {
            let mut player = OnlineEntry::new();
            player.deserialize(reader);
            self.players.push(player);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.players.len() as EOShort);
        builder.add_byte(EO_BREAK_CHAR);
        for player in &self.players {
            builder.append(&mut player.serialize());
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use crate::{character::PaperdollIcon, net::OnlineEntry};

    use super::{EOByte, InitPlayers, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![
            0x02, 0xFE, 0xFF, 0x61, 0x64, 0x6D, 0x69, 0x6E, 0xFF, 0x74, 0x68, 0x65, 0x20, 0x67,
            0x72, 0x65, 0x61, 0x74, 0xFF, 0x01, 0x06, 0x02, 0x47, 0x4d, 0x20, 0xFF,
        ];
        let mut init_players = InitPlayers::new();
        let reader = StreamReader::new(&buf);
        init_players.deserialize(&reader);
        assert_eq!(init_players.players.len(), 1);
        assert_eq!(init_players.players[0].name, "admin");
        assert_eq!(init_players.players[0].title, "the great");
        assert_eq!(init_players.players[0].icon, PaperdollIcon::HighGameMaster);
        assert_eq!(init_players.players[0].class_id, 1);
        assert_eq!(init_players.players[0].guild_tag, "GM ");
    }

    #[test]
    fn serialize() {
        let mut init_players = InitPlayers::new();
        init_players.players = Vec::with_capacity(1);

        let mut player = OnlineEntry::new();
        player.name = "admin".to_string();
        player.title = "the great".to_string();
        player.icon = PaperdollIcon::HighGameMaster;
        player.class_id = 1;
        player.guild_tag = "GM ".to_string();
        init_players.players.push(player);

        assert_eq!(
            init_players.serialize(),
            [
                0x02, 0xFE, 0xFF, 0x61, 0x64, 0x6D, 0x69, 0x6E, 0xFF, 0x74, 0x68, 0x65, 0x20, 0x67,
                0x72, 0x65, 0x61, 0x74, 0xFF, 0x01, 0x06, 0x02, 0x47, 0x4d, 0x20, 0xFF,
            ]
        );
    }
}
