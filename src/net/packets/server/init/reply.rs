use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, Serializeable, StreamBuilder, StreamReader},
    net::replies::InitReply,
};

use super::{
    ReplyBanned, ReplyFileClass, ReplyFileItem, ReplyFileMap, ReplyFileNPC, ReplyFileSpell,
    ReplyFriendListPlayers, ReplyMapMutation, ReplyOk, ReplyOutOfDate, ReplyPlayers,
};

// TODO: consider merging these into one?
pub struct Reply {
    pub reply_code: InitReply,
    pub reply: Box<dyn Serializeable + Send + Sync>,
}

impl Reply {
    pub fn new() -> Self {
        Self {
            reply_code: InitReply::default(),
            reply: Box::new(ReplyOk::default()),
        }
    }
}

impl Default for Reply {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Reply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self.reply.serialize();
        let reader = StreamReader::new(&buf);
        match self.reply_code {
            InitReply::OutOfDate => {
                let mut reply = ReplyOutOfDate::default();
                reply.deserialize(&reader);
                write!(f, "ReplyOutOfDate: {:?}", reply)
            }
            InitReply::OK => {
                let mut reply = ReplyOk::default();
                reply.deserialize(&reader);
                write!(f, "ReplyOk: {:?}", reply)
            }
            InitReply::Banned => {
                let mut reply = ReplyBanned::default();
                reply.deserialize(&reader);
                write!(f, "ReplyBanned: {:?}", reply)
            }
            InitReply::FileMap => {
                let mut reply = ReplyFileMap::default();
                reply.deserialize(&reader);
                write!(f, "ReplyFileMap: {:?}", reply)
            }
            InitReply::FileItem => {
                let mut reply = ReplyFileItem::default();
                reply.deserialize(&reader);
                write!(f, "ReplyFileItem: {:?}", reply)
            }
            InitReply::FileNPC => {
                let mut reply = ReplyFileNPC::default();
                reply.deserialize(&reader);
                write!(f, "ReplyFileNPC: {:?}", reply)
            }
            InitReply::FileSpell => {
                let mut reply = ReplyFileSpell::default();
                reply.deserialize(&reader);
                write!(f, "ReplyFileSpell: {:?}", reply)
            }
            InitReply::Players => {
                let mut reply = ReplyPlayers::default();
                reply.deserialize(&reader);
                write!(f, "ReplyPlayers: {:?}", reply)
            }
            InitReply::MapMutation => {
                let mut reply = ReplyMapMutation::default();
                reply.deserialize(&reader);
                write!(f, "ReplyMapMutation: {:?}", reply)
            }
            InitReply::FriendListPlayers => {
                let mut reply = ReplyFriendListPlayers::default();
                reply.deserialize(&reader);
                write!(f, "ReplyFriendListPlayers: {:?}", reply)
            }
            InitReply::FileClass => {
                let mut reply = ReplyFileClass::default();
                reply.deserialize(&reader);
                write!(f, "ReplyFileClass: {:?}", reply)
            }
        }
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_code_byte = reader.get_byte();
        self.reply_code = match InitReply::from_u8(reply_code_byte) {
            Some(reply_code) => reply_code,
            None => panic!("Failed to convert byte to InitReply: {}", reply_code_byte),
        };
        self.reply = match self.reply_code {
            InitReply::OutOfDate => Box::new(ReplyOutOfDate::new()),
            InitReply::OK => Box::new(ReplyOk::new()),
            InitReply::Banned => Box::new(ReplyBanned::new()),
            InitReply::FileMap => Box::new(ReplyFileMap::new()),
            InitReply::FileItem => Box::new(ReplyFileItem::new()),
            InitReply::FileNPC => Box::new(ReplyFileNPC::new()),
            InitReply::FileSpell => Box::new(ReplyFileSpell::new()),
            InitReply::Players => Box::new(ReplyPlayers::new()),
            InitReply::MapMutation => Box::new(ReplyMapMutation::new()),
            InitReply::FriendListPlayers => Box::new(ReplyFriendListPlayers::new()),
            InitReply::FileClass => Box::new(ReplyFileClass::new()),
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
