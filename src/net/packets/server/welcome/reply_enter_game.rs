use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR},
    net::{
        replies::WelcomeReply, Item, NearbyInfo, Spell, Weight, ITEM_SIZE, NEARBY_INFO_SIZE,
        SPELL_SIZE, WEIGHT_SIZE,
    },
};

const REPLY_SIZE: usize = WEIGHT_SIZE + NEARBY_INFO_SIZE + 12;
#[derive(Debug, Default)]
pub struct ReplyEnterGame {
    pub reply: WelcomeReply,
    pub news: [String; 9],
    pub weight: Weight,
    pub items: Vec<Item>,
    pub spells: Vec<Spell>,
    pub nearby: NearbyInfo,
}

impl ReplyEnterGame {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ReplyEnterGame {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_char = reader.get_char();
        self.reply = match WelcomeReply::from_u8(reply_char) {
            Some(reply) => reply,
            None => panic!("Failed to convert char to WelcomeReply: {}", reply_char),
        };
        for i in 0..9 {
            self.news[i] = reader.get_break_string();
        }
        self.weight.deserialize(reader);
        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut item = Item::new();
            item.deserialize(reader);
            self.items.push(item);
        }
        reader.get_byte();
        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut spell = Spell::new();
            spell.deserialize(reader);
            self.spells.push(spell);
        }
        self.nearby.deserialize(reader);
    }
    fn serialize(&self) -> Vec<EOByte> {
        let news_size = self.news.iter().fold(0, |acc, news| acc + news.len());
        let mut builder = StreamBuilder::with_capacity(
            REPLY_SIZE + news_size + self.items.len() * ITEM_SIZE + self.spells.len() * SPELL_SIZE,
        );
        builder.add_char(self.reply as EOChar);
        for i in 0..9 {
            builder.add_break_string(&self.news[i]);
        }
        builder.append(&mut self.weight.serialize());
        for item in &self.items {
            builder.append(&mut item.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        for spell in &self.spells {
            builder.append(&mut spell.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        builder.append(&mut self.nearby.serialize());
        builder.get()
    }
}
