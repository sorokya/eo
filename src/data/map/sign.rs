#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{decode_map_string, encode_map_string, SIGN_SIZE};
use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sign {
    pub x: EOChar,
    pub y: EOChar,
    pub title: String,
    pub message: String,
}

impl Sign {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Sign {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.x = reader.get_char();
        self.y = reader.get_char();
        let text_length = reader.get_short() - 1;
        if reader.remaining() >= text_length as usize {
            let sign_text = decode_map_string(reader.get_vec(text_length as usize));
            let title_length = reader.get_char();
            self.title = sign_text.chars().take(title_length as usize).collect();
            self.message = sign_text.chars().skip(title_length as usize).collect();
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut sign_text = String::from(&self.title);
        sign_text.push_str(&self.message);

        let mut builder = StreamBuilder::with_capacity(SIGN_SIZE + 2 + sign_text.len());
        builder.add_char(self.x);
        builder.add_char(self.y);
        builder.add_short(sign_text.len() as EOShort + 1);
        builder.append(&mut encode_map_string(&sign_text, sign_text.len()));
        builder.add_char(self.title.len() as EOChar);
        builder.get()
    }
}
