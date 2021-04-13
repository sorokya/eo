use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

const SIZE: usize = 10;

#[derive(Debug, Default)]
pub struct Create {
    pub session_id: EOShort,
    pub name: String,
    pub password: String,
    pub fullname: String,
    pub location: String,
    pub email: String,
    pub computer: String,
    pub hdid: String,
}

impl Create {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Create {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.session_id = reader.get_short();
        reader.get_byte();
        self.name = reader.get_break_string();
        self.password = reader.get_break_string();
        self.fullname = reader.get_break_string();
        self.location = reader.get_break_string();
        self.email = reader.get_break_string();
        self.computer = reader.get_break_string();
        self.hdid = reader.get_break_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            SIZE + self.name.len()
                + self.password.len()
                + self.fullname.len()
                + self.location.len()
                + self.email.len()
                + self.computer.len()
                + self.hdid.len(),
        );

        builder.add_short(self.session_id);
        builder.add_byte(255);
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.password);
        builder.add_break_string(&self.fullname);
        builder.add_break_string(&self.location);
        builder.add_break_string(&self.email);
        builder.add_break_string(&self.computer);
        builder.add_break_string(&self.hdid);

        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Create, EOByte, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![
            242, 4, 255, 116, 101, 115, 116, 255, 112, 97, 115, 115, 119, 111, 114, 100, 255,
            116, 101, 115, 116, 255, 116, 101, 115, 255, 116, 101, 115, 116, 64, 101, 109, 97, 105,
            108, 46, 99, 111, 109, 255, 88, 80, 45, 86, 77, 255, 49, 56, 49, 54, 56, 52, 57, 56,
            57, 54, 255,
        ];
        let mut packet = Create::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.name, "test");
        assert_eq!(packet.password, "password");
        assert_eq!(packet.fullname, "test");
        assert_eq!(packet.location, "tes");
        assert_eq!(packet.email, "test@email.com");
        assert_eq!(packet.computer, "XP-VM");
        assert_eq!(packet.hdid, "1816849896");
    }
    #[test]
    fn serialize() {
        let mut packet = Create::new();
        packet.session_id = 1000;
        packet.name = "test".to_string();
        packet.password = "password".to_string();
        packet.fullname = "test".to_string();
        packet.location = "tes".to_string();
        packet.email = "test@email.com".to_string();
        packet.computer = "XP-VM".to_string();
        packet.hdid = "1816849896".to_string();
        assert_eq!(
            packet.serialize(),
            [
                242, 4, 255, 116, 101, 115, 116, 255, 112, 97, 115, 115, 119, 111, 114, 100,
                255, 116, 101, 115, 116, 255, 116, 101, 115, 255, 116, 101, 115, 116, 64, 101, 109,
                97, 105, 108, 46, 99, 111, 109, 255, 88, 80, 45, 86, 77, 255, 49, 56, 49, 54, 56,
                52, 57, 56, 57, 54, 255
            ]
        )
    }
}
