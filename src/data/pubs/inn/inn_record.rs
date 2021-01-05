#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::{inn::EID_DATA_SIZE, PubRecord},
    {EOByte, EOChar, EOShort, StreamBuilder, StreamReader},
};

/// data structure of a single eid record
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InnRecord {
    /// links to the vendor_id of the inn keeper npc
    pub id: EOShort,
    /// name of the home
    pub name: String,
    /// default respawn map id
    pub spawn_map: EOShort,
    /// default respawn map x
    pub spawn_x: EOChar,
    /// default respawn map y
    pub spawn_y: EOChar,
    /// map id player warps to after "sleeping"
    pub inn_sleep_map: EOShort,
    /// map x player warps to after "sleeping"
    pub inn_sleep_x: EOChar,
    /// map y player warps to after "sleeping"
    pub inn_sleep_y: EOChar,
    /// tells the server if the inn uses an alternate spawn
    ///
    /// Only known use is the "Wanderer" home. Players will
    /// respawn on nubland until a requirement is met on the server
    /// then they will respawn at the alternate spawn location.
    pub alt_spawn_enabled: bool,
    /// alternate respawn map id
    pub alt_spawn_map: EOShort,
    /// alternate respawn map x
    pub alt_spawn_x: EOChar,
    /// alternate respawn map y
    pub alt_spawn_y: EOChar,
    /// question 1 text
    pub question1: String,
    /// answer 1 text
    pub answer1: String,
    /// question 2 text
    pub question2: String,
    /// answer 2 text
    pub answer2: String,
    /// question 3 text
    pub question3: String,
    /// answer 3 text
    pub answer3: String,
}

impl InnRecord {
    /// creates a new InnRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl PubRecord for InnRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.id = reader.get_short();
        self.name = reader.get_prefix_string();
        self.spawn_map = reader.get_short();
        self.spawn_x = reader.get_char();
        self.spawn_y = reader.get_char();
        self.inn_sleep_map = reader.get_short();
        self.inn_sleep_x = reader.get_char();
        self.inn_sleep_y = reader.get_char();
        self.alt_spawn_enabled = reader.get_char() == 1;
        self.alt_spawn_map = reader.get_short();
        self.alt_spawn_x = reader.get_char();
        self.alt_spawn_y = reader.get_char();
        self.question1 = reader.get_prefix_string();
        self.answer1 = reader.get_prefix_string();
        self.question2 = reader.get_prefix_string();
        self.answer2 = reader.get_prefix_string();
        self.question3 = reader.get_prefix_string();
        self.answer3 = reader.get_prefix_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            EID_DATA_SIZE
                + self.name.len()
                + self.question1.len()
                + self.answer1.len()
                + self.question2.len()
                + self.answer2.len()
                + self.question3.len()
                + self.answer3.len()
                + 7,
        );

        builder.add_short(self.id);
        builder.add_prefix_string(&self.name);
        builder.add_short(self.spawn_map);
        builder.add_char(self.spawn_x);
        builder.add_char(self.spawn_y);
        builder.add_short(self.inn_sleep_map);
        builder.add_char(self.inn_sleep_x);
        builder.add_char(self.inn_sleep_y);
        builder.add_char(self.alt_spawn_enabled as EOChar);
        builder.add_short(self.alt_spawn_map);
        builder.add_char(self.alt_spawn_x);
        builder.add_char(self.alt_spawn_y);
        builder.add_prefix_string(&self.question1);
        builder.add_prefix_string(&self.answer1);
        builder.add_prefix_string(&self.question2);
        builder.add_prefix_string(&self.answer2);
        builder.add_prefix_string(&self.question3);
        builder.add_prefix_string(&self.answer3);
        builder.get()
    }
}
