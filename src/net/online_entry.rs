use crate::{
    character::PaperdollIcon,
    data::{EOChar, Serializeable, StreamBuilder, StreamReader},
};

pub const ONLINE_ENTRY_SIZE: usize = 6;

#[derive(Debug, Default)]
pub struct OnlineEntry {
    pub name: String,
    pub title: String,
    unk: EOChar,
    pub icon: PaperdollIcon,
    pub class_id: EOChar,
    pub guild_tag: String,
}

impl OnlineEntry {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for OnlineEntry {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.title = reader.get_break_string();
        self.unk = reader.get_char();
        self.icon = PaperdollIcon::from_char(reader.get_char());
        self.class_id = reader.get_char();
        self.guild_tag = reader.get_break_string();
    }
    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder =
            StreamBuilder::with_capacity(ONLINE_ENTRY_SIZE + self.name.len() + self.title.len());
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.title);
        builder.add_char(self.unk);
        builder.add_char(self.icon as EOChar);
        builder.add_char(self.class_id);
        builder.add_break_string(&self.guild_tag);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{OnlineEntry, PaperdollIcon, Serializeable, StreamReader};
    use crate::data::EOByte;

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![
            0x61, 0x64, 0x6D, 0x69, 0x6E, 0xFF, 0x74, 0x68, 0x65, 0x20, 0x67, 0x72, 0x65, 0x61,
            0x74, 0xFF, 0x01, 0x06, 0x02, 0x47, 0x4d, 0x20, 0xFF,
        ];
        let reader = StreamReader::new(&buf);
        let mut online_entry = OnlineEntry::new();
        online_entry.deserialize(&reader);
        assert_eq!(online_entry.name, "admin");
        assert_eq!(online_entry.title, "the great");
        assert_eq!(online_entry.icon, PaperdollIcon::HighGameMaster);
        assert_eq!(online_entry.class_id, 1);
        assert_eq!(online_entry.guild_tag, "GM ");
    }
    #[test]
    fn serialize() {
        let mut online_entry = OnlineEntry::new();
        online_entry.name = "admin".to_string();
        online_entry.title = "the great".to_string();
        online_entry.icon = PaperdollIcon::HighGameMaster;
        online_entry.class_id = 1;
        online_entry.guild_tag = "GM ".to_string();

        assert_eq!(
            online_entry.serialize(),
            [
                0x61, 0x64, 0x6D, 0x69, 0x6E, 0xFF, 0x74, 0x68, 0x65, 0x20, 0x67, 0x72, 0x65, 0x61,
                0x74, 0xFF, 0x01, 0x06, 0x02, 0x47, 0x4d, 0x20, 0xFF,
            ]
        );
    }
}
