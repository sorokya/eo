/// number of bytes that describe a packet's data length
pub const PACKET_LENGTH_SIZE: usize = 2;
/// number of bytes that describe a packet's action/family
pub const PACKET_HEADER_SIZE: usize = 2;

mod action;
pub use action::Action;
mod family;
pub use family::Family;
mod character_info;
pub use character_info::{CharacterInfo, CHARACTER_INFO_SIZE};
mod character_list;
pub use character_list::CharacterList;
mod init_ban_type;
pub use init_ban_type::InitBanType;
mod file_type;
pub use file_type::FileType;
mod online_entry;
pub use online_entry::{OnlineEntry, ONLINE_ENTRY_SIZE};
mod guild_info_type;
pub use guild_info_type::GuildInfoType;
mod inn_unsubscribe_reply;
pub use inn_unsubscribe_reply::InnUnsubscribeReply;
mod party_request_type;
pub use party_request_type::PartyRequestType;
mod packet_processing;
pub use packet_processing::*;
mod client_state;
pub use client_state::ClientState;
pub mod packets;
mod paperdoll;
pub mod replies;
pub use paperdoll::{
    PaperdollB000A0HSW, PaperdollBAHSW, PaperdollBAHWS, PaperdollFull, PAPERDOLL_B000A0HSW_SIZE,
    PAPERDOLL_BAHSW_SIZE, PAPERDOLL_BAHWS_SIZE, PAPERDOLL_FULL_SIZE,
};

pub fn stupid_hash(mut value: crate::data::EOThree) -> crate::data::EOThree {
    value += 1;
    110905 + (value % 9 + 1) * ((11092004 - value) % ((value % 11 + 1) * 119)) * 119 + value % 2004
}
