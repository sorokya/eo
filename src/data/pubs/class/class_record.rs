use crate::data::{
    pubs::{class::ECF_DATA_SIZE, PubRecord},
    *,
};

/// data structure of a single ecf record
#[derive(Debug, Clone, Default)]
pub struct ClassRecord {
    /// used to identify a class
    ///
    /// it is the record's index in the [ClassFile]
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
    pub fn new(id: EOInt) -> Self {
        let mut record = Self::default();
        record.id = id;
        record
    }
}

impl PubRecord for ClassRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
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
