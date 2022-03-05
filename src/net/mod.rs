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
mod ban_type;
pub use ban_type::BanType;
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
mod character_stats;
pub use character_stats::{
    CharacterBaseStats, CharacterSecondaryStats, CharacterStats1, CharacterStats2, CharacterStats3,
    CharacterStats4, CHARACTER_BASE_STATS_SIZE, CHARACTER_SECONDARY_STATS_SIZE,
    CHARACTER_STATS_1_SIZE, CHARACTER_STATS_2_SIZE, CHARACTER_STATS_3_SIZE, CHARACTER_STATS_4_SIZE,
};
mod server_settings;
pub use server_settings::{ServerSettings, SERVER_SETTINGS_SIZE};
mod weight;
pub use weight::{Weight, WEIGHT_SIZE};
mod item;
pub use item::{
    Item, ReverseItem, ShortItem, VeryShortItem, ITEM_SIZE, REVERSE_ITEM_SIZE, SHORT_ITEM_SIZE,
    VERY_SHORT_ITEM_SIZE,
};
mod spell;
pub use spell::{Spell, SPELL_SIZE};
mod nearby_info;
pub use nearby_info::{NearbyInfo, NEARBY_INFO_SIZE};
mod character_map_info;
pub use character_map_info::{CharacterMapInfo, CHARACTER_MAP_INFO_SIZE};
mod item_map_info;
pub use item_map_info::{ItemMapInfo, ITEM_MAP_INFO_SIZE};
mod npc_map_info;
pub use npc_map_info::{NpcMapInfo, NPC_MAP_INFO_SIZE};

pub fn stupid_hash(mut value: crate::data::EOThree) -> crate::data::EOThree {
    value += 1;
    110905 + (value % 9 + 1) * ((11092004 - value) % ((value % 11 + 1) * 119)) * 119 + value % 2004
}
