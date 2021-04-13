#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::class::ECF_DATA_SIZE, EOByte, EOChar, EOInt, EOShort, Serializeable, StreamBuilder,
    StreamReader,
};

/// data structure of a single ecf record
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClassRecord {
    /// used to identify a class
    ///
    /// it is the record's index in the [super::ClassFile]
    pub id: EOInt,
    /// the name of the class
    pub name: String,
    /// links to another class's id
    ///
    /// This makes the class a sub-class of the linked class.
    /// It is used for class requirements in the game. Anything
    /// that requires a base class will also allow a sub-class.
    pub base: EOChar,
    /// an identifier for what formula will be used for stat calculations
    pub class_type: EOChar,
    /// bonus strength points provided by class
    pub strength: EOShort,
    /// bonus intelligence points provided by class
    pub intelligence: EOShort,
    /// bonus wisdom points provided by class
    pub wisdom: EOShort,
    /// bonus agility points provided by class
    pub agility: EOShort,
    /// bonus constitution points provided by class
    pub constitution: EOShort,
    /// bonus charisma points provided by class
    pub charisma: EOShort,
}

impl ClassRecord {
    /// creates a new record with the given id
    ///
    /// # Examples
    /// ```
    /// use eo::data::pubs::ClassRecord;
    ///
    /// let record = ClassRecord::new(1);
    /// assert_eq!(record.id, 1);
    /// ```
    pub fn new(id: EOInt) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl Serializeable for ClassRecord {
    /// deserializes data from a [StreamReader] into [ClassRecord]
    ///
    /// # Examples
    /// ```
    /// use eo::data::{EOByte, pubs::ClassRecord, Serializeable, StreamReader};
    /// let buf: Vec<EOByte> = vec![
    ///     0x08, 0x57, 0x61, 0x72, 0x72, 0x69, 0x6F, 0x72, 0x01, 0x01, 0x03, 0xFE, 0x01, 0xFE,
    ///     0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
    /// ];
    ///
    /// let reader = StreamReader::new(&buf);
    /// let mut record = ClassRecord::new(1);
    /// record.deserialize(&reader);
    /// assert_eq!(record.name, "Warrior");
    /// assert_eq!(record.strength, 2);
    /// ```
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_prefix_string();
        self.base = reader.get_char();
        self.class_type = reader.get_char();
        self.strength = reader.get_short();
        self.intelligence = reader.get_short();
        self.wisdom = reader.get_short();
        self.agility = reader.get_short();
        self.constitution = reader.get_short();
        self.charisma = reader.get_short();
    }
    /// serializes data from a [ClassRecord] into a [Vec]
    ///
    /// # Examples
    /// ```
    /// use eo::data::{EOByte, pubs::ClassRecord, Serializeable};
    ///
    /// let mut record = ClassRecord::new(1);
    /// record.name = "Warrior".to_string();
    ///
    /// let buf = record.serialize();
    /// assert_eq!(buf, [
    ///     0x08, 0x57, 0x61, 0x72, 0x72, 0x69, 0x6F, 0x72, 0x01, 0x01, 0x01, 0xFE, 0x01, 0xFE,
    ///     0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
    /// ]);
    /// ```
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ECF_DATA_SIZE + self.name.len() + 1);
        builder.add_prefix_string(&self.name);
        builder.add_char(self.base);
        builder.add_char(self.class_type);
        builder.add_short(self.strength);
        builder.add_short(self.intelligence);
        builder.add_short(self.wisdom);
        builder.add_short(self.agility);
        builder.add_short(self.constitution);
        builder.add_short(self.charisma);
        builder.get()
    }
}
