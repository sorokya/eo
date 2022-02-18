use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const PAPERDOLL_BAHWS_SIZE: usize = 10;
#[derive(Debug, Default)]
pub struct PaperdollBAHWS {
    pub boots: EOShort,
    pub armor: EOShort,
    pub hat: EOShort,
    pub weapon: EOShort,
    pub shield: EOShort,
}

impl PaperdollBAHWS {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollBAHWS {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        self.armor = reader.get_short();
        self.hat = reader.get_short();
        self.weapon = reader.get_short();
        self.shield = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(PAPERDOLL_BAHWS_SIZE);
        builder.add_short(self.boots);
        builder.add_short(self.armor);
        builder.add_short(self.hat);
        builder.add_short(self.weapon);
        builder.add_short(self.shield);
        builder.get()
    }
}

pub const PAPERDOLL_BAHSW_SIZE: usize = 10;
#[derive(Debug, Default)]
pub struct PaperdollBAHSW {
    pub boots: EOShort,
    pub armor: EOShort,
    pub hat: EOShort,
    pub shield: EOShort,
    pub weapon: EOShort,
}

impl PaperdollBAHSW {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollBAHSW {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        self.armor = reader.get_short();
        self.hat = reader.get_short();
        self.shield = reader.get_short();
        self.weapon = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(PAPERDOLL_BAHSW_SIZE);
        builder.add_short(self.boots);
        builder.add_short(self.armor);
        builder.add_short(self.hat);
        builder.add_short(self.shield);
        builder.add_short(self.weapon);
        builder.get()
    }
}

pub const PAPERDOLL_B000A0HSW_SIZE: usize = 18;
#[derive(Debug, Default)]
pub struct PaperdollB000A0HSW {
    pub boots: EOShort,
    pub armor: EOShort,
    pub hat: EOShort,
    pub shield: EOShort,
    pub weapon: EOShort,
}

impl PaperdollB000A0HSW {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollB000A0HSW {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        reader.get_short();
        reader.get_short();
        reader.get_short();
        self.armor = reader.get_short();
        reader.get_short();
        self.hat = reader.get_short();
        self.shield = reader.get_short();
        self.weapon = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(PAPERDOLL_B000A0HSW_SIZE);
        builder.add_short(self.boots);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(self.armor);
        builder.add_short(0);
        builder.add_short(self.hat);
        builder.add_short(self.shield);
        builder.add_short(self.weapon);
        builder.get()
    }
}

pub const PAPERDOLL_FULL_SIZE: usize = 30;
#[derive(Debug, Default, Clone)]
pub struct PaperdollFull {
    pub boots: EOShort,
    pub accessory: EOShort,
    pub gloves: EOShort,
    pub belt: EOShort,
    pub armor: EOShort,
    pub necklace: EOShort,
    pub hat: EOShort,
    pub shield: EOShort,
    pub weapon: EOShort,
    pub rings: [EOShort; 2],
    pub armlets: [EOShort; 2],
    pub bracers: [EOShort; 2],
}

impl PaperdollFull {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollFull {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        self.accessory = reader.get_short();
        self.gloves = reader.get_short();
        self.belt = reader.get_short();
        self.armor = reader.get_short();
        self.necklace = reader.get_short();
        self.hat = reader.get_short();
        self.shield = reader.get_short();
        self.weapon = reader.get_short();
        self.rings[0] = reader.get_short();
        self.rings[1] = reader.get_short();
        self.armlets[0] = reader.get_short();
        self.armlets[1] = reader.get_short();
        self.bracers[0] = reader.get_short();
        self.bracers[1] = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(PAPERDOLL_FULL_SIZE);
        builder.add_short(self.boots);
        builder.add_short(self.accessory);
        builder.add_short(self.gloves);
        builder.add_short(self.belt);
        builder.add_short(self.armor);
        builder.add_short(self.necklace);
        builder.add_short(self.hat);
        builder.add_short(self.shield);
        builder.add_short(self.weapon);
        builder.add_short(self.rings[0]);
        builder.add_short(self.rings[1]);
        builder.add_short(self.armlets[0]);
        builder.add_short(self.armlets[1]);
        builder.add_short(self.bracers[0]);
        builder.add_short(self.bracers[1]);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{PaperdollB000A0HSW, PaperdollBAHSW, PaperdollBAHWS, Serializeable, StreamReader};
    use crate::data::EOByte;

