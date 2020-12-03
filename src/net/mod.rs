use crate::data::EOInt;

mod action;
pub use action::Action;

mod family;
pub use family::Family;

/// flattens a packet [Family] and [Action] into a single [EOInt]
///
/// This is useful for identifying pairs of packet [Family]s, and [Action]s.
/// It is assumed there are a max of 255 actions per packet family.
///
/// # Example
/// ```
/// use eo::net::*;
///
/// assert_eq!(packet_id_hash(Family::Connection, Action::Request), 257)
/// ```
pub fn packet_id_hash(family: Family, action: Action) -> EOInt {
    ((family as EOInt) << 8) | action as EOInt
}

mod talk_reply;
pub use talk_reply::TalkReply;
mod init_reply;
pub use init_reply::InitReply;
mod file_type;
pub use file_type::FileType;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packet_id_hash() {
        assert_eq!(super::packet_id_hash(Family::Init, Action::Init), 65535);
        assert_eq!(
            super::packet_id_hash(Family::Connection, Action::Request),
            257
        );
        assert_eq!(super::packet_id_hash(Family::Welcome, Action::Reply), 1283);
    }
}
