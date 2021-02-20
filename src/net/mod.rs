use crate::data::EOThree;

/// number of bytes that describe a packet's data length
pub const PACKET_LENGTH_SIZE: usize = 2;

mod action;
pub use action::Action;
mod family;
pub use family::Family;
mod talk_reply;
pub use talk_reply::TalkReply;
mod init_reply;
pub use init_reply::InitReply;
mod init_ban_type;
pub use init_ban_type::InitBanType;
mod file_type;
pub use file_type::FileType;
mod online_entry;
pub use online_entry::{OnlineEntry, ONLINE_ENTRY_SIZE};
mod guild_reply;
pub use guild_reply::GuildReply;
mod guild_info_type;
pub use guild_info_type::GuildInfoType;
mod inn_unsubscribe_reply;
pub use inn_unsubscribe_reply::InnUnsubscribeReply;
mod skill_master_reply;
pub use skill_master_reply::SkillMasterReply;
mod party_request_type;
pub use party_request_type::PartyRequestType;
mod character_reply;
pub use character_reply::CharacterReply;
mod account_reply;
pub use account_reply::AccountReply;
mod login_reply;
pub use login_reply::LoginReply;
mod packet_processing;
pub use packet_processing::*;
mod client_state;
pub use client_state::ClientState;
pub mod packets;

pub fn stupid_hash(mut value: EOThree) -> EOThree {
    value += 1;
    110905 + (value % 9 + 1) * ((11092004 - value) % ((value % 11 + 1) * 119)) * 119 + value % 2004
}
