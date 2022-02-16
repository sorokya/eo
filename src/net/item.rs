use crate::data::{
    EOByte, EOChar, EOInt, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader,
};

pub const ITEM_SIZE: usize = 6;
#[derive(Debug, Default)]
pub struct Item {
    pub id: EOShort,
    pub amount: EOInt,
}

impl Item {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Item {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.id = reader.get_short();
        self.amount = reader.get_int();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ITEM_SIZE);
        builder.add_short(self.id);
        builder.add_int(self.amount);
        builder.get()
    }
}

pub const REVERSE_ITEM_SIZE: usize = 6;
#[derive(Debug, Default)]
pub struct ReverseItem {
    pub amount: EOInt,
    pub id: EOShort,
}

impl ReverseItem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ReverseItem {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.amount = reader.get_int();
        self.id = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REVERSE_ITEM_SIZE);
        builder.add_int(self.amount);
        builder.add_short(self.id);
        builder.get()
    }
}

pub const SHORT_ITEM_SIZE: usize = 5;
#[derive(Debug, Default)]
pub struct ShortItem {
    pub id: EOShort,
    pub amount: EOThree,
}

impl ShortItem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ShortItem {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.id = reader.get_short();
        self.amount = reader.get_three();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SHORT_ITEM_SIZE);
        builder.add_short(self.id);
        builder.add_three(self.amount);
        builder.get()
    }
}

pub const VERY_SHORT_ITEM_SIZE: usize = 3;
#[derive(Debug, Default)]
pub struct VeryShortItem {
    pub id: EOShort,
    pub amount: EOChar,
}

impl VeryShortItem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for VeryShortItem {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.id = reader.get_short();
        self.amount = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(VERY_SHORT_ITEM_SIZE);
        builder.add_short(self.id);
        builder.add_char(self.amount);
        builder.get()
    }
}

// TODO: tests