    #[test]
    fn deserialize_paperdoll_bahws() {
        let buf: Vec<EOByte> = vec![53, 254, 49, 254, 34, 254, 17, 254, 74, 254];
        let reader = StreamReader::new(&buf);
        let mut paperdoll_bahws = PaperdollBAHWS::new();
        paperdoll_bahws.deserialize(&reader);
        assert_eq!(paperdoll_bahws.boots, 52);
        assert_eq!(paperdoll_bahws.armor, 48);
        assert_eq!(paperdoll_bahws.hat, 33);
        assert_eq!(paperdoll_bahws.weapon, 16);
        assert_eq!(paperdoll_bahws.shield, 73);
    }
    #[test]
    fn serialize_paperdoll_bahws() {
        let mut paperdoll_bahws = PaperdollBAHWS::new();
        paperdoll_bahws.boots = 52;
        paperdoll_bahws.armor = 48;
        paperdoll_bahws.hat = 33;
        paperdoll_bahws.weapon = 16;
        paperdoll_bahws.shield = 73;

        assert_eq!(
            paperdoll_bahws.serialize(),
            [53, 254, 49, 254, 34, 254, 17, 254, 74, 254]
        );
    }

    #[test]
    fn deserialize_paperdoll_bahsw() {
        let buf: Vec<EOByte> = vec![53, 254, 49, 254, 34, 254, 74, 254, 17, 254];
        let reader = StreamReader::new(&buf);
        let mut paperdoll_bahws = PaperdollBAHSW::new();
        paperdoll_bahws.deserialize(&reader);
        assert_eq!(paperdoll_bahws.boots, 52);
        assert_eq!(paperdoll_bahws.armor, 48);
        assert_eq!(paperdoll_bahws.hat, 33);
        assert_eq!(paperdoll_bahws.shield, 73);
        assert_eq!(paperdoll_bahws.weapon, 16);
    }
    #[test]
    fn serialize_paperdoll_bahsw() {
        let mut paperdoll_bahws = PaperdollBAHSW::new();
        paperdoll_bahws.boots = 52;
        paperdoll_bahws.armor = 48;
        paperdoll_bahws.hat = 33;
        paperdoll_bahws.shield = 73;
        paperdoll_bahws.weapon = 16;

        assert_eq!(
            paperdoll_bahws.serialize(),
            [53, 254, 49, 254, 34, 254, 74, 254, 17, 254]
        );
    }

    #[test]
    fn deserialize_paperdoll_b000a0hsw() {
        let buf: Vec<EOByte> = vec![
            53, 254, 1, 254, 1, 254, 1, 254, 49, 254, 1, 254, 34, 254, 74, 254, 17, 254,
        ];
        let reader = StreamReader::new(&buf);
        let mut paperdoll_bahws = PaperdollB000A0HSW::new();
        paperdoll_bahws.deserialize(&reader);
        assert_eq!(paperdoll_bahws.boots, 52);
        assert_eq!(paperdoll_bahws.armor, 48);
        assert_eq!(paperdoll_bahws.hat, 33);
        assert_eq!(paperdoll_bahws.shield, 73);
        assert_eq!(paperdoll_bahws.weapon, 16);
    }
    #[test]
    fn serialize_paperdoll_b000a0hsw() {
        let mut paperdoll_bahws = PaperdollB000A0HSW::new();
        paperdoll_bahws.boots = 52;
        paperdoll_bahws.armor = 48;
        paperdoll_bahws.hat = 33;
        paperdoll_bahws.shield = 73;
        paperdoll_bahws.weapon = 16;

        assert_eq!(
            paperdoll_bahws.serialize(),
            [53, 254, 1, 254, 1, 254, 1, 254, 49, 254, 1, 254, 34, 254, 74, 254, 17, 254]
        );
    }
}
