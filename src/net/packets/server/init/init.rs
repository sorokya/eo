use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, Serializeable, StreamBuilder, StreamReader},
    net::replies::InitReply,
};

use super::{
    InitBanned, InitFileClass, InitFileItem, InitFileMap, InitFileNPC, InitFileSpell,
    InitFriendListPlayers, InitMapMutation, InitOk, InitOutOfDate, InitPlayers,
};

pub struct Init {
    pub reply_code: InitReply,
    pub reply: Box<dyn Serializeable>,
}

impl Init {
    pub fn new() -> Self {
        Self {
            reply_code: InitReply::default(),
            reply: Box::new(InitOk::default()),
        }
    }
}

impl Serializeable for Init {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_code_byte = reader.get_byte();
        self.reply_code = match InitReply::from_u8(reply_code_byte) {
            Some(reply_code) => reply_code,
            None => panic!("Failed to convert byte to InitReply: {}", reply_code_byte),
        };
        self.reply = match self.reply_code {
            InitReply::OutOfDate => Box::new(InitOutOfDate::new()),
            InitReply::OK => Box::new(InitOk::new()),
            InitReply::Banned => Box::new(InitBanned::new()),
            InitReply::FileMap => Box::new(InitFileMap::new()),
            InitReply::FileItem => Box::new(InitFileItem::new()),
            InitReply::FileNPC => Box::new(InitFileNPC::new()),
            InitReply::FileSpell => Box::new(InitFileSpell::new()),
            InitReply::Players => Box::new(InitPlayers::new()),
            InitReply::MapMutation => Box::new(InitMapMutation::new()),
            InitReply::FriendListPlayers => Box::new(InitFriendListPlayers::new()),
            InitReply::FileClass => Box::new(InitFileClass::new()),
        };
        self.reply.deserialize(reader);
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(1);
        builder.add_byte(self.reply_code as EOByte);
        builder.append(&mut self.reply.serialize());
        builder.get()
    }
}
