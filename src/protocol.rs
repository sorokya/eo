// WARNING! This file was generated automatically. Do NOT edit it manually.
// https://github.com/sorokya/eo_protocol_parser

use crate::data::{
    EOByte, EOChar, EOInt, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader,
    EO_BREAK_CHAR,
};
use log::warn;

pub const PACKET_FAMILY_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketFamily {
    Connection,
    Account,
    Character,
    Login,
    Welcome,
    Walk,
    Face,
    Chair,
    Emote,
    Attack,
    Spell,
    Shop,
    Item,
    StatSkill,
    Global,
    Talk,
    Warp,
    Jukebox,
    Players,
    Avatar,
    Party,
    Refresh,
    NPC,
    PlayerRange,
    NPCRange,
    Range,
    Paperdoll,
    Effect,
    Trade,
    Chest,
    Door,
    Message,
    Bank,
    Locker,
    Barber,
    Guild,
    Music,
    Sit,
    Recover,
    Board,
    Cast,
    Arena,
    Priest,
    Marriage,
    AdminInteract,
    Citizen,
    Quest,
    Book,
    Init,
}

impl PacketFamily {
    pub fn from_byte(value: EOByte) -> Self {
        match value {
            1 => Self::Connection,
            2 => Self::Account,
            3 => Self::Character,
            4 => Self::Login,
            5 => Self::Welcome,
            6 => Self::Walk,
            7 => Self::Face,
            8 => Self::Chair,
            9 => Self::Emote,
            11 => Self::Attack,
            12 => Self::Spell,
            13 => Self::Shop,
            14 => Self::Item,
            16 => Self::StatSkill,
            17 => Self::Global,
            18 => Self::Talk,
            19 => Self::Warp,
            21 => Self::Jukebox,
            22 => Self::Players,
            23 => Self::Avatar,
            24 => Self::Party,
            25 => Self::Refresh,
            26 => Self::NPC,
            27 => Self::PlayerRange,
            28 => Self::NPCRange,
            29 => Self::Range,
            30 => Self::Paperdoll,
            31 => Self::Effect,
            32 => Self::Trade,
            33 => Self::Chest,
            34 => Self::Door,
            35 => Self::Message,
            36 => Self::Bank,
            37 => Self::Locker,
            38 => Self::Barber,
            39 => Self::Guild,
            40 => Self::Music,
            41 => Self::Sit,
            42 => Self::Recover,
            43 => Self::Board,
            44 => Self::Cast,
            45 => Self::Arena,
            46 => Self::Priest,
            47 => Self::Marriage,
            48 => Self::AdminInteract,
            49 => Self::Citizen,
            50 => Self::Quest,
            51 => Self::Book,
            255 => Self::Init,
            _ => {
                warn!("Invalid value for enum PacketFamily: {}", value);
                Self::Connection
            }
        }
    }

    pub fn to_byte(self) -> EOByte {
        match self {
            Self::Connection => 1,
            Self::Account => 2,
            Self::Character => 3,
            Self::Login => 4,
            Self::Welcome => 5,
            Self::Walk => 6,
            Self::Face => 7,
            Self::Chair => 8,
            Self::Emote => 9,
            Self::Attack => 11,
            Self::Spell => 12,
            Self::Shop => 13,
            Self::Item => 14,
            Self::StatSkill => 16,
            Self::Global => 17,
            Self::Talk => 18,
            Self::Warp => 19,
            Self::Jukebox => 21,
            Self::Players => 22,
            Self::Avatar => 23,
            Self::Party => 24,
            Self::Refresh => 25,
            Self::NPC => 26,
            Self::PlayerRange => 27,
            Self::NPCRange => 28,
            Self::Range => 29,
            Self::Paperdoll => 30,
            Self::Effect => 31,
            Self::Trade => 32,
            Self::Chest => 33,
            Self::Door => 34,
            Self::Message => 35,
            Self::Bank => 36,
            Self::Locker => 37,
            Self::Barber => 38,
            Self::Guild => 39,
            Self::Music => 40,
            Self::Sit => 41,
            Self::Recover => 42,
            Self::Board => 43,
            Self::Cast => 44,
            Self::Arena => 45,
            Self::Priest => 46,
            Self::Marriage => 47,
            Self::AdminInteract => 48,
            Self::Citizen => 49,
            Self::Quest => 50,
            Self::Book => 51,
            Self::Init => 255,
        }
    }
}

impl Default for PacketFamily {
    fn default() -> Self {
        PacketFamily::Connection
    }
}

pub const PACKET_ACTION_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketAction {
    Request,
    Accept,
    Reply,
    Remove,
    Agree,
    Create,
    Add,
    Player,
    Take,
    Use,
    Buy,
    Sell,
    Open,
    Close,
    Msg,
    Spec,
    Admin,
    List,
    Tell,
    Report,
    Announce,
    Server,
    Drop,
    Junk,
    Obtain,
    Get,
    Kick,
    Rank,
    TargetSelf,
    TargetOther,
    TargetGroup,
    Dialog,
    Ping,
    Pong,
    Net3,
    Init,
}

impl PacketAction {
    pub fn from_byte(value: EOByte) -> Self {
        match value {
            1 => Self::Request,
            2 => Self::Accept,
            3 => Self::Reply,
            4 => Self::Remove,
            5 => Self::Agree,
            6 => Self::Create,
            7 => Self::Add,
            8 => Self::Player,
            9 => Self::Take,
            10 => Self::Use,
            11 => Self::Buy,
            12 => Self::Sell,
            13 => Self::Open,
            14 => Self::Close,
            15 => Self::Msg,
            16 => Self::Spec,
            17 => Self::Admin,
            18 => Self::List,
            20 => Self::Tell,
            21 => Self::Report,
            22 => Self::Announce,
            23 => Self::Server,
            24 => Self::Drop,
            25 => Self::Junk,
            26 => Self::Obtain,
            27 => Self::Get,
            28 => Self::Kick,
            29 => Self::Rank,
            30 => Self::TargetSelf,
            31 => Self::TargetOther,
            33 => Self::TargetGroup,
            34 => Self::Dialog,
            240 => Self::Ping,
            241 => Self::Pong,
            242 => Self::Net3,
            255 => Self::Init,
            _ => {
                warn!("Invalid value for enum PacketAction: {}", value);
                Self::Request
            }
        }
    }

    pub fn to_byte(self) -> EOByte {
        match self {
            Self::Request => 1,
            Self::Accept => 2,
            Self::Reply => 3,
            Self::Remove => 4,
            Self::Agree => 5,
            Self::Create => 6,
            Self::Add => 7,
            Self::Player => 8,
            Self::Take => 9,
            Self::Use => 10,
            Self::Buy => 11,
            Self::Sell => 12,
            Self::Open => 13,
            Self::Close => 14,
            Self::Msg => 15,
            Self::Spec => 16,
            Self::Admin => 17,
            Self::List => 18,
            Self::Tell => 20,
            Self::Report => 21,
            Self::Announce => 22,
            Self::Server => 23,
            Self::Drop => 24,
            Self::Junk => 25,
            Self::Obtain => 26,
            Self::Get => 27,
            Self::Kick => 28,
            Self::Rank => 29,
            Self::TargetSelf => 30,
            Self::TargetOther => 31,
            Self::TargetGroup => 33,
            Self::Dialog => 34,
            Self::Ping => 240,
            Self::Pong => 241,
            Self::Net3 => 242,
            Self::Init => 255,
        }
    }
}

impl Default for PacketAction {
    fn default() -> Self {
        PacketAction::Request
    }
}

pub const ADMIN_LEVEL_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminLevel {
    Player,
    Guide,
    Guardian,
    GM,
    HGM,
}

impl AdminLevel {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::Player,
            1 => Self::Guide,
            2 => Self::Guardian,
            3 => Self::GM,
            4 => Self::HGM,
            _ => {
                warn!("Invalid value for enum AdminLevel: {}", value);
                Self::Player
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Player => 0,
            Self::Guide => 1,
            Self::Guardian => 2,
            Self::GM => 3,
            Self::HGM => 4,
        }
    }
}

impl Default for AdminLevel {
    fn default() -> Self {
        AdminLevel::Player
    }
}

pub const DIRECTION_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Down,
    Left,
    Up,
    Right,
}

impl Direction {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::Down,
            1 => Self::Left,
            2 => Self::Up,
            3 => Self::Right,
            _ => {
                warn!("Invalid value for enum Direction: {}", value);
                Self::Down
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Down => 0,
            Self::Left => 1,
            Self::Up => 2,
            Self::Right => 3,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}

pub const EMOTE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emote {
    Happy,
    Depressed,
    Sad,
    Angry,
    Confused,
    Surprised,
    Hearts,
    Moon,
    Suicidal,
    Embarrassed,
    Drunk,
    Trade,
    LevelUp,
    Playful,
}

impl Emote {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Happy,
            2 => Self::Depressed,
            3 => Self::Sad,
            4 => Self::Angry,
            5 => Self::Confused,
            6 => Self::Surprised,
            7 => Self::Hearts,
            8 => Self::Moon,
            9 => Self::Suicidal,
            10 => Self::Embarrassed,
            11 => Self::Drunk,
            12 => Self::Trade,
            13 => Self::LevelUp,
            14 => Self::Playful,
            _ => {
                warn!("Invalid value for enum Emote: {}", value);
                Self::Happy
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Happy => 1,
            Self::Depressed => 2,
            Self::Sad => 3,
            Self::Angry => 4,
            Self::Confused => 5,
            Self::Surprised => 6,
            Self::Hearts => 7,
            Self::Moon => 8,
            Self::Suicidal => 9,
            Self::Embarrassed => 10,
            Self::Drunk => 11,
            Self::Trade => 12,
            Self::LevelUp => 13,
            Self::Playful => 14,
        }
    }
}

impl Default for Emote {
    fn default() -> Self {
        Emote::Happy
    }
}

pub const QUEST_PAGE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestPage {
    Progress,
    History,
}

impl QuestPage {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Progress,
            2 => Self::History,
            _ => {
                warn!("Invalid value for enum QuestPage: {}", value);
                Self::Progress
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Progress => 1,
            Self::History => 2,
        }
    }
}

impl Default for QuestPage {
    fn default() -> Self {
        QuestPage::Progress
    }
}

pub const GENDER_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Female,
    Male,
}

impl Gender {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::Female,
            1 => Self::Male,
            _ => {
                warn!("Invalid value for enum Gender: {}", value);
                Self::Female
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Female => 0,
            Self::Male => 1,
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Female
    }
}

pub const SKIN_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skin {
    White,
    Tan,
    Yellow,
    Orc,
    Skeleton,
    Panda,
    Fish,
}

impl Skin {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::White,
            1 => Self::Tan,
            2 => Self::Yellow,
            3 => Self::Orc,
            4 => Self::Skeleton,
            5 => Self::Panda,
            6 => Self::Fish,
            _ => {
                warn!("Invalid value for enum Skin: {}", value);
                Self::White
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::White => 0,
            Self::Tan => 1,
            Self::Yellow => 2,
            Self::Orc => 3,
            Self::Skeleton => 4,
            Self::Panda => 5,
            Self::Fish => 6,
        }
    }
}

impl Default for Skin {
    fn default() -> Self {
        Skin::White
    }
}

pub const PAPERDOLL_ICON_SIZE: usize = 1;

/// "Shows on paperdoll and online player list. Official server/client doesn't support Guide or Guardian icons"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperdollIcon {
    Player,
    GM,
    HGM,
    Party,
    GMParty,
    HGMParty,
}

impl PaperdollIcon {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Player,
            4 => Self::GM,
            5 => Self::HGM,
            6 => Self::Party,
            9 => Self::GMParty,
            10 => Self::HGMParty,
            _ => {
                warn!("Invalid value for enum PaperdollIcon: {}", value);
                Self::Player
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Player => 1,
            Self::GM => 4,
            Self::HGM => 5,
            Self::Party => 6,
            Self::GMParty => 9,
            Self::HGMParty => 10,
        }
    }
}

impl Default for PaperdollIcon {
    fn default() -> Self {
        PaperdollIcon::Player
    }
}

pub const AVATAR_SLOT_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSlot {
    Clothes,
    Hair,
    HairColor,
}

impl AvatarSlot {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Clothes,
            2 => Self::Hair,
            3 => Self::HairColor,
            _ => {
                warn!("Invalid value for enum AvatarSlot: {}", value);
                Self::Clothes
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Clothes => 1,
            Self::Hair => 2,
            Self::HairColor => 3,
        }
    }
}

impl Default for AvatarSlot {
    fn default() -> Self {
        AvatarSlot::Clothes
    }
}

pub const TALK_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TalkReply {
    NotFound,
}

impl TalkReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::NotFound,
            _ => {
                warn!("Invalid value for enum TalkReply: {}", value);
                Self::NotFound
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::NotFound => 1,
        }
    }
}

impl Default for TalkReply {
    fn default() -> Self {
        TalkReply::NotFound
    }
}

pub const SIT_STATE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SitState {
    Stand,
    Chair,
    Floor,
}

impl SitState {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::Stand,
            1 => Self::Chair,
            2 => Self::Floor,
            _ => {
                warn!("Invalid value for enum SitState: {}", value);
                Self::Stand
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Stand => 0,
            Self::Chair => 1,
            Self::Floor => 2,
        }
    }
}

impl Default for SitState {
    fn default() -> Self {
        SitState::Stand
    }
}

pub const SIT_ACTION_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SitAction {
    Sit,
    Stand,
}

impl SitAction {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Sit,
            2 => Self::Stand,
            _ => {
                warn!("Invalid value for enum SitAction: {}", value);
                Self::Sit
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Sit => 1,
            Self::Stand => 2,
        }
    }
}

impl Default for SitAction {
    fn default() -> Self {
        SitAction::Sit
    }
}

pub const TRAIN_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainType {
    Stat,
    Skill,
}

impl TrainType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Stat,
            2 => Self::Skill,
            _ => {
                warn!("Invalid value for enum TrainType: {}", value);
                Self::Stat
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Stat => 1,
            Self::Skill => 2,
        }
    }
}

impl Default for TrainType {
    fn default() -> Self {
        TrainType::Stat
    }
}

pub const BOOK_ICON_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookIcon {
    Item,
    Talk,
    Kill,
    Step,
}

impl BookIcon {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            3 => Self::Item,
            5 => Self::Talk,
            8 => Self::Kill,
            10 => Self::Step,
            _ => {
                warn!("Invalid value for enum BookIcon: {}", value);
                Self::Item
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::Item => 3,
            Self::Talk => 5,
            Self::Kill => 8,
            Self::Step => 10,
        }
    }
}

impl Default for BookIcon {
    fn default() -> Self {
        BookIcon::Item
    }
}

pub const DIALOG_ENTRY_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogEntryType {
    Text,
    Link,
}

impl DialogEntryType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Text,
            2 => Self::Link,
            _ => {
                warn!("Invalid value for enum DialogEntryType: {}", value);
                Self::Text
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Text => 1,
            Self::Link => 2,
        }
    }
}

impl Default for DialogEntryType {
    fn default() -> Self {
        DialogEntryType::Text
    }
}

pub const DIALOG_REPLY_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogReply {
    OK,
    Link,
}

impl DialogReply {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::OK,
            2 => Self::Link,
            _ => {
                warn!("Invalid value for enum DialogReply: {}", value);
                Self::OK
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::OK => 1,
            Self::Link => 2,
        }
    }
}

impl Default for DialogReply {
    fn default() -> Self {
        DialogReply::OK
    }
}

pub const INIT_REPLY_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitReply {
    OutOfDate,
    OK,
    Banned,
    WarpFileEMF,
    FileEMF,
    FileEIF,
    FileENF,
    FileESF,
    Players,
    MapMutation,
    FriendListPlayers,
    FileECF,
}

impl InitReply {
    pub fn from_byte(value: EOByte) -> Self {
        match value {
            1 => Self::OutOfDate,
            2 => Self::OK,
            3 => Self::Banned,
            4 => Self::WarpFileEMF,
            5 => Self::FileEMF,
            6 => Self::FileEIF,
            7 => Self::FileENF,
            8 => Self::FileESF,
            9 => Self::Players,
            10 => Self::MapMutation,
            11 => Self::FriendListPlayers,
            12 => Self::FileECF,
            _ => {
                warn!("Invalid value for enum InitReply: {}", value);
                Self::OutOfDate
            }
        }
    }

    pub fn to_byte(self) -> EOByte {
        match self {
            Self::OutOfDate => 1,
            Self::OK => 2,
            Self::Banned => 3,
            Self::WarpFileEMF => 4,
            Self::FileEMF => 5,
            Self::FileEIF => 6,
            Self::FileENF => 7,
            Self::FileESF => 8,
            Self::Players => 9,
            Self::MapMutation => 10,
            Self::FriendListPlayers => 11,
            Self::FileECF => 12,
        }
    }
}

impl Default for InitReply {
    fn default() -> Self {
        InitReply::OutOfDate
    }
}

pub const INIT_BAN_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitBanType {
    Temp0,
    Temp,
    Perm,
}

impl InitBanType {
    pub fn from_byte(value: EOByte) -> Self {
        match value {
            0 => Self::Temp0,
            1 => Self::Temp,
            2 => Self::Perm,
            _ => {
                warn!("Invalid value for enum InitBanType: {}", value);
                Self::Temp0
            }
        }
    }

    pub fn to_byte(self) -> EOByte {
        match self {
            Self::Temp0 => 0,
            Self::Temp => 1,
            Self::Perm => 2,
        }
    }
}

impl Default for InitBanType {
    fn default() -> Self {
        InitBanType::Temp0
    }
}

pub const FILE_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Map,
    Item,
    NPC,
    Spell,
    Class,
}

impl FileType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Map,
            2 => Self::Item,
            3 => Self::NPC,
            4 => Self::Spell,
            5 => Self::Class,
            _ => {
                warn!("Invalid value for enum FileType: {}", value);
                Self::Map
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Map => 1,
            Self::Item => 2,
            Self::NPC => 3,
            Self::Spell => 4,
            Self::Class => 5,
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Map
    }
}

pub const GUILD_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildReply {
    Busy,
    NotApproved,
    AlreadyMember,
    NoCandidates,
    Exists,
    CreateBegin,
    CreateAddConfirm,
    CreateAdd,
    RecruiterOffline,
    RecruiterNotHere,
    RecruiterWrongGuild,
    NotRecruiter,
    JoinRequest,
    NotPresent,
    AccountLow,
    Accepted,
    NotFound,
    Updated,
    RanksUpdated,
    RemoveLeader,
    RemoveNotMember,
    Removed,
    RankingLeader,
    RankingNotMember,
}

impl GuildReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::Busy,
            2 => Self::NotApproved,
            3 => Self::AlreadyMember,
            4 => Self::NoCandidates,
            5 => Self::Exists,
            6 => Self::CreateBegin,
            7 => Self::CreateAddConfirm,
            8 => Self::CreateAdd,
            9 => Self::RecruiterOffline,
            10 => Self::RecruiterNotHere,
            11 => Self::RecruiterWrongGuild,
            12 => Self::NotRecruiter,
            13 => Self::JoinRequest,
            14 => Self::NotPresent,
            15 => Self::AccountLow,
            16 => Self::Accepted,
            17 => Self::NotFound,
            18 => Self::Updated,
            19 => Self::RanksUpdated,
            20 => Self::RemoveLeader,
            21 => Self::RemoveNotMember,
            22 => Self::Removed,
            23 => Self::RankingLeader,
            24 => Self::RankingNotMember,
            _ => {
                warn!("Invalid value for enum GuildReply: {}", value);
                Self::Busy
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::Busy => 1,
            Self::NotApproved => 2,
            Self::AlreadyMember => 3,
            Self::NoCandidates => 4,
            Self::Exists => 5,
            Self::CreateBegin => 6,
            Self::CreateAddConfirm => 7,
            Self::CreateAdd => 8,
            Self::RecruiterOffline => 9,
            Self::RecruiterNotHere => 10,
            Self::RecruiterWrongGuild => 11,
            Self::NotRecruiter => 12,
            Self::JoinRequest => 13,
            Self::NotPresent => 14,
            Self::AccountLow => 15,
            Self::Accepted => 16,
            Self::NotFound => 17,
            Self::Updated => 18,
            Self::RanksUpdated => 19,
            Self::RemoveLeader => 20,
            Self::RemoveNotMember => 21,
            Self::Removed => 22,
            Self::RankingLeader => 23,
            Self::RankingNotMember => 24,
        }
    }
}

impl Default for GuildReply {
    fn default() -> Self {
        GuildReply::Busy
    }
}

pub const GUILD_INFO_TYPE_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuildInfoType {
    Description,
    Ranks,
    Bank,
}

impl GuildInfoType {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::Description,
            2 => Self::Ranks,
            3 => Self::Bank,
            _ => {
                warn!("Invalid value for enum GuildInfoType: {}", value);
                Self::Description
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::Description => 1,
            Self::Ranks => 2,
            Self::Bank => 3,
        }
    }
}

impl Default for GuildInfoType {
    fn default() -> Self {
        GuildInfoType::Description
    }
}

pub const MAP_EFFECT_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapEffect {
    Quake,
}

impl MapEffect {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Quake,
            _ => {
                warn!("Invalid value for enum MapEffect: {}", value);
                Self::Quake
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Quake => 1,
        }
    }
}

impl Default for MapEffect {
    fn default() -> Self {
        MapEffect::Quake
    }
}

pub const INN_UNSUBSCRIBE_REPLY_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnUnsubscribeReply {
    NotCitizen,
    Unsubscribed,
}

impl InnUnsubscribeReply {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::NotCitizen,
            1 => Self::Unsubscribed,
            _ => {
                warn!("Invalid value for enum InnUnsubscribeReply: {}", value);
                Self::NotCitizen
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::NotCitizen => 0,
            Self::Unsubscribed => 1,
        }
    }
}

impl Default for InnUnsubscribeReply {
    fn default() -> Self {
        InnUnsubscribeReply::NotCitizen
    }
}

pub const SKILL_MASTER_REPLY_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillMasterReply {
    RemoveItems,
    WrongClass,
}

impl SkillMasterReply {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::RemoveItems,
            2 => Self::WrongClass,
            _ => {
                warn!("Invalid value for enum SkillMasterReply: {}", value);
                Self::RemoveItems
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::RemoveItems => 1,
            Self::WrongClass => 2,
        }
    }
}

impl Default for SkillMasterReply {
    fn default() -> Self {
        SkillMasterReply::RemoveItems
    }
}

pub const PARTY_REQUEST_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyRequestType {
    Join,
    Invite,
}

impl PartyRequestType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::Join,
            1 => Self::Invite,
            _ => {
                warn!("Invalid value for enum PartyRequestType: {}", value);
                Self::Join
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Join => 0,
            Self::Invite => 1,
        }
    }
}

impl Default for PartyRequestType {
    fn default() -> Self {
        PartyRequestType::Join
    }
}

pub const CHARACTER_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterReply {
    Exists,
    Full,
    Full3,
    NotApproved,
    OK,
    Deleted,
    SessionId(EOShort),
}

impl CharacterReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::Exists,
            2 => Self::Full,
            3 => Self::Full3,
            4 => Self::NotApproved,
            5 => Self::OK,
            6 => Self::Deleted,
            _ => Self::SessionId(value),
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::Exists => 1,
            Self::Full => 2,
            Self::Full3 => 3,
            Self::NotApproved => 4,
            Self::OK => 5,
            Self::Deleted => 6,
            Self::SessionId(value) => value,
        }
    }
}

impl Default for CharacterReply {
    fn default() -> Self {
        CharacterReply::Exists
    }
}

pub const ACCOUNT_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountReply {
    Exists,
    NotApproved,
    Created,
    ChangeFailed,
    Changed,
    RequestDenied,
    Reserved8,
    Reserved9,
    SessionId(EOShort),
}

impl AccountReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::Exists,
            2 => Self::NotApproved,
            3 => Self::Created,
            5 => Self::ChangeFailed,
            6 => Self::Changed,
            7 => Self::RequestDenied,
            8 => Self::Reserved8,
            9 => Self::Reserved9,
            _ => Self::SessionId(value),
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::Exists => 1,
            Self::NotApproved => 2,
            Self::Created => 3,
            Self::ChangeFailed => 5,
            Self::Changed => 6,
            Self::RequestDenied => 7,
            Self::Reserved8 => 8,
            Self::Reserved9 => 9,
            Self::SessionId(value) => value,
        }
    }
}

impl Default for AccountReply {
    fn default() -> Self {
        AccountReply::Exists
    }
}

pub const LOGIN_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginReply {
    WrongUser,
    WrongUserPass,
    OK,
    Banned,
    LoggedIn,
    Busy,
}

impl LoginReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::WrongUser,
            2 => Self::WrongUserPass,
            3 => Self::OK,
            4 => Self::Banned,
            5 => Self::LoggedIn,
            6 => Self::Busy,
            _ => {
                warn!("Invalid value for enum LoginReply: {}", value);
                Self::WrongUser
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::WrongUser => 1,
            Self::WrongUserPass => 2,
            Self::OK => 3,
            Self::Banned => 4,
            Self::LoggedIn => 5,
            Self::Busy => 6,
        }
    }
}

impl Default for LoginReply {
    fn default() -> Self {
        LoginReply::WrongUser
    }
}

pub const WARP_ANIMATION_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarpAnimation {
    Scroll,
    Admin,
}

impl WarpAnimation {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Scroll,
            2 => Self::Admin,
            _ => {
                warn!("Invalid value for enum WarpAnimation: {}", value);
                Self::Scroll
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Scroll => 1,
            Self::Admin => 2,
        }
    }
}

impl Default for WarpAnimation {
    fn default() -> Self {
        WarpAnimation::Scroll
    }
}

pub const WARP_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarpType {
    Local,
    MapSwitch,
}

impl WarpType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Local,
            2 => Self::MapSwitch,
            _ => {
                warn!("Invalid value for enum WarpType: {}", value);
                Self::Local
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Local => 1,
            Self::MapSwitch => 2,
        }
    }
}

impl Default for WarpType {
    fn default() -> Self {
        WarpType::Local
    }
}

pub const ITEM_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Static,
    Money,
    Heal,
    Teleport,
    Spell,
    EXPReward,
    StatReward,
    SkillReward,
    Key,
    Weapon,
    Shield,
    Armor,
    Hat,
    Boots,
    Gloves,
    Accessory,
    Belt,
    Necklace,
    Ring,
    Armlet,
    Bracer,
    Beer,
    EffectPotion,
    HairDye,
    CureCurse,
}

impl ItemType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Static,
            3 => Self::Money,
            4 => Self::Heal,
            5 => Self::Teleport,
            6 => Self::Spell,
            7 => Self::EXPReward,
            8 => Self::StatReward,
            9 => Self::SkillReward,
            10 => Self::Key,
            11 => Self::Weapon,
            12 => Self::Shield,
            13 => Self::Armor,
            14 => Self::Hat,
            15 => Self::Boots,
            16 => Self::Gloves,
            17 => Self::Accessory,
            18 => Self::Belt,
            19 => Self::Necklace,
            20 => Self::Ring,
            21 => Self::Armlet,
            22 => Self::Bracer,
            23 => Self::Beer,
            24 => Self::EffectPotion,
            25 => Self::HairDye,
            26 => Self::CureCurse,
            _ => {
                warn!("Invalid value for enum ItemType: {}", value);
                Self::Static
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Static => 1,
            Self::Money => 3,
            Self::Heal => 4,
            Self::Teleport => 5,
            Self::Spell => 6,
            Self::EXPReward => 7,
            Self::StatReward => 8,
            Self::SkillReward => 9,
            Self::Key => 10,
            Self::Weapon => 11,
            Self::Shield => 12,
            Self::Armor => 13,
            Self::Hat => 14,
            Self::Boots => 15,
            Self::Gloves => 16,
            Self::Accessory => 17,
            Self::Belt => 18,
            Self::Necklace => 19,
            Self::Ring => 20,
            Self::Armlet => 21,
            Self::Bracer => 22,
            Self::Beer => 23,
            Self::EffectPotion => 24,
            Self::HairDye => 25,
            Self::CureCurse => 26,
        }
    }
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Static
    }
}

pub const WELCOME_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WelcomeReply {
    SelectCharacter,
    EnterGame,
    ServerBusy,
    LoggedIn,
}

impl WelcomeReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::SelectCharacter,
            2 => Self::EnterGame,
            3 => Self::ServerBusy,
            4 => Self::LoggedIn,
            _ => {
                warn!("Invalid value for enum WelcomeReply: {}", value);
                Self::SelectCharacter
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::SelectCharacter => 1,
            Self::EnterGame => 2,
            Self::ServerBusy => 3,
            Self::LoggedIn => 4,
        }
    }
}

impl Default for WelcomeReply {
    fn default() -> Self {
        WelcomeReply::SelectCharacter
    }
}

pub const ADMIN_MESSAGE_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminMessageType {
    Message,
    Report,
}

impl AdminMessageType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Message,
            2 => Self::Report,
            _ => {
                warn!("Invalid value for enum AdminMessageType: {}", value);
                Self::Message
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Message => 1,
            Self::Report => 2,
        }
    }
}

impl Default for AdminMessageType {
    fn default() -> Self {
        AdminMessageType::Message
    }
}

pub const STAT_ID_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatId {
    Str,
    Int,
    Wis,
    Agi,
    Con,
    Cha,
}

impl StatId {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::Str,
            2 => Self::Int,
            3 => Self::Wis,
            4 => Self::Agi,
            5 => Self::Con,
            6 => Self::Cha,
            _ => {
                warn!("Invalid value for enum StatId: {}", value);
                Self::Str
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::Str => 1,
            Self::Int => 2,
            Self::Wis => 3,
            Self::Agi => 4,
            Self::Con => 5,
            Self::Cha => 6,
        }
    }
}

impl Default for StatId {
    fn default() -> Self {
        StatId::Str
    }
}

pub const MARRIAGE_REQUEST_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarriageRequestType {
    MarriageApproval,
    Divorce,
}

impl MarriageRequestType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::MarriageApproval,
            2 => Self::Divorce,
            _ => {
                warn!("Invalid value for enum MarriageRequestType: {}", value);
                Self::MarriageApproval
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::MarriageApproval => 1,
            Self::Divorce => 2,
        }
    }
}

impl Default for MarriageRequestType {
    fn default() -> Self {
        MarriageRequestType::MarriageApproval
    }
}

pub const MARRIAGE_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarriageReply {
    AlreadyMarried,
    NotMarried,
    Success,
    NotEnoughGold,
    WrongName,
    ServiceBusy,
    DivorceNotification,
}

impl MarriageReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::AlreadyMarried,
            2 => Self::NotMarried,
            3 => Self::Success,
            4 => Self::NotEnoughGold,
            5 => Self::WrongName,
            6 => Self::ServiceBusy,
            7 => Self::DivorceNotification,
            _ => {
                warn!("Invalid value for enum MarriageReply: {}", value);
                Self::AlreadyMarried
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::AlreadyMarried => 1,
            Self::NotMarried => 2,
            Self::Success => 3,
            Self::NotEnoughGold => 4,
            Self::WrongName => 5,
            Self::ServiceBusy => 6,
            Self::DivorceNotification => 7,
        }
    }
}

impl Default for MarriageReply {
    fn default() -> Self {
        MarriageReply::AlreadyMarried
    }
}

pub const PRIEST_REPLY_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriestReply {
    NotDressed,
    LowLevel,
    PartnerNotPresent,
    PartnerNotDressed,
    Busy,
    DoYou,
    PartnerAlreadyMarried,
    NoPermission,
}

impl PriestReply {
    pub fn from_short(value: EOShort) -> Self {
        match value {
            1 => Self::NotDressed,
            2 => Self::LowLevel,
            3 => Self::PartnerNotPresent,
            4 => Self::PartnerNotDressed,
            5 => Self::Busy,
            6 => Self::DoYou,
            7 => Self::PartnerAlreadyMarried,
            8 => Self::NoPermission,
            _ => {
                warn!("Invalid value for enum PriestReply: {}", value);
                Self::NotDressed
            }
        }
    }

    pub fn to_short(self) -> EOShort {
        match self {
            Self::NotDressed => 1,
            Self::LowLevel => 2,
            Self::PartnerNotPresent => 3,
            Self::PartnerNotDressed => 4,
            Self::Busy => 5,
            Self::DoYou => 6,
            Self::PartnerAlreadyMarried => 7,
            Self::NoPermission => 8,
        }
    }
}

impl Default for PriestReply {
    fn default() -> Self {
        PriestReply::NotDressed
    }
}

pub const PARTY_REPLY_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyReply {
    AlreadyInAnotherParty,
    AlreadyInYourParty,
    PartyIsFull,
}

impl PartyReply {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            0 => Self::AlreadyInAnotherParty,
            1 => Self::AlreadyInYourParty,
            2 => Self::PartyIsFull,
            _ => {
                warn!("Invalid value for enum PartyReply: {}", value);
                Self::AlreadyInAnotherParty
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::AlreadyInAnotherParty => 0,
            Self::AlreadyInYourParty => 1,
            Self::PartyIsFull => 2,
        }
    }
}

impl Default for PartyReply {
    fn default() -> Self {
        PartyReply::AlreadyInAnotherParty
    }
}

pub const SPELL_TARGET_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellTargetType {
    Player,
    NPC,
}

impl SpellTargetType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::Player,
            2 => Self::NPC,
            _ => {
                warn!("Invalid value for enum SpellTargetType: {}", value);
                Self::Player
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::Player => 1,
            Self::NPC => 2,
        }
    }
}

impl Default for SpellTargetType {
    fn default() -> Self {
        SpellTargetType::Player
    }
}

pub const MAP_DAMAGE_TYPE_SIZE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapDamageType {
    TPDrain,
    Spikes,
}

impl MapDamageType {
    pub fn from_char(value: EOChar) -> Self {
        match value {
            1 => Self::TPDrain,
            2 => Self::Spikes,
            _ => {
                warn!("Invalid value for enum MapDamageType: {}", value);
                Self::TPDrain
            }
        }
    }

    pub fn to_char(self) -> EOChar {
        match self {
            Self::TPDrain => 1,
            Self::Spikes => 2,
        }
    }
}

impl Default for MapDamageType {
    fn default() -> Self {
        MapDamageType::TPDrain
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PaperdollBAHWS {
    pub boots: EOShort,
    pub armor: EOShort,
    pub hat: EOShort,
    pub weapon: EOShort,
    pub shield: EOShort,
}

impl PaperdollBAHWS {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollBAHWS {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        self.armor = reader.get_short();
        self.hat = reader.get_short();
        self.weapon = reader.get_short();
        self.shield = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.boots);
        builder.add_short(self.armor);
        builder.add_short(self.hat);
        builder.add_short(self.weapon);
        builder.add_short(self.shield);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PaperdollBAHSW {
    pub boots: EOShort,
    pub armor: EOShort,
    pub hat: EOShort,
    pub shield: EOShort,
    pub weapon: EOShort,
}

impl PaperdollBAHSW {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollBAHSW {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        self.armor = reader.get_short();
        self.hat = reader.get_short();
        self.shield = reader.get_short();
        self.weapon = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.boots);
        builder.add_short(self.armor);
        builder.add_short(self.hat);
        builder.add_short(self.shield);
        builder.add_short(self.weapon);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PaperdollB000A0HSW {
    pub boots: EOShort,
    pub armor: EOShort,
    pub hat: EOShort,
    pub shield: EOShort,
    pub weapon: EOShort,
}

impl PaperdollB000A0HSW {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollB000A0HSW {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        reader.get_short();
        reader.get_short();
        reader.get_short();
        self.armor = reader.get_short();
        reader.get_short();
        self.hat = reader.get_short();
        self.shield = reader.get_short();
        self.weapon = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.boots);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(self.armor);
        builder.add_short(0);
        builder.add_short(self.hat);
        builder.add_short(self.shield);
        builder.add_short(self.weapon);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PaperdollFull {
    pub boots: EOShort,
    pub accessory: EOShort,
    pub gloves: EOShort,
    pub belt: EOShort,
    pub armor: EOShort,
    pub necklace: EOShort,
    pub hat: EOShort,
    pub shield: EOShort,
    pub weapon: EOShort,
    pub ring: [EOShort; 2],
    pub armlet: [EOShort; 2],
    pub bracer: [EOShort; 2],
}

impl PaperdollFull {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollFull {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.boots = reader.get_short();
        self.accessory = reader.get_short();
        self.gloves = reader.get_short();
        self.belt = reader.get_short();
        self.armor = reader.get_short();
        self.necklace = reader.get_short();
        self.hat = reader.get_short();
        self.shield = reader.get_short();
        self.weapon = reader.get_short();
        for i in 0..2 {
            self.ring[i] = reader.get_short();
        }
        for i in 0..2 {
            self.armlet[i] = reader.get_short();
        }
        for i in 0..2 {
            self.bracer[i] = reader.get_short();
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.boots);
        builder.add_short(self.accessory);
        builder.add_short(self.gloves);
        builder.add_short(self.belt);
        builder.add_short(self.armor);
        builder.add_short(self.necklace);
        builder.add_short(self.hat);
        builder.add_short(self.shield);
        builder.add_short(self.weapon);
        for i in 0..self.ring.len() {
            builder.add_short(self.ring[i]);
        }
        for i in 0..self.armlet.len() {
            builder.add_short(self.armlet[i]);
        }
        for i in 0..self.bracer.len() {
            builder.add_short(self.bracer[i]);
        }
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TinyCoords {
    pub x: EOChar,
    pub y: EOChar,
}

impl TinyCoords {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for TinyCoords {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.x = reader.get_char();
        self.y = reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.x);
        builder.add_char(self.y);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Coords {
    pub x: EOShort,
    pub y: EOShort,
}

impl Coords {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Coords {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.x = reader.get_short();
        self.y = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.x);
        builder.add_short(self.y);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Weight {
    pub current: EOChar,
    pub max: EOChar,
}

impl Weight {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Weight {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.current = reader.get_char();
        self.max = reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.current);
        builder.add_char(self.max);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
        let mut builder = StreamBuilder::new();
        builder.add_short(self.id);
        builder.add_int(self.amount);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
        let mut builder = StreamBuilder::new();
        builder.add_int(self.amount);
        builder.add_short(self.id);
        builder.get()
    }
}

/// Used for shops, lockers, and various item transfers
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
        let mut builder = StreamBuilder::new();
        builder.add_short(self.id);
        builder.add_three(self.amount);
        builder.get()
    }
}

/// Used for craft ingredients
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
        let mut builder = StreamBuilder::new();
        builder.add_short(self.id);
        builder.add_char(self.amount);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Spell {
    pub id: EOShort,
    pub level: EOShort,
}

impl Spell {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Spell {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.id = reader.get_short();
        self.level = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.id);
        builder.add_short(self.level);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterMapInfo {
    pub name: String,
    pub id: EOShort,
    pub map_id: EOShort,
    pub coords: Coords,
    pub direction: Direction,
    pub class_id: EOChar,
    pub guild_tag: String,
    pub level: EOChar,
    pub gender: EOChar,
    pub hairstyle: EOChar,
    pub haircolor: EOChar,
    pub skin_id: Skin,
    pub max_hp: EOShort,
    pub hp: EOShort,
    pub max_tp: EOShort,
    pub tp: EOShort,
    pub paperdoll: PaperdollB000A0HSW,
    pub sit_state: SitState,
    pub invisible: EOChar,
    pub animation: WarpAnimation,
}

impl CharacterMapInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterMapInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.id = reader.get_short();
        self.map_id = reader.get_short();
        self.coords.deserialize(&reader);
        self.direction = Direction::from_char(reader.get_char());
        self.class_id = reader.get_char();
        self.guild_tag = reader.get_fixed_string(3 as usize);
        self.level = reader.get_char();
        self.gender = reader.get_char();
        self.hairstyle = reader.get_char();
        self.haircolor = reader.get_char();
        self.skin_id = Skin::from_char(reader.get_char());
        self.max_hp = reader.get_short();
        self.hp = reader.get_short();
        self.max_tp = reader.get_short();
        self.tp = reader.get_short();
        self.paperdoll.deserialize(&reader);
        self.sit_state = SitState::from_char(reader.get_char());
        self.invisible = reader.get_char();
        self.animation = WarpAnimation::from_char(reader.get_char());
        reader.get_byte();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_break_string(&self.name);
        builder.add_short(self.id);
        builder.add_short(self.map_id);
        builder.append(&mut self.coords.serialize());
        builder.add_char(self.direction.to_char());
        builder.add_char(self.class_id);
        builder.add_fixed_string(&self.guild_tag, 3 as usize);
        builder.add_char(self.level);
        builder.add_char(self.gender);
        builder.add_char(self.hairstyle);
        builder.add_char(self.haircolor);
        builder.add_char(self.skin_id.to_char());
        builder.add_short(self.max_hp);
        builder.add_short(self.hp);
        builder.add_short(self.max_tp);
        builder.add_short(self.tp);
        builder.append(&mut self.paperdoll.serialize());
        builder.add_char(self.sit_state.to_char());
        builder.add_char(self.invisible);
        builder.add_char(self.animation.to_char());
        builder.add_byte(EO_BREAK_CHAR);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NPCMapInfo {
    pub index: EOChar,
    pub id: EOShort,
    pub coords: TinyCoords,
    pub direction: Direction,
}

impl NPCMapInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NPCMapInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.index = reader.get_char();
        self.id = reader.get_short();
        self.coords.deserialize(&reader);
        self.direction = Direction::from_char(reader.get_char());
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.index);
        builder.add_short(self.id);
        builder.append(&mut self.coords.serialize());
        builder.add_char(self.direction.to_char());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ItemMapInfo {
    pub uid: EOShort,
    pub id: EOShort,
    pub coords: TinyCoords,
    pub amount: EOThree,
}

impl ItemMapInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ItemMapInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.uid = reader.get_short();
        self.id = reader.get_short();
        self.coords.deserialize(&reader);
        self.amount = reader.get_three();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.uid);
        builder.add_short(self.id);
        builder.append(&mut self.coords.serialize());
        builder.add_three(self.amount);
        builder.get()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AvatarChangeData {
    Clothes(AvatarChangeClothes),
    Hair(AvatarChangeHair),
    HairColor(AvatarChangeHairColor),
}

impl Default for AvatarChangeData {
    fn default() -> Self {
        Self::Clothes(AvatarChangeClothes::default())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AvatarChange {
    pub player_id: EOShort,
    pub slot: AvatarSlot,
    pub sound: EOChar,
    pub data: AvatarChangeData,
}

impl AvatarChange {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for AvatarChange {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.slot = AvatarSlot::from_char(reader.get_char());
        self.sound = reader.get_char();
        match self.slot {
            AvatarSlot::Clothes => {
                let mut clothes = AvatarChangeClothes::new();
                clothes.deserialize(&reader);
                self.data = AvatarChangeData::Clothes(clothes);
            }
            AvatarSlot::Hair => {
                let mut hair = AvatarChangeHair::new();
                hair.deserialize(&reader);
                self.data = AvatarChangeData::Hair(hair);
            }
            AvatarSlot::HairColor => {
                let mut hair_color = AvatarChangeHairColor::new();
                hair_color.deserialize(&reader);
                self.data = AvatarChangeData::HairColor(hair_color);
            }
            _ => {}
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.player_id);
        builder.add_char(self.slot.to_char());
        builder.add_char(self.sound);
        match &self.data {
            AvatarChangeData::Clothes(clothes) => {
                builder.append(&mut clothes.serialize());
            }
            AvatarChangeData::Hair(hair) => {
                builder.append(&mut hair.serialize());
            }
            AvatarChangeData::HairColor(hair_color) => {
                builder.append(&mut hair_color.serialize());
            }
            _ => {}
        }
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AvatarChangeClothes {
    pub paperdoll: PaperdollBAHWS,
}

impl AvatarChangeClothes {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for AvatarChangeClothes {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.paperdoll.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.append(&mut self.paperdoll.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AvatarChangeHair {
    pub style: EOChar,
    pub color: EOChar,
}

impl AvatarChangeHair {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for AvatarChangeHair {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.style = reader.get_char();
        self.color = reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.style);
        builder.add_char(self.color);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AvatarChangeHairColor {
    pub color: EOChar,
}

impl AvatarChangeHairColor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for AvatarChangeHairColor {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.color = reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.color);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NearbyInfo {
    pub num_characters: EOChar,
    pub characters: Vec<CharacterMapInfo>,
    pub npcs: Vec<NPCMapInfo>,
    pub items: Vec<ItemMapInfo>,
}

impl NearbyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NearbyInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.num_characters = reader.get_char();
        reader.get_byte();
        for _ in 0..self.num_characters {
            let mut character_map_info = CharacterMapInfo::new();
            character_map_info.deserialize(&reader);
            self.characters.push(character_map_info);
        }
        while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
            let mut npc_map_info = NPCMapInfo::new();
            npc_map_info.deserialize(&reader);
            self.npcs.push(npc_map_info);
        }
        reader.get_byte();
        while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
            let mut item_map_info = ItemMapInfo::new();
            item_map_info.deserialize(&reader);
            self.items.push(item_map_info);
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.num_characters);
        builder.add_byte(EO_BREAK_CHAR);
        for i in 0..self.characters.len() {
            builder.append(&mut self.characters[i].serialize());
        }
        for i in 0..self.npcs.len() {
            builder.append(&mut self.npcs[i].serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        for i in 0..self.items.len() {
            builder.append(&mut self.items[i].serialize());
        }
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct OnlinePlayers {
    pub name: String,
    pub title: String,
    pub icon: PaperdollIcon,
    pub class_id: EOChar,
    pub guild_tag: String,
}

impl OnlinePlayers {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for OnlinePlayers {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.title = reader.get_break_string();
        reader.get_char();
        self.icon = PaperdollIcon::from_char(reader.get_char());
        self.class_id = reader.get_char();
        self.guild_tag = reader.get_fixed_string(3 as usize);
        reader.get_byte();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.title);
        builder.add_char(0);
        builder.add_char(self.icon.to_char());
        builder.add_char(self.class_id);
        builder.add_fixed_string(&self.guild_tag, 3 as usize);
        builder.add_byte(EO_BREAK_CHAR);
        builder.get()
    }
}

/// Character selection screen character info
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterInfo {
    pub name: String,
    pub id: EOInt,
    pub level: EOChar,
    pub gender: Gender,
    pub hairstyle: EOChar,
    pub haircolor: EOChar,
    pub skin: Skin,
    pub admin: AdminLevel,
    pub paperdoll: PaperdollBAHSW,
}

impl CharacterInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.id = reader.get_int();
        self.level = reader.get_char();
        self.gender = Gender::from_char(reader.get_char());
        self.hairstyle = reader.get_char();
        self.haircolor = reader.get_char();
        self.skin = Skin::from_char(reader.get_char());
        self.admin = AdminLevel::from_char(reader.get_char());
        self.paperdoll.deserialize(&reader);
        reader.get_byte();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_break_string(&self.name);
        builder.add_int(self.id);
        builder.add_char(self.level);
        builder.add_char(self.gender.to_char());
        builder.add_char(self.hairstyle);
        builder.add_char(self.haircolor);
        builder.add_char(self.skin.to_char());
        builder.add_char(self.admin.to_char());
        builder.append(&mut self.paperdoll.serialize());
        builder.add_byte(EO_BREAK_CHAR);
        builder.get()
    }
}

/// Character selection screen character info
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterList {
    pub num_characters: EOChar,
    pub characters: Vec<CharacterInfo>,
}

impl CharacterList {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterList {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.num_characters = reader.get_char();
        reader.get_char();
        reader.get_byte();
        for _ in 0..self.num_characters {
            let mut character_info = CharacterInfo::new();
            character_info.deserialize(&reader);
            self.characters.push(character_info);
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.num_characters);
        builder.add_char(1);
        builder.add_byte(EO_BREAK_CHAR);
        for i in 0..self.characters.len() {
            builder.append(&mut self.characters[i].serialize());
        }
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ServerSettings {
    pub jail_map: EOShort,
    pub rescue_map: EOShort,
    pub rescue_x: EOChar,
    pub rescue_y: EOChar,
    pub light_guide_flood_rate: EOShort,
    pub guardian_flood_rate: EOShort,
    pub gm_flood_rate: EOShort,
    pub hgm_flood_rate: EOShort,
}

impl ServerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ServerSettings {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.jail_map = reader.get_short();
        self.rescue_map = reader.get_short();
        self.rescue_x = reader.get_char();
        self.rescue_y = reader.get_char();
        self.light_guide_flood_rate = reader.get_short();
        self.guardian_flood_rate = reader.get_short();
        self.gm_flood_rate = reader.get_short();
        self.hgm_flood_rate = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.jail_map);
        builder.add_short(self.rescue_map);
        builder.add_char(self.rescue_x);
        builder.add_char(self.rescue_y);
        builder.add_short(self.light_guide_flood_rate);
        builder.add_short(self.guardian_flood_rate);
        builder.add_short(self.gm_flood_rate);
        builder.add_short(self.hgm_flood_rate);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterStats4 {
    pub hp: EOShort,
    pub max_hp: EOShort,
    pub tp: EOShort,
    pub max_tp: EOShort,
    pub base: CharacterBaseStats,
    pub secondary: CharacterSecondaryStats,
}

impl CharacterStats4 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterStats4 {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.hp = reader.get_short();
        self.max_hp = reader.get_short();
        self.tp = reader.get_short();
        self.max_tp = reader.get_short();
        self.base.deserialize(&reader);
        self.secondary.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.hp);
        builder.add_short(self.max_hp);
        builder.add_short(self.tp);
        builder.add_short(self.max_tp);
        builder.append(&mut self.base.serialize());
        builder.append(&mut self.secondary.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GlobalBackfillMessage {
    pub player_name: String,
    pub message: String,
}

impl GlobalBackfillMessage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for GlobalBackfillMessage {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_name = reader.get_break_string();
        self.message = reader.get_break_string();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_break_string(&self.player_name);
        builder.add_break_string(&self.message);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WalkCommon {
    pub direction: Direction,
    pub timestamp: EOThree,
    pub coords: TinyCoords,
}

impl WalkCommon {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for WalkCommon {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.direction = Direction::from_char(reader.get_char());
        self.timestamp = reader.get_three();
        self.coords.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.direction.to_char());
        builder.add_three(self.timestamp);
        builder.append(&mut self.coords.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ShopTradeItem {
    pub item_id: EOShort,
    pub buy_price: EOThree,
    pub sell_price: EOThree,
    pub max_buy_amount: EOChar,
}

impl ShopTradeItem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ShopTradeItem {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.item_id = reader.get_short();
        self.buy_price = reader.get_three();
        self.sell_price = reader.get_three();
        self.max_buy_amount = reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.item_id);
        builder.add_three(self.buy_price);
        builder.add_three(self.sell_price);
        builder.add_char(self.max_buy_amount);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ShopCraftItem {
    pub item_id: EOShort,
    pub ingredients: [VeryShortItem; 4],
}

impl ShopCraftItem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ShopCraftItem {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.item_id = reader.get_short();
        for i in 0..4 {
            self.ingredients[i].deserialize(&reader);
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.item_id);
        for i in 0..self.ingredients.len() {
            builder.append(&mut self.ingredients[i].serialize());
        }
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterBaseStats {
    pub str: EOShort,
    pub intl: EOShort,
    pub wis: EOShort,
    pub agi: EOShort,
    pub con: EOShort,
    pub cha: EOShort,
}

impl CharacterBaseStats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterBaseStats {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.str = reader.get_short();
        self.intl = reader.get_short();
        self.wis = reader.get_short();
        self.agi = reader.get_short();
        self.con = reader.get_short();
        self.cha = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.str);
        builder.add_short(self.intl);
        builder.add_short(self.wis);
        builder.add_short(self.agi);
        builder.add_short(self.con);
        builder.add_short(self.cha);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterSecondaryStats {
    pub mindam: EOShort,
    pub maxdam: EOShort,
    pub accuracy: EOShort,
    pub evade: EOShort,
    pub armor: EOShort,
}

impl CharacterSecondaryStats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterSecondaryStats {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.mindam = reader.get_short();
        self.maxdam = reader.get_short();
        self.accuracy = reader.get_short();
        self.evade = reader.get_short();
        self.armor = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.mindam);
        builder.add_short(self.maxdam);
        builder.add_short(self.accuracy);
        builder.add_short(self.evade);
        builder.add_short(self.armor);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterStats1 {
    pub stat_points: EOShort,
    pub skill_points: EOShort,
    pub hp: EOShort,
    pub max_hp: EOShort,
    pub tp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
    pub base: CharacterBaseStats,
    pub secondary: CharacterSecondaryStats,
}

impl CharacterStats1 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterStats1 {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.stat_points = reader.get_short();
        self.skill_points = reader.get_short();
        self.hp = reader.get_short();
        self.max_hp = reader.get_short();
        self.tp = reader.get_short();
        self.max_tp = reader.get_short();
        self.max_sp = reader.get_short();
        self.base.deserialize(&reader);
        self.secondary.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.stat_points);
        builder.add_short(self.skill_points);
        builder.add_short(self.hp);
        builder.add_short(self.max_hp);
        builder.add_short(self.tp);
        builder.add_short(self.max_tp);
        builder.add_short(self.max_sp);
        builder.append(&mut self.base.serialize());
        builder.append(&mut self.secondary.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterStats2 {
    pub hp: EOShort,
    pub max_hp: EOShort,
    pub tp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
    pub stat_points: EOShort,
    pub skill_points: EOShort,
    pub karma: EOShort,
    pub secondary: CharacterSecondaryStats,
    pub base: CharacterBaseStats,
}

impl CharacterStats2 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterStats2 {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.hp = reader.get_short();
        self.max_hp = reader.get_short();
        self.tp = reader.get_short();
        self.max_tp = reader.get_short();
        self.max_sp = reader.get_short();
        self.stat_points = reader.get_short();
        self.skill_points = reader.get_short();
        self.karma = reader.get_short();
        self.secondary.deserialize(&reader);
        self.base.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.hp);
        builder.add_short(self.max_hp);
        builder.add_short(self.tp);
        builder.add_short(self.max_tp);
        builder.add_short(self.max_sp);
        builder.add_short(self.stat_points);
        builder.add_short(self.skill_points);
        builder.add_short(self.karma);
        builder.append(&mut self.secondary.serialize());
        builder.append(&mut self.base.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharacterStats3 {
    pub base: CharacterBaseStats,
    pub max_hp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
    pub max_weight: EOShort,
    pub secondary: CharacterSecondaryStats,
}

impl CharacterStats3 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterStats3 {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.base.deserialize(&reader);
        self.max_hp = reader.get_short();
        self.max_tp = reader.get_short();
        self.max_sp = reader.get_short();
        self.max_weight = reader.get_short();
        self.secondary.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.append(&mut self.base.serialize());
        builder.add_short(self.max_hp);
        builder.add_short(self.max_tp);
        builder.add_short(self.max_sp);
        builder.add_short(self.max_weight);
        builder.append(&mut self.secondary.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SkillLearn {
    pub id: EOShort,
    pub level_req: EOChar,
    pub class_req: EOChar,
    pub cost: EOInt,
    pub skill_req: [EOShort; 4],
    pub stat_req: CharacterBaseStats,
}

impl SkillLearn {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for SkillLearn {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.id = reader.get_short();
        self.level_req = reader.get_char();
        self.class_req = reader.get_char();
        self.cost = reader.get_int();
        for i in 0..4 {
            self.skill_req[i] = reader.get_short();
        }
        self.stat_req.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.id);
        builder.add_char(self.level_req);
        builder.add_char(self.class_req);
        builder.add_int(self.cost);
        for i in 0..self.skill_req.len() {
            builder.add_short(self.skill_req[i]);
        }
        builder.append(&mut self.stat_req.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ItemCharacterStats {
    pub max_hp: EOShort,
    pub max_tp: EOShort,
    pub base: CharacterBaseStats,
    pub secondary: CharacterSecondaryStats,
}

impl ItemCharacterStats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ItemCharacterStats {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.max_hp = reader.get_short();
        self.max_tp = reader.get_short();
        self.base.deserialize(&reader);
        self.secondary.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.max_hp);
        builder.add_short(self.max_tp);
        builder.append(&mut self.base.serialize());
        builder.append(&mut self.secondary.serialize());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BoardPostListing {
    pub post_id: EOShort,
    pub author: String,
    pub subject: String,
}

impl BoardPostListing {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for BoardPostListing {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.post_id = reader.get_short();
        reader.get_byte();
        self.author = reader.get_break_string();
        self.subject = reader.get_break_string();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.post_id);
        builder.add_byte(EO_BREAK_CHAR);
        builder.add_break_string(&self.author);
        builder.add_break_string(&self.subject);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PaperdollInfo {
    pub name: String,
    pub home: String,
    pub partner: String,
    pub title: String,
    pub guild: String,
    pub guild_rank: String,
    pub player_id: EOShort,
    pub class_id: EOChar,
    pub gender: Gender,
}

impl PaperdollInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PaperdollInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.home = reader.get_break_string();
        self.partner = reader.get_break_string();
        self.title = reader.get_break_string();
        self.guild = reader.get_break_string();
        self.guild_rank = reader.get_break_string();
        self.player_id = reader.get_short();
        self.class_id = reader.get_char();
        self.gender = Gender::from_char(reader.get_char());
        reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.home);
        builder.add_break_string(&self.partner);
        builder.add_break_string(&self.title);
        builder.add_break_string(&self.guild);
        builder.add_break_string(&self.guild_rank);
        builder.add_short(self.player_id);
        builder.add_char(self.class_id);
        builder.add_char(self.gender.to_char());
        builder.add_char(0);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PartyMember {
    pub player_id: EOShort,
    pub leader: EOChar,
    pub level: EOChar,
    pub hp_percentage: EOChar,
    pub name: String,
}

impl PartyMember {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PartyMember {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.leader = reader.get_char();
        self.level = reader.get_char();
        self.hp_percentage = reader.get_char();
        self.name = reader.get_break_string();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.player_id);
        builder.add_char(self.leader);
        builder.add_char(self.level);
        builder.add_char(self.hp_percentage);
        builder.add_break_string(&self.name);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PartyExpShare {
    pub player_id: EOShort,
    pub experience: EOInt,
    /// "A value greater than 0 is 'new level' and indicates the player leveled up."
    pub level_up: EOChar,
}

impl PartyExpShare {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for PartyExpShare {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.experience = reader.get_int();
        self.level_up = reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.player_id);
        builder.add_int(self.experience);
        builder.add_char(self.level_up);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GuildStaff {
    pub rank: EOChar,
    pub name: String,
}

impl GuildStaff {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for GuildStaff {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.rank = reader.get_char();
        reader.get_byte();
        self.name = reader.get_break_string();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.rank);
        builder.add_byte(EO_BREAK_CHAR);
        builder.add_break_string(&self.name);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GuildMember {
    pub rank: EOChar,
    pub name: String,
    pub rank_name: String,
}

impl GuildMember {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for GuildMember {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.rank = reader.get_char();
        reader.get_byte();
        self.name = reader.get_break_string();
        self.rank_name = reader.get_break_string();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.rank);
        builder.add_byte(EO_BREAK_CHAR);
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.rank_name);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GroupHealTargetPlayer {
    pub player_id: EOShort,
    pub hp_percentage: EOChar,
    pub hp: EOShort,
}

impl GroupHealTargetPlayer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for GroupHealTargetPlayer {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.hp_percentage = reader.get_char();
        self.hp = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.player_id);
        builder.add_char(self.hp_percentage);
        builder.add_short(self.hp);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TradeItemData {
    pub player1_id: EOShort,
    pub player1_items: Vec<Item>,
    pub player2_id: EOShort,
    pub player2_items: Vec<Item>,
}

impl TradeItemData {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for TradeItemData {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player1_id = reader.get_short();
        while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
            let mut item = Item::new();
            item.deserialize(&reader);
            self.player1_items.push(item);
        }
        reader.get_byte();
        self.player2_id = reader.get_short();
        while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
            let mut item = Item::new();
            item.deserialize(&reader);
            self.player2_items.push(item);
        }
        reader.get_byte();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.player1_id);
        for i in 0..self.player1_items.len() {
            builder.append(&mut self.player1_items[i].serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        builder.add_short(self.player2_id);
        for i in 0..self.player2_items.len() {
            builder.append(&mut self.player2_items[i].serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LevelUpStats {
    pub level: EOChar,
    pub stat_points: EOShort,
    pub skill_points: EOShort,
    pub max_hp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
}

impl LevelUpStats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for LevelUpStats {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.level = reader.get_char();
        self.stat_points = reader.get_short();
        self.skill_points = reader.get_short();
        self.max_hp = reader.get_short();
        self.max_tp = reader.get_short();
        self.max_sp = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.level);
        builder.add_short(self.stat_points);
        builder.add_short(self.skill_points);
        builder.add_short(self.max_hp);
        builder.add_short(self.max_tp);
        builder.add_short(self.max_sp);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NPCUpdatePos {
    pub npc_index: EOChar,
    pub coords: Coords,
    pub direction: Direction,
}

impl NPCUpdatePos {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NPCUpdatePos {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.npc_index = reader.get_char();
        self.coords.deserialize(&reader);
        self.direction = Direction::from_char(reader.get_char());
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.npc_index);
        builder.append(&mut self.coords.serialize());
        builder.add_char(self.direction.to_char());
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NPCUpdateAttack {
    pub npc_index: EOChar,
    pub killed_state: EOChar,
    pub direction: Direction,
    pub player_id: EOShort,
    pub damage: EOThree,
    pub hp_percentage: EOChar,
}

impl NPCUpdateAttack {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NPCUpdateAttack {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.npc_index = reader.get_char();
        self.killed_state = reader.get_char();
        self.direction = Direction::from_char(reader.get_char());
        self.player_id = reader.get_short();
        self.damage = reader.get_three();
        self.hp_percentage = reader.get_char();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.npc_index);
        builder.add_char(self.killed_state);
        builder.add_char(self.direction.to_char());
        builder.add_short(self.player_id);
        builder.add_three(self.damage);
        builder.add_char(self.hp_percentage);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NPCUpdateChat {
    pub npc_index: EOChar,
    pub message_length: EOChar,
    pub message: String,
}

impl NPCUpdateChat {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NPCUpdateChat {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.npc_index = reader.get_char();
        self.message_length = reader.get_char();
        self.message = reader.get_fixed_string(self.message_length as usize);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.npc_index);
        builder.add_char(self.message_length);
        builder.add_fixed_string(&self.message, self.message_length as usize);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QuestProgressEntry {
    pub name: String,
    pub description: String,
    pub icon: EOShort,
    pub progress: EOShort,
    pub target: EOShort,
}

impl QuestProgressEntry {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for QuestProgressEntry {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.description = reader.get_break_string();
        self.icon = reader.get_short();
        self.progress = reader.get_short();
        self.target = reader.get_short();
        reader.get_byte();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.description);
        builder.add_short(self.icon);
        builder.add_short(self.progress);
        builder.add_short(self.target);
        builder.add_byte(EO_BREAK_CHAR);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DialogQuestEntry {
    pub quest_id: EOShort,
    pub quest_name: String,
}

impl DialogQuestEntry {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for DialogQuestEntry {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.quest_id = reader.get_short();
        self.quest_name = reader.get_break_string();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.quest_id);
        builder.add_break_string(&self.quest_name);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DialogEntry {
    pub entry_type: DialogEntryType,
    pub link_id: EOShort,
    pub line: String,
}

impl DialogEntry {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for DialogEntry {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.entry_type = DialogEntryType::from_char(reader.get_char());
        self.link_id = reader.get_short();
        self.line = reader.get_break_string();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_char(self.entry_type.to_char());
        builder.add_short(self.link_id);
        builder.add_break_string(&self.line);
        builder.get()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MapDrainDamageOther {
    pub player_id: EOShort,
    pub hp_percentage: EOChar,
    pub damage: EOShort,
}

impl MapDrainDamageOther {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for MapDrainDamageOther {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.hp_percentage = reader.get_char();
        self.damage = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_short(self.player_id);
        builder.add_char(self.hp_percentage);
        builder.add_short(self.damage);
        builder.get()
    }
}

pub mod client {
    use super::*;

    pub mod Init {
        use super::super::*;

        /// Initialization request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Init {
            pub challenge: EOThree,
            pub version: [EOChar; 3],
            pub hdid: String,
        }

        impl Init {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Init {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.challenge = reader.get_three();
                for i in 0..3 {
                    self.version[i] = reader.get_char();
                }
                reader.get_char();
                reader.get_char();
                self.hdid = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_three(self.challenge);
                for i in 0..self.version.len() {
                    builder.add_char(self.version[i]);
                }
                builder.add_char(0);
                builder.add_char(0);
                builder.add_string(&self.hdid);
                builder.get()
            }
        }
    }

    pub mod Message {
        use super::super::*;

        /// Requests a list of online players. Was used by WatchDog.exe to generate website pages
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {}

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }

        /// #ping command request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Ping {}

        impl Ping {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Ping {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(2);
                builder.get()
            }
        }
    }

    pub mod Connection {
        use super::super::*;

        /// Confirm initialization data
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub decode_multiple: EOShort,
            pub encode_multiple: EOShort,
            pub player_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.decode_multiple = reader.get_short();
                self.encode_multiple = reader.get_short();
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.decode_multiple);
                builder.add_short(self.encode_multiple);
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Ping reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Ping {
            pub k: String,
        }

        impl Ping {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Ping {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.k = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.k);
                builder.get()
            }
        }
    }

    pub mod Account {
        use super::super::*;

        /// Request creating an account
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub username: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.username = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.username);
                builder.get()
            }
        }

        /// Confirm creating an account
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub session_id: EOShort,
            pub username: String,
            pub password: String,
            pub fullname: String,
            pub location: String,
            pub email: String,
            pub computer: String,
            pub hdid: String,
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                reader.get_byte();
                self.username = reader.get_break_string();
                self.password = reader.get_break_string();
                self.fullname = reader.get_break_string();
                self.location = reader.get_break_string();
                self.email = reader.get_break_string();
                self.computer = reader.get_break_string();
                self.hdid = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.username);
                builder.add_break_string(&self.password);
                builder.add_break_string(&self.fullname);
                builder.add_break_string(&self.location);
                builder.add_break_string(&self.email);
                builder.add_break_string(&self.computer);
                builder.add_break_string(&self.hdid);
                builder.get()
            }
        }

        /// Change password
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub username: String,
            pub oldpassword: String,
            pub newpassword: String,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.username = reader.get_break_string();
                self.oldpassword = reader.get_break_string();
                self.newpassword = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.username);
                builder.add_break_string(&self.oldpassword);
                builder.add_break_string(&self.newpassword);
                builder.get()
            }
        }
    }

    pub mod Character {
        use super::super::*;

        /// Request to create a character
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub new: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.new = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.new);
                builder.get()
            }
        }

        /// Confirm creating a character
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub session_id: EOShort,
            pub gender: Gender,
            pub hairstyle: EOShort,
            pub haircolor: EOShort,
            pub skin: Skin,
            pub name: String,
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.gender = Gender::from_char(reader.get_char());
                self.hairstyle = reader.get_short();
                self.haircolor = reader.get_short();
                self.skin = Skin::from_char(reader.get_char());
                reader.get_byte();
                self.name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_char(self.gender.to_char());
                builder.add_short(self.hairstyle);
                builder.add_short(self.haircolor);
                builder.add_char(self.skin.to_char());
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.name);
                builder.get()
            }
        }

        /// Confirm deleting character from an account
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub session_id: EOShort,
            pub character_id: EOInt,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.character_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_int(self.character_id);
                builder.get()
            }
        }

        /// Request to delete a character from an account
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub character_id: EOInt,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.character_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.character_id);
                builder.get()
            }
        }
    }

    pub mod Login {
        use super::super::*;

        /// Login request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub username: String,
            pub password: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.username = reader.get_break_string();
                self.password = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.username);
                builder.add_break_string(&self.password);
                builder.get()
            }
        }
    }

    pub mod Welcome {
        use super::super::*;

        /// Selected a character
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub character_id: EOInt,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.character_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.character_id);
                builder.get()
            }
        }

        /// Entering game
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Msg {
            pub session_id: EOThree,
            pub character_id: EOInt,
        }

        impl Msg {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Msg {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_three();
                self.character_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_three(self.session_id);
                builder.add_int(self.character_id);
                builder.get()
            }
        }

        /// Requesting a file
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum AgreeData {
            Map(AgreeMap),
            Item(AgreeItem),
            NPC(AgreeNPC),
            Spell(AgreeSpell),
            Class(AgreeClass),
        }

        impl Default for AgreeData {
            fn default() -> Self {
                Self::Map(AgreeMap::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub file_type: FileType,
            pub session_id: EOShort,
            pub data: AgreeData,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_type = FileType::from_char(reader.get_char());
                self.session_id = reader.get_short();
                match self.file_type {
                    FileType::Map => {
                        let mut map = AgreeMap::new();
                        map.deserialize(&reader);
                        self.data = AgreeData::Map(map);
                    }
                    FileType::Item => {
                        let mut item = AgreeItem::new();
                        item.deserialize(&reader);
                        self.data = AgreeData::Item(item);
                    }
                    FileType::NPC => {
                        let mut npc = AgreeNPC::new();
                        npc.deserialize(&reader);
                        self.data = AgreeData::NPC(npc);
                    }
                    FileType::Spell => {
                        let mut spell = AgreeSpell::new();
                        spell.deserialize(&reader);
                        self.data = AgreeData::Spell(spell);
                    }
                    FileType::Class => {
                        let mut class = AgreeClass::new();
                        class.deserialize(&reader);
                        self.data = AgreeData::Class(class);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_type.to_char());
                builder.add_short(self.session_id);
                match &self.data {
                    AgreeData::Map(map) => {
                        builder.append(&mut map.serialize());
                    }
                    AgreeData::Item(item) => {
                        builder.append(&mut item.serialize());
                    }
                    AgreeData::NPC(npc) => {
                        builder.append(&mut npc.serialize());
                    }
                    AgreeData::Spell(spell) => {
                        builder.append(&mut spell.serialize());
                    }
                    AgreeData::Class(class) => {
                        builder.append(&mut class.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeMap {
            pub character_id: EOShort,
        }

        impl AgreeMap {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeMap {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.character_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.character_id);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeItem {
            pub file_id: EOChar,
        }

        impl AgreeItem {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeItem {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeNPC {
            pub file_id: EOChar,
        }

        impl AgreeNPC {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeNPC {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeSpell {
            pub file_id: EOChar,
        }

        impl AgreeSpell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeSpell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeClass {
            pub file_id: EOChar,
        }

        impl AgreeClass {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeClass {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                builder.get()
            }
        }
    }

    pub mod AdminInteract {
        use super::super::*;

        /// Talk to admin
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Tell {
            pub message: String,
        }

        impl Tell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Tell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Report character
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Report {
            pub reportee: String,
            pub message: String,
        }

        impl Report {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Report {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reportee = reader.get_break_string();
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.reportee);
                builder.add_string(&self.message);
                builder.get()
            }
        }
    }

    pub mod Global {
        use super::super::*;

        /// Enable whispers
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {}

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'n');
                builder.get()
            }
        }

        /// Disable whispers
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {}

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'y');
                builder.get()
            }
        }

        /// Opened global tab
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {}

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'y');
                builder.get()
            }
        }

        /// Closed global tab
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {}

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'n');
                builder.get()
            }
        }
    }

    pub mod Talk {
        use super::super::*;

        /// Guild chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub message: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Party chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub message: String,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Global chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Msg {
            pub message: String,
        }

        impl Msg {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Msg {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Private chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Tell {
            pub name: String,
            pub message: String,
        }

        impl Tell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Tell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_break_string();
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.name);
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Public chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Report {
            pub message: String,
        }

        impl Report {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Report {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Admin chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Admin {
            pub message: String,
        }

        impl Admin {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Admin {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Admin announcement
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Announce {
            pub message: String,
        }

        impl Announce {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Announce {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }
    }

    pub mod Attack {
        use super::super::*;

        /// Attacking
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub direction: Direction,
            pub timestamp: EOThree,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.direction = Direction::from_char(reader.get_char());
                self.timestamp = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.direction.to_char());
                builder.add_three(self.timestamp);
                builder.get()
            }
        }
    }

    pub mod Chair {
        use super::super::*;

        /// Sitting on a chair
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum RequestData {
            Sit(RequestSit),
        }

        impl Default for RequestData {
            fn default() -> Self {
                Self::Sit(RequestSit::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub sit_action: SitAction,
            pub data: RequestData,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.sit_action = SitAction::from_char(reader.get_char());
                match self.sit_action {
                    SitAction::Sit => {
                        let mut sit = RequestSit::new();
                        sit.deserialize(&reader);
                        self.data = RequestData::Sit(sit);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.sit_action.to_char());
                match &self.data {
                    RequestData::Sit(sit) => {
                        builder.append(&mut sit.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct RequestSit {
            pub coords: TinyCoords,
        }

        impl RequestSit {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for RequestSit {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }
    }

    pub mod Sit {
        use super::super::*;

        /// Floor sit/stand request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub sit_action: SitAction,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.sit_action = SitAction::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.sit_action.to_char());
                builder.get()
            }
        }
    }

    pub mod Emote {
        use super::super::*;

        /// Doing an emote
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Report {
            pub emote: Emote,
        }

        impl Report {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Report {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.emote = Emote::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.emote.to_char());
                builder.get()
            }
        }
    }

    pub mod Face {
        use super::super::*;

        /// Facing a direction
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub direction: Direction,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.direction = Direction::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.direction.to_char());
                builder.get()
            }
        }
    }

    pub mod Walk {
        use super::super::*;

        /// Walking with #nowall
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Admin {
            pub walk: WalkCommon,
        }

        impl Admin {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Admin {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.walk.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.walk.serialize());
                builder.get()
            }
        }

        /// Walking through a player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub walk: WalkCommon,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.walk.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.walk.serialize());
                builder.get()
            }
        }

        /// Walking
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub walk: WalkCommon,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.walk.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.walk.serialize());
                builder.get()
            }
        }
    }

    pub mod Bank {
        use super::super::*;

        /// Talked to a banker NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.get()
            }
        }

        /// Depositing gold
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub amount: EOInt,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.amount);
                builder.get()
            }
        }

        /// Withdrawing gold
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub amount: EOInt,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.amount);
                builder.get()
            }
        }
    }

    pub mod Locker {
        use super::super::*;

        /// Placing an item in a bank locker
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub locker_coords: Coords,
            pub deposit_item: ShortItem,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.locker_coords.deserialize(&reader);
                self.deposit_item.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.locker_coords.serialize());
                builder.append(&mut self.deposit_item.serialize());
                builder.get()
            }
        }

        /// Taking an item from a bank locker
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub locker_coords: Coords,
            pub take_item_id: EOShort,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.locker_coords.deserialize(&reader);
                self.take_item_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.locker_coords.serialize());
                builder.add_short(self.take_item_id);
                builder.get()
            }
        }

        /// Opening a bank locker
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub locker_coords: Coords,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.locker_coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.locker_coords.serialize());
                builder.get()
            }
        }

        /// Buying a locker space upgrade from a banker NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Buy {}

        impl Buy {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Buy {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(1);
                builder.get()
            }
        }
    }

    pub mod Citizen {
        use super::super::*;

        /// Sleeping at an inn
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub session_id: EOShort,
            pub vendor_id: EOShort,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.vendor_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_short(self.vendor_id);
                builder.get()
            }
        }

        /// Confirm sleeping at an inn
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub session_id: EOShort,
            pub vendor_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.vendor_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_short(self.vendor_id);
                builder.get()
            }
        }

        /// Subscribing to a town
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub session_id: EOShort,
            pub vendor_id: EOShort,
            pub answer1: String,
            pub answer2: String,
            pub answer3: String,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                reader.get_byte();
                self.vendor_id = reader.get_short();
                reader.get_byte();
                self.answer1 = reader.get_break_string();
                self.answer2 = reader.get_break_string();
                self.answer3 = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_short(self.vendor_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.answer1);
                builder.add_break_string(&self.answer2);
                builder.add_string(&self.answer3);
                builder.get()
            }
        }

        /// Giving up citizenship of a town
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub session_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.get()
            }
        }

        /// Talking to a citizenship NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.get()
            }
        }
    }

    pub mod Shop {
        use super::super::*;

        /// Crafting an item from a shop
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub craft_item_id: EOShort,
            pub session_id: EOInt,
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.craft_item_id = reader.get_short();
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.craft_item_id);
                builder.add_int(self.session_id);
                builder.get()
            }
        }

        /// Purchasing an item from a shop
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Buy {
            pub buy_item_id: EOShort,
            pub session_id: EOInt,
        }

        impl Buy {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Buy {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.buy_item_id = reader.get_short();
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.buy_item_id);
                builder.add_int(self.session_id);
                builder.get()
            }
        }

        /// Selling an item to a shop
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Sell {
            pub sell_item: Item,
            pub session_id: EOInt,
        }

        impl Sell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Sell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.sell_item.deserialize(&reader);
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.sell_item.serialize());
                builder.add_int(self.session_id);
                builder.get()
            }
        }

        /// Talking to a shop NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.get()
            }
        }
    }

    pub mod StatSkill {
        use super::super::*;

        /// Talking to a skill master NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.get()
            }
        }

        /// Learning a skill from a skill master
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub session_id: EOInt,
            pub spell_id: EOShort,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.spell_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_short(self.spell_id);
                builder.get()
            }
        }

        /// Forgetting a skill at a skill master
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub session_id: EOInt,
            pub spell_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.spell_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_short(self.spell_id);
                builder.get()
            }
        }

        /// Spending a stat point on a stat or skill
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum AddData {
            Stat(AddStat),
            Skill(AddSkill),
        }

        impl Default for AddData {
            fn default() -> Self {
                Self::Stat(AddStat::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub action_type: TrainType,
            pub data: AddData,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.action_type = TrainType::from_char(reader.get_char());
                match self.action_type {
                    TrainType::Stat => {
                        let mut stat = AddStat::new();
                        stat.deserialize(&reader);
                        self.data = AddData::Stat(stat);
                    }
                    TrainType::Skill => {
                        let mut skill = AddSkill::new();
                        skill.deserialize(&reader);
                        self.data = AddData::Skill(skill);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.action_type.to_char());
                match &self.data {
                    AddData::Stat(stat) => {
                        builder.append(&mut stat.serialize());
                    }
                    AddData::Skill(skill) => {
                        builder.append(&mut skill.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AddStat {
            pub stat_id: StatId,
        }

        impl AddStat {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AddStat {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.stat_id = StatId::from_short(reader.get_short());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.stat_id.to_short());
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AddSkill {
            pub spell_id: EOShort,
        }

        impl AddSkill {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AddSkill {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.get()
            }
        }

        /// Resetting stats at a skill master
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Junk {
            pub session_id: EOInt,
        }

        impl Junk {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Junk {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.get()
            }
        }
    }

    pub mod Item {
        use super::super::*;

        /// Using an item
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub use_item_id: EOShort,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.use_item_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.use_item_id);
                builder.get()
            }
        }

        /// Dropping items on the ground
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Drop {
            pub drop_item: ShortItem,
            pub coords: Coords,
        }

        impl Drop {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Drop {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.drop_item.deserialize(&reader);
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.drop_item.serialize());
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }

        /// Junking items
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Junk {
            pub junk_item: Item,
        }

        impl Junk {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Junk {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.junk_item.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.junk_item.serialize());
                builder.get()
            }
        }

        /// Taking items from the ground
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Get {
            pub take_item_index: EOShort,
        }

        impl Get {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Get {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.take_item_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.take_item_index);
                builder.get()
            }
        }
    }

    pub mod Barber {
        use super::super::*;

        /// Talking to barber NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.get()
            }
        }

        /// Purchasing a hair-style
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Buy {
            pub style: EOChar,
            pub color: EOChar,
            pub session_id: EOInt,
        }

        impl Buy {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Buy {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.style = reader.get_char();
                self.color = reader.get_char();
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.style);
                builder.add_char(self.color);
                builder.add_int(self.session_id);
                builder.get()
            }
        }
    }

    pub mod Board {
        use super::super::*;

        /// Removing a post from a town board
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub board_id: EOShort,
            pub post_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.board_id = reader.get_short();
                self.post_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.board_id);
                builder.add_short(self.post_id);
                builder.get()
            }
        }

        /// Posting a new message to a town board
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub board_id: EOShort,
            pub post_subject: String,
            pub post_body: String,
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.board_id = reader.get_short();
                reader.get_byte();
                self.post_subject = reader.get_break_string();
                self.post_body = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.board_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.post_subject);
                builder.add_break_string(&self.post_body);
                builder.get()
            }
        }

        /// Reading a post on a town board
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub board_id: EOShort,
            pub post_id: EOShort,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.board_id = reader.get_short();
                self.post_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.board_id);
                builder.add_short(self.post_id);
                builder.get()
            }
        }

        /// Opening a a town board
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub board_id: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.board_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.board_id);
                builder.get()
            }
        }
    }

    pub mod Jukebox {
        use super::super::*;

        /// Opening the jukebox listing
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub coords: Coords,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }

        /// Requesting a song on a jukebox
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Msg {
            pub track_id: EOShort,
        }

        impl Msg {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Msg {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
                reader.get_byte();
                self.track_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_short(self.track_id);
                builder.get()
            }
        }

        /// Playing a note with the bard skill
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub instrument_id: EOChar,
            pub note_id: EOChar,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.instrument_id = reader.get_char();
                self.note_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.instrument_id);
                builder.add_char(self.note_id);
                builder.get()
            }
        }
    }

    pub mod Warp {
        use super::super::*;

        /// Accept a warp request from the server
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub map_id: EOShort,
            pub session_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.map_id = reader.get_short();
                self.session_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.map_id);
                builder.add_short(self.session_id);
                builder.get()
            }
        }

        /// Request to download a copy of the map
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub map_id: EOShort,
            pub session_id: EOShort,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.map_id = reader.get_short();
                self.session_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.map_id);
                builder.add_short(self.session_id);
                builder.get()
            }
        }
    }

    pub mod Paperdoll {
        use super::super::*;

        /// Request for a player's paperdoll
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub player_id: EOShort,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Unequipping an item
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub item_id: EOShort,
            pub sub_loc: EOChar,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.item_id = reader.get_short();
                self.sub_loc = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.item_id);
                builder.add_char(self.sub_loc);
                builder.get()
            }
        }

        /// Equipping an item
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub item_id: EOShort,
            pub sub_loc: EOChar,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.item_id = reader.get_short();
                self.sub_loc = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.item_id);
                builder.add_char(self.sub_loc);
                builder.get()
            }
        }
    }

    pub mod Book {
        use super::super::*;

        /// Request for a player's book
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub player_id: EOShort,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }
    }

    pub mod Players {
        use super::super::*;

        /// #find command request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub name: String,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.name);
                builder.get()
            }
        }

        /// Requesting a list of online players
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {}

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }

        /// Requesting a list of online friends
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {}

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }
    }

    pub mod Door {
        use super::super::*;

        /// Opening a door
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub coords: Coords,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }
    }

    pub mod Chest {
        use super::super::*;

        /// Opening a chest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub coords: Coords,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }

        /// Placing an item in to a chest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub coords: Coords,
            pub add_item: ShortItem,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
                self.add_item.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.append(&mut self.add_item.serialize());
                builder.get()
            }
        }

        /// Taking an item from a chest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub coords: Coords,
            pub take_item_id: EOShort,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
                self.take_item_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.add_short(self.take_item_id);
                builder.get()
            }
        }
    }

    pub mod Refresh {
        use super::super::*;

        /// Requesting new info about nearby objects
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {}

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }
    }

    pub mod Range {
        use super::super::*;

        /// Requesting info about nearby players and/or npcs
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub player_ids: Vec<EOShort>,
            pub npc_indexes: Vec<EOChar>,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.player_ids.push(reader.get_short());
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.npc_indexes.push(reader.get_char());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.player_ids.len() {
                    builder.add_short(self.player_ids[i]);
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.npc_indexes.len() {
                    builder.add_char(self.npc_indexes[i]);
                }
                builder.get()
            }
        }
    }

    pub mod PlayerRange {
        use super::super::*;

        /// Sent when unknown player has walked into view
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub player_ids: Vec<EOShort>,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.player_ids.push(reader.get_short());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.player_ids.len() {
                    builder.add_short(self.player_ids[i]);
                }
                builder.get()
            }
        }
    }

    pub mod NPCRange {
        use super::super::*;

        /// Sent when unknown npc has walked into view
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub num_npcs: EOChar,
            pub npc_indexes: Vec<EOChar>,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.num_npcs = reader.get_char();
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.npc_indexes.push(reader.get_char());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.num_npcs);
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.npc_indexes.len() {
                    builder.add_char(self.npc_indexes[i]);
                }
                builder.get()
            }
        }
    }

    pub mod Party {
        use super::super::*;

        /// Send party invite / join request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub request_type: PartyRequestType,
            pub player_id: EOShort,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.request_type = PartyRequestType::from_char(reader.get_char());
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.request_type.to_char());
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Accept party invite / join request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub request_type: PartyRequestType,
            pub player_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.request_type = PartyRequestType::from_char(reader.get_char());
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.request_type.to_char());
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Remove player from a party
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub player_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Request updated party info
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub num_members: EOChar,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.num_members = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.num_members);
                builder.get()
            }
        }
    }

    pub mod Guild {
        use super::super::*;

        /// Requested to create a guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub session_id: EOInt,
            pub guild_tag: String,
            pub guild_name: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                reader.get_byte();
                self.guild_tag = reader.get_break_string();
                self.guild_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.guild_tag);
                builder.add_break_string(&self.guild_name);
                builder.get()
            }
        }

        /// Accept pending guild creation invite
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub player_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_int();
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(20202);
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Leave guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub session_id: EOInt,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.get()
            }
        }

        /// Update the guild description or rank list
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum AgreeData {
            Description(AgreeDescription),
            Ranks(AgreeRanks),
        }

        impl Default for AgreeData {
            fn default() -> Self {
                Self::Description(AgreeDescription::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub session_id: EOInt,
            pub info_type: GuildInfoType,
            pub data: AgreeData,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.info_type = GuildInfoType::from_short(reader.get_short());
                match self.info_type {
                    GuildInfoType::Description => {
                        let mut description = AgreeDescription::new();
                        description.deserialize(&reader);
                        self.data = AgreeData::Description(description);
                    }
                    GuildInfoType::Ranks => {
                        let mut ranks = AgreeRanks::new();
                        ranks.deserialize(&reader);
                        self.data = AgreeData::Ranks(ranks);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_short(self.info_type.to_short());
                match &self.data {
                    AgreeData::Description(description) => {
                        builder.append(&mut description.serialize());
                    }
                    AgreeData::Ranks(ranks) => {
                        builder.append(&mut ranks.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeDescription {
            pub description: String,
        }

        impl AgreeDescription {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeDescription {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.description = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.description);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeRanks {
            pub ranks: [String; 9],
        }

        impl AgreeRanks {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeRanks {
            fn deserialize(&mut self, reader: &StreamReader) {
                for i in 0..9 {
                    self.ranks[i] = reader.get_break_string();
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.ranks.len() {
                    builder.add_break_string(&self.ranks[i]);
                }
                builder.get()
            }
        }

        /// Final confirm creating a guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub session_id: EOInt,
            pub guild_tag: String,
            pub guild_name: String,
            pub description: String,
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                reader.get_byte();
                self.guild_tag = reader.get_break_string();
                self.guild_name = reader.get_break_string();
                self.description = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.guild_tag);
                builder.add_break_string(&self.guild_name);
                builder.add_break_string(&self.description);
                builder.get()
            }
        }

        /// Request to join a guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub sesssion_id: EOInt,
            pub guild_tag: String,
            pub recruiter_name: String,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.sesssion_id = reader.get_int();
                reader.get_byte();
                self.guild_tag = reader.get_break_string();
                self.recruiter_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.sesssion_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.guild_tag);
                builder.add_break_string(&self.recruiter_name);
                builder.get()
            }
        }

        /// Request guild bank / ranks / description
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub session_id: EOInt,
            pub info_type: GuildInfoType,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.info_type = GuildInfoType::from_short(reader.get_short());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_short(self.info_type.to_short());
                builder.get()
            }
        }

        /// Accepted a join request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub player_id: EOShort,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Deposit gold in to the guild bank
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Buy {
            pub session_id: EOInt,
            pub gold_amount: EOInt,
        }

        impl Buy {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Buy {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }

        /// Talking to a guild master NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.get()
            }
        }

        /// Requested member list of a guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Tell {
            pub session_id: EOInt,
            pub guild_identity: String,
        }

        impl Tell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Tell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.guild_identity = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_string(&self.guild_identity);
                builder.get()
            }
        }

        /// Requested general information of a guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Report {
            pub session_id: EOInt,
            pub guild_identity: String,
        }

        impl Report {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Report {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.guild_identity = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_string(&self.guild_identity);
                builder.get()
            }
        }

        /// Disband guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Junk {
            pub session_id: EOInt,
        }

        impl Junk {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Junk {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.get()
            }
        }

        /// Kick member from guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Kick {
            pub session_id: EOInt,
            pub member_name: String,
        }

        impl Kick {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Kick {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                self.member_name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_string(&self.member_name);
                builder.get()
            }
        }

        /// Update a member's rank
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Rank {
            pub sesson_id: EOInt,
            pub rank: EOChar,
            pub member_name: String,
        }

        impl Rank {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Rank {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.sesson_id = reader.get_int();
                self.rank = reader.get_char();
                self.member_name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.sesson_id);
                builder.add_char(self.rank);
                builder.add_string(&self.member_name);
                builder.get()
            }
        }
    }

    pub mod Spell {
        use super::super::*;

        /// Begin spell chanting
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub spell_id: EOShort,
            pub timestamp: EOThree,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
                self.timestamp = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.add_three(self.timestamp);
                builder.get()
            }
        }

        /// Self-targeted spell cast
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetSelf {
            pub direction: Direction,
            pub spell_id: EOShort,
            pub timestamp: EOThree,
        }

        impl TargetSelf {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetSelf {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.direction = Direction::from_char(reader.get_char());
                self.spell_id = reader.get_short();
                self.timestamp = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.direction.to_char());
                builder.add_short(self.spell_id);
                builder.add_three(self.timestamp);
                builder.get()
            }
        }

        /// Targeted spell cast
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetOther {
            pub target_type: SpellTargetType,
            pub previous_timestamp: EOThree,
            pub spell_id: EOShort,
            pub timestamp: EOThree,
        }

        impl TargetOther {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetOther {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.target_type = SpellTargetType::from_char(reader.get_char());
                self.previous_timestamp = reader.get_three();
                self.spell_id = reader.get_short();
                self.timestamp = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.target_type.to_char());
                builder.add_three(self.previous_timestamp);
                builder.add_short(self.spell_id);
                builder.add_three(self.timestamp);
                builder.get()
            }
        }

        /// Group spell cast
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetGroup {
            pub spell_id: EOShort,
            pub timestamp: EOThree,
        }

        impl TargetGroup {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetGroup {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
                self.timestamp = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.add_three(self.timestamp);
                builder.get()
            }
        }

        /// Raise arm to cast a spell (vestigial)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub direction: Direction,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.direction = Direction::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.direction.to_char());
                builder.get()
            }
        }
    }

    pub mod Trade {
        use super::super::*;

        /// Requesting a trade with another player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub unk1: EOChar,
            pub player_id: EOShort,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.unk1 = reader.get_char();
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.unk1);
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Accepting a trade request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub unk1: EOChar,
            pub player_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.unk1 = reader.get_char();
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.unk1);
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Remove an item from the trade screen
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub item_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.item_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.item_id);
                builder.get()
            }
        }

        /// Mark trade as agreed
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub agree_state: EOChar,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.agree_state = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.agree_state);
                builder.get()
            }
        }

        /// Add an item to the trade screen
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub add_item: Item,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.add_item.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.add_item.serialize());
                builder.get()
            }
        }

        /// Cancel the trade
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {
            pub unk1: EOChar,
        }

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.unk1 = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.unk1);
                builder.get()
            }
        }
    }

    pub mod Quest {
        use super::super::*;

        /// Talking to a quest NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub npc_index: EOShort,
            pub quest_id: EOShort,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
                self.quest_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.add_short(self.quest_id);
                builder.get()
            }
        }

        /// Response to a quest NPC dialog
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum AcceptData {
            Link(AcceptLink),
        }

        impl Default for AcceptData {
            fn default() -> Self {
                Self::Link(AcceptLink::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub session_id: EOShort,
            pub dialog_id: EOShort,
            pub quest_id: EOShort,
            pub npc_index: EOShort,
            pub reply_type: DialogReply,
            pub data: AcceptData,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.dialog_id = reader.get_short();
                self.quest_id = reader.get_short();
                self.npc_index = reader.get_short();
                self.reply_type = DialogReply::from_char(reader.get_char());
                match self.reply_type {
                    DialogReply::Link => {
                        let mut link = AcceptLink::new();
                        link.deserialize(&reader);
                        self.data = AcceptData::Link(link);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_short(self.dialog_id);
                builder.add_short(self.quest_id);
                builder.add_short(self.npc_index);
                builder.add_char(self.reply_type.to_char());
                match &self.data {
                    AcceptData::Link(link) => {
                        builder.append(&mut link.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AcceptLink {
            pub action: EOChar,
        }

        impl AcceptLink {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AcceptLink {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.action = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.action);
                builder.get()
            }
        }

        /// Quest history / progress request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {
            pub page: QuestPage,
        }

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.page = QuestPage::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.page.to_char());
                builder.get()
            }
        }
    }

    pub mod Marriage {
        use super::super::*;

        /// Talking to a law NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOShort,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.get()
            }
        }

        /// Requesting marriage approval
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub request_type: MarriageRequestType,
            pub session_id: EOInt,
            pub name: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.request_type = MarriageRequestType::from_char(reader.get_char());
                self.session_id = reader.get_int();
                reader.get_byte();
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.request_type.to_char());
                builder.add_int(self.session_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_string(&self.name);
                builder.get()
            }
        }
    }

    pub mod Priest {
        use super::super::*;

        /// Accepting a marriage request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub session_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.get()
            }
        }

        /// Talking to a priest NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub npc_index: EOInt,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.npc_index);
                builder.get()
            }
        }

        /// Requesting marriage at a priest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub session_id: EOInt,
            pub name: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
                reader.get_byte();
                self.name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.name);
                builder.get()
            }
        }

        /// Saying 'I do' at a wedding
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub session_id: EOInt,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.get()
            }
        }
    }
}

pub mod server {
    use super::*;

    pub mod Init {
        use super::super::*;

        /// Initialization reply
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum InitData {
            OutOfDate(InitOutOfDate),
            OK(InitOK),
            Banned(InitBanned),
            WarpFileEMF(InitWarpFileEMF),
            FileEMF(InitFileEMF),
            FileEIF(InitFileEIF),
            FileENF(InitFileENF),
            FileESF(InitFileESF),
            FileECF(InitFileECF),
            MapMutation(InitMapMutation),
            Players(InitPlayers),
            FriendListPlayers(InitFriendListPlayers),
        }

        impl Default for InitData {
            fn default() -> Self {
                Self::OutOfDate(InitOutOfDate::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Init {
            pub reply_code: InitReply,
            pub data: InitData,
        }

        impl Init {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Init {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = InitReply::from_byte(reader.get_byte());
                match self.reply_code {
                    InitReply::OutOfDate => {
                        let mut out_of_date = InitOutOfDate::new();
                        out_of_date.deserialize(&reader);
                        self.data = InitData::OutOfDate(out_of_date);
                    }
                    InitReply::OK => {
                        let mut ok = InitOK::new();
                        ok.deserialize(&reader);
                        self.data = InitData::OK(ok);
                    }
                    InitReply::Banned => {
                        let mut banned = InitBanned::new();
                        banned.deserialize(&reader);
                        self.data = InitData::Banned(banned);
                    }
                    InitReply::WarpFileEMF => {
                        let mut warp_file_emf = InitWarpFileEMF::new();
                        warp_file_emf.deserialize(&reader);
                        self.data = InitData::WarpFileEMF(warp_file_emf);
                    }
                    InitReply::FileEMF => {
                        let mut file_emf = InitFileEMF::new();
                        file_emf.deserialize(&reader);
                        self.data = InitData::FileEMF(file_emf);
                    }
                    InitReply::FileEIF => {
                        let mut file_eif = InitFileEIF::new();
                        file_eif.deserialize(&reader);
                        self.data = InitData::FileEIF(file_eif);
                    }
                    InitReply::FileENF => {
                        let mut file_enf = InitFileENF::new();
                        file_enf.deserialize(&reader);
                        self.data = InitData::FileENF(file_enf);
                    }
                    InitReply::FileESF => {
                        let mut file_esf = InitFileESF::new();
                        file_esf.deserialize(&reader);
                        self.data = InitData::FileESF(file_esf);
                    }
                    InitReply::FileECF => {
                        let mut file_ecf = InitFileECF::new();
                        file_ecf.deserialize(&reader);
                        self.data = InitData::FileECF(file_ecf);
                    }
                    InitReply::MapMutation => {
                        let mut map_mutation = InitMapMutation::new();
                        map_mutation.deserialize(&reader);
                        self.data = InitData::MapMutation(map_mutation);
                    }
                    InitReply::Players => {
                        let mut players = InitPlayers::new();
                        players.deserialize(&reader);
                        self.data = InitData::Players(players);
                    }
                    InitReply::FriendListPlayers => {
                        let mut friends = InitFriendListPlayers::new();
                        friends.deserialize(&reader);
                        self.data = InitData::FriendListPlayers(friends);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(self.reply_code.to_byte());
                match &self.data {
                    InitData::OutOfDate(out_of_date) => {
                        builder.append(&mut out_of_date.serialize());
                    }
                    InitData::OK(ok) => {
                        builder.append(&mut ok.serialize());
                    }
                    InitData::Banned(banned) => {
                        builder.append(&mut banned.serialize());
                    }
                    InitData::WarpFileEMF(warp_file_emf) => {
                        builder.append(&mut warp_file_emf.serialize());
                    }
                    InitData::FileEMF(file_emf) => {
                        builder.append(&mut file_emf.serialize());
                    }
                    InitData::FileEIF(file_eif) => {
                        builder.append(&mut file_eif.serialize());
                    }
                    InitData::FileENF(file_enf) => {
                        builder.append(&mut file_enf.serialize());
                    }
                    InitData::FileESF(file_esf) => {
                        builder.append(&mut file_esf.serialize());
                    }
                    InitData::FileECF(file_ecf) => {
                        builder.append(&mut file_ecf.serialize());
                    }
                    InitData::MapMutation(map_mutation) => {
                        builder.append(&mut map_mutation.serialize());
                    }
                    InitData::Players(players) => {
                        builder.append(&mut players.serialize());
                    }
                    InitData::FriendListPlayers(friends) => {
                        builder.append(&mut friends.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitOutOfDate {
            pub version: [EOChar; 3],
        }

        impl InitOutOfDate {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitOutOfDate {
            fn deserialize(&mut self, reader: &StreamReader) {
                for i in 0..3 {
                    self.version[i] = reader.get_char();
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.version.len() {
                    builder.add_char(self.version[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitOK {
            pub seq_bytes: [EOByte; 2],
            pub encode_multiple: EOByte,
            pub decode_multiple: EOByte,
            pub player_id: EOShort,
            pub response: EOThree,
        }

        impl InitOK {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitOK {
            fn deserialize(&mut self, reader: &StreamReader) {
                for i in 0..2 {
                    self.seq_bytes[i] = reader.get_byte();
                }
                self.encode_multiple = reader.get_byte();
                self.decode_multiple = reader.get_byte();
                self.player_id = reader.get_short();
                self.response = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.seq_bytes.len() {
                    builder.add_byte(self.seq_bytes[i]);
                }
                builder.add_byte(self.encode_multiple);
                builder.add_byte(self.decode_multiple);
                builder.add_short(self.player_id);
                builder.add_three(self.response);
                builder.get()
            }
        }

        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum InitBannedData {
            Temp0(InitBannedTemp0),
            Temp(InitBannedTemp),
        }

        impl Default for InitBannedData {
            fn default() -> Self {
                Self::Temp0(InitBannedTemp0::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitBanned {
            pub ban_type: InitBanType,
            pub data: InitBannedData,
        }

        impl InitBanned {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitBanned {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.ban_type = InitBanType::from_byte(reader.get_byte());
                match self.ban_type {
                    InitBanType::Temp0 => {
                        let mut ban_temp0 = InitBannedTemp0::new();
                        ban_temp0.deserialize(&reader);
                        self.data = InitBannedData::Temp0(ban_temp0);
                    }
                    InitBanType::Temp => {
                        let mut ban_temp = InitBannedTemp::new();
                        ban_temp.deserialize(&reader);
                        self.data = InitBannedData::Temp(ban_temp);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(self.ban_type.to_byte());
                match &self.data {
                    InitBannedData::Temp0(ban_temp0) => {
                        builder.append(&mut ban_temp0.serialize());
                    }
                    InitBannedData::Temp(ban_temp) => {
                        builder.append(&mut ban_temp.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitBannedTemp0 {
            pub mins_remaining: EOByte,
        }

        impl InitBannedTemp0 {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitBannedTemp0 {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.mins_remaining = reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(self.mins_remaining);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitBannedTemp {
            pub mins_remaining: EOByte,
        }

        impl InitBannedTemp {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitBannedTemp {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.mins_remaining = reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(self.mins_remaining);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitWarpFileEMF {
            pub content: Vec<EOByte>,
        }

        impl InitWarpFileEMF {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitWarpFileEMF {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.content.push(reader.get_byte());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.content.len() {
                    builder.add_byte(self.content[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitFileEMF {
            pub content: Vec<EOByte>,
        }

        impl InitFileEMF {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitFileEMF {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.content.push(reader.get_byte());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.content.len() {
                    builder.add_byte(self.content[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitFileEIF {
            pub file_id: EOChar,
            pub content: Vec<EOByte>,
        }

        impl InitFileEIF {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitFileEIF {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.content.push(reader.get_byte());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                for i in 0..self.content.len() {
                    builder.add_byte(self.content[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitFileENF {
            pub file_id: EOChar,
            pub content: Vec<EOByte>,
        }

        impl InitFileENF {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitFileENF {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.content.push(reader.get_byte());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                for i in 0..self.content.len() {
                    builder.add_byte(self.content[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitFileESF {
            pub file_id: EOChar,
            pub content: Vec<EOByte>,
        }

        impl InitFileESF {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitFileESF {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.content.push(reader.get_byte());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                for i in 0..self.content.len() {
                    builder.add_byte(self.content[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitFileECF {
            pub file_id: EOChar,
            pub content: Vec<EOByte>,
        }

        impl InitFileECF {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitFileECF {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.file_id = reader.get_char();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.content.push(reader.get_byte());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.file_id);
                for i in 0..self.content.len() {
                    builder.add_byte(self.content[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitMapMutation {
            pub content: Vec<EOByte>,
        }

        impl InitMapMutation {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitMapMutation {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.content.push(reader.get_byte());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.content.len() {
                    builder.add_byte(self.content[i]);
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitPlayers {
            pub num_online: EOShort,
            pub list: Vec<OnlinePlayers>,
        }

        impl InitPlayers {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitPlayers {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.num_online = reader.get_short();
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut online_players = OnlinePlayers::new();
                    online_players.deserialize(&reader);
                    self.list.push(online_players);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.num_online);
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.list.len() {
                    builder.append(&mut self.list[i].serialize());
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct InitFriendListPlayers {
            pub num_online: EOShort,
            pub list: Vec<String>,
        }

        impl InitFriendListPlayers {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for InitFriendListPlayers {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.num_online = reader.get_short();
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.list.push(reader.get_break_string());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.num_online);
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.list.len() {
                    builder.add_break_string(&self.list[i]);
                }
                builder.get()
            }
        }
    }

    pub mod Connection {
        use super::super::*;

        /// Ping request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub seq1: EOShort,
            pub seq2: EOChar,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.seq1 = reader.get_short();
                self.seq2 = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.seq1);
                builder.add_char(self.seq2);
                builder.get()
            }
        }
    }

    pub mod Account {
        use super::super::*;

        /// Reply to client Account-family packets
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: AccountReply,
            pub sequence_start: EOChar,
            pub ok_no: String,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = AccountReply::from_short(reader.get_short());
                self.sequence_start = reader.get_char();
                self.ok_no = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                builder.add_char(self.sequence_start);
                builder.add_string(&self.ok_no);
                builder.get()
            }
        }
    }

    pub mod Character {
        use super::super::*;

        /// Reply to client Character-family packets
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            OK(ReplyOK),
            Deleted(ReplyDeleted),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::OK(ReplyOK::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: CharacterReply,
            pub ok_no: String,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = CharacterReply::from_short(reader.get_short());
                self.ok_no = reader.get_end_string();
                match self.reply_code {
                    CharacterReply::OK => {
                        let mut ok = ReplyOK::new();
                        ok.deserialize(&reader);
                        self.data = ReplyData::OK(ok);
                    }
                    CharacterReply::Deleted => {
                        let mut deleted = ReplyDeleted::new();
                        deleted.deserialize(&reader);
                        self.data = ReplyData::Deleted(deleted);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                builder.add_string(&self.ok_no);
                match &self.data {
                    ReplyData::OK(ok) => {
                        builder.append(&mut ok.serialize());
                    }
                    ReplyData::Deleted(deleted) => {
                        builder.append(&mut deleted.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyOK {
            pub character_list: CharacterList,
        }

        impl ReplyOK {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyOK {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.character_list.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.character_list.serialize());
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyDeleted {
            pub character_list: CharacterList,
        }

        impl ReplyDeleted {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyDeleted {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.character_list.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.character_list.serialize());
                builder.get()
            }
        }

        /// Reply to client Character_Take
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub session_id: EOShort,
            pub character_id: EOInt,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.character_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_int(self.character_id);
                builder.get()
            }
        }
    }

    pub mod Login {
        use super::super::*;

        /// Login reply
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            OK(ReplyOK),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::OK(ReplyOK::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: LoginReply,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = LoginReply::from_short(reader.get_short());
                match self.reply_code {
                    LoginReply::OK => {
                        let mut ok = ReplyOK::new();
                        ok.deserialize(&reader);
                        self.data = ReplyData::OK(ok);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                match &self.data {
                    ReplyData::OK(ok) => {
                        builder.append(&mut ok.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyOK {
            pub character_list: CharacterList,
        }

        impl ReplyOK {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyOK {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.character_list.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.character_list.serialize());
                builder.get()
            }
        }
    }

    pub mod Welcome {
        use super::super::*;

        /// Reply to selecting a character / entering game
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            SelectCharacter(ReplySelectCharacter),
            EnterGame(ReplyEnterGame),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::SelectCharacter(ReplySelectCharacter::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: WelcomeReply,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = WelcomeReply::from_short(reader.get_short());
                match self.reply_code {
                    WelcomeReply::SelectCharacter => {
                        let mut select_character = ReplySelectCharacter::new();
                        select_character.deserialize(&reader);
                        self.data = ReplyData::SelectCharacter(select_character);
                    }
                    WelcomeReply::EnterGame => {
                        let mut enter_game = ReplyEnterGame::new();
                        enter_game.deserialize(&reader);
                        self.data = ReplyData::EnterGame(enter_game);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                match &self.data {
                    ReplyData::SelectCharacter(select_character) => {
                        builder.append(&mut select_character.serialize());
                    }
                    ReplyData::EnterGame(enter_game) => {
                        builder.append(&mut enter_game.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplySelectCharacter {
            pub session_id: EOShort,
            pub character_id: EOInt,
            pub map_id: EOShort,
            pub map_rid: [EOShort; 2],
            pub map_filesize: EOThree,
            pub eif_rid: [EOShort; 2],
            pub eif_length: EOShort,
            pub enf_rid: [EOShort; 2],
            pub enf_length: EOShort,
            pub esf_rid: [EOShort; 2],
            pub esf_length: EOShort,
            pub ecf_rid: [EOShort; 2],
            pub ecf_length: EOShort,
            pub name: String,
            pub title: String,
            pub guild_name: String,
            pub guild_rank_name: String,
            pub class_id: EOChar,
            pub guild_tag: String,
            pub admin: AdminLevel,
            pub level: EOChar,
            pub experience: EOInt,
            pub usage: EOInt,
            pub stats: CharacterStats2,
            pub paperdoll: PaperdollFull,
            pub guild_rank: EOChar,
            pub settings: ServerSettings,
            pub login_message: EOChar,
        }

        impl ReplySelectCharacter {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplySelectCharacter {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.character_id = reader.get_int();
                self.map_id = reader.get_short();
                for i in 0..2 {
                    self.map_rid[i] = reader.get_short();
                }
                self.map_filesize = reader.get_three();
                for i in 0..2 {
                    self.eif_rid[i] = reader.get_short();
                }
                self.eif_length = reader.get_short();
                for i in 0..2 {
                    self.enf_rid[i] = reader.get_short();
                }
                self.enf_length = reader.get_short();
                for i in 0..2 {
                    self.esf_rid[i] = reader.get_short();
                }
                self.esf_length = reader.get_short();
                for i in 0..2 {
                    self.ecf_rid[i] = reader.get_short();
                }
                self.ecf_length = reader.get_short();
                self.name = reader.get_break_string();
                self.title = reader.get_break_string();
                self.guild_name = reader.get_break_string();
                self.guild_rank_name = reader.get_break_string();
                self.class_id = reader.get_char();
                self.guild_tag = reader.get_fixed_string(3 as usize);
                self.admin = AdminLevel::from_char(reader.get_char());
                self.level = reader.get_char();
                self.experience = reader.get_int();
                self.usage = reader.get_int();
                self.stats.deserialize(&reader);
                self.paperdoll.deserialize(&reader);
                self.guild_rank = reader.get_char();
                self.settings.deserialize(&reader);
                self.login_message = reader.get_char();
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_int(self.character_id);
                builder.add_short(self.map_id);
                for i in 0..self.map_rid.len() {
                    builder.add_short(self.map_rid[i]);
                }
                builder.add_three(self.map_filesize);
                for i in 0..self.eif_rid.len() {
                    builder.add_short(self.eif_rid[i]);
                }
                builder.add_short(self.eif_length);
                for i in 0..self.enf_rid.len() {
                    builder.add_short(self.enf_rid[i]);
                }
                builder.add_short(self.enf_length);
                for i in 0..self.esf_rid.len() {
                    builder.add_short(self.esf_rid[i]);
                }
                builder.add_short(self.esf_length);
                for i in 0..self.ecf_rid.len() {
                    builder.add_short(self.ecf_rid[i]);
                }
                builder.add_short(self.ecf_length);
                builder.add_break_string(&self.name);
                builder.add_break_string(&self.title);
                builder.add_break_string(&self.guild_name);
                builder.add_break_string(&self.guild_rank_name);
                builder.add_char(self.class_id);
                builder.add_fixed_string(&self.guild_tag, 3 as usize);
                builder.add_char(self.admin.to_char());
                builder.add_char(self.level);
                builder.add_int(self.experience);
                builder.add_int(self.usage);
                builder.append(&mut self.stats.serialize());
                builder.append(&mut self.paperdoll.serialize());
                builder.add_char(self.guild_rank);
                builder.append(&mut self.settings.serialize());
                builder.add_char(self.login_message);
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyEnterGame {
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
                reader.get_byte();
                for i in 0..9 {
                    self.news[i] = reader.get_break_string();
                }
                self.weight.deserialize(&reader);
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut item = Item::new();
                    item.deserialize(&reader);
                    self.items.push(item);
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut spell = Spell::new();
                    spell.deserialize(&reader);
                    self.spells.push(spell);
                }
                reader.get_byte();
                self.nearby.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.news.len() {
                    builder.add_break_string(&self.news[i]);
                }
                builder.append(&mut self.weight.serialize());
                for i in 0..self.items.len() {
                    builder.append(&mut self.items[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.spells.len() {
                    builder.append(&mut self.spells[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                builder.append(&mut self.nearby.serialize());
                builder.get()
            }
        }
    }

    pub mod AdminInteract {
        use super::super::*;

        /// Incoming admin report
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            Message(ReplyMessage),
            Report(ReplyReport),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::Message(ReplyMessage::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub message_type: AdminMessageType,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message_type = AdminMessageType::from_char(reader.get_char());
                reader.get_byte();
                match self.message_type {
                    AdminMessageType::Message => {
                        let mut message = ReplyMessage::new();
                        message.deserialize(&reader);
                        self.data = ReplyData::Message(message);
                    }
                    AdminMessageType::Report => {
                        let mut report = ReplyReport::new();
                        report.deserialize(&reader);
                        self.data = ReplyData::Report(report);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.message_type.to_char());
                builder.add_byte(EO_BREAK_CHAR);
                match &self.data {
                    ReplyData::Message(message) => {
                        builder.append(&mut message.serialize());
                    }
                    ReplyData::Report(report) => {
                        builder.append(&mut report.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyMessage {
            pub player_name: String,
            pub message: String,
        }

        impl ReplyMessage {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyMessage {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyReport {
            pub player_name: String,
            pub message: String,
            pub reportee_name: String,
        }

        impl ReplyReport {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyReport {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
                self.message = reader.get_break_string();
                self.reportee_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.add_break_string(&self.message);
                builder.add_break_string(&self.reportee_name);
                builder.get()
            }
        }

        /// Nearby player disappearing (admin hide)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub player_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Nearby player appearing (admin un-hide)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub player_id: EOShort,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Admin character inventory popup
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {
            pub name: String,
            pub usage: EOInt,
            pub gold_bank: EOInt,
            pub inventory: Vec<Item>,
            pub bank: Vec<ShortItem>,
        }

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_break_string();
                self.usage = reader.get_int();
                reader.get_byte();
                self.gold_bank = reader.get_int();
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut item = Item::new();
                    item.deserialize(&reader);
                    self.inventory.push(item);
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.bank.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.name);
                builder.add_int(self.usage);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_int(self.gold_bank);
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.inventory.len() {
                    builder.append(&mut self.inventory[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.bank.len() {
                    builder.append(&mut self.bank[i].serialize());
                }
                builder.get()
            }
        }

        /// Admin character info popup
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Tell {
            pub name: String,
            pub usage: EOInt,
            pub exp: EOInt,
            pub level: EOChar,
            pub map_id: EOShort,
            pub map_coords: Coords,
            pub stats: CharacterStats4,
            pub light: EOShort,
            pub dark: EOShort,
            pub fire: EOShort,
            pub water: EOShort,
            pub earth: EOShort,
            pub wind: EOShort,
            pub weight: Weight,
        }

        impl Tell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Tell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_break_string();
                self.usage = reader.get_int();
                reader.get_byte();
                reader.get_byte();
                self.exp = reader.get_int();
                self.level = reader.get_char();
                self.map_id = reader.get_short();
                self.map_coords.deserialize(&reader);
                self.stats.deserialize(&reader);
                self.light = reader.get_short();
                self.dark = reader.get_short();
                self.fire = reader.get_short();
                self.water = reader.get_short();
                self.earth = reader.get_short();
                self.wind = reader.get_short();
                self.weight.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.name);
                builder.add_int(self.usage);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_int(self.exp);
                builder.add_char(self.level);
                builder.add_short(self.map_id);
                builder.append(&mut self.map_coords.serialize());
                builder.append(&mut self.stats.serialize());
                builder.add_short(self.light);
                builder.add_short(self.dark);
                builder.add_short(self.fire);
                builder.add_short(self.water);
                builder.add_short(self.earth);
                builder.add_short(self.wind);
                builder.append(&mut self.weight.serialize());
                builder.get()
            }
        }
    }

    pub mod Talk {
        use super::super::*;

        /// Guild chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub player_name: String,
            pub message: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Party chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub player_id: EOShort,
            pub message: String,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Global chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Msg {
            pub player_name: String,
            pub message: String,
        }

        impl Msg {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Msg {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Private chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Tell {
            pub player_name: String,
            pub message: String,
        }

        impl Tell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Tell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Public chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub message: String,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Admin chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Admin {
            pub player_name: String,
            pub message: String,
        }

        impl Admin {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Admin {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Admin announcement
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Announce {
            pub player_name: String,
            pub message: String,
        }

        impl Announce {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Announce {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Server message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Server {
            pub message: String,
        }

        impl Server {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Server {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Reply to trying to send a private message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: TalkReply,
            pub name: String,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = TalkReply::from_short(reader.get_short());
                self.name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                builder.add_break_string(&self.name);
                builder.get()
            }
        }

        /// Global chat backfill
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {
            pub messages: Vec<GlobalBackfillMessage>,
        }

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut global_backfill_message = GlobalBackfillMessage::new();
                    global_backfill_message.deserialize(&reader);
                    self.messages.push(global_backfill_message);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.messages.len() {
                    builder.append(&mut self.messages[i].serialize());
                }
                builder.get()
            }
        }

        /// Temporary mute applied
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub admin_name: String,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.admin_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.admin_name);
                builder.get()
            }
        }
    }

    pub mod Message {
        use super::super::*;

        /// Status bar message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub message: String,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.message = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.message);
                builder.get()
            }
        }

        /// Large message box
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub messages: [String; 4],
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                for i in 0..4 {
                    self.messages[i] = reader.get_break_string();
                }
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.messages.len() {
                    builder.add_break_string(&self.messages[i]);
                }
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }

        /// #ping command reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Pong {}

        impl Pong {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Pong {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(2);
                builder.get()
            }
        }

        /// Server is rebooting
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {}

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'r');
                builder.get()
            }
        }
    }

    pub mod Attack {
        use super::super::*;

        /// Nearby player attacking
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub direction: Direction,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.direction = Direction::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.direction.to_char());
                builder.get()
            }
        }
    }

    pub mod Avatar {
        use super::super::*;

        /// Nearby player hit by another player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub player_id: EOShort,
            pub victim_id: EOShort,
            pub damage: EOThree,
            pub direction: Direction,
            pub hp_percentage: EOChar,
            pub died: EOChar,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.victim_id = reader.get_short();
                self.damage = reader.get_three();
                self.direction = Direction::from_char(reader.get_char());
                self.hp_percentage = reader.get_char();
                self.died = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_short(self.victim_id);
                builder.add_three(self.damage);
                builder.add_char(self.direction.to_char());
                builder.add_char(self.hp_percentage);
                builder.add_char(self.died);
                builder.get()
            }
        }

        /// Nearby player has disappeared from view
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub player_id: EOShort,
            pub animation: WarpAnimation,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.animation = WarpAnimation::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.animation.to_char());
                builder.get()
            }
        }

        /// Nearby player changed clothes
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub change: AvatarChange,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.change.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.change.serialize());
                builder.get()
            }
        }

        /// Nearby player hit by a damage spell from a player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Admin {
            pub caster_id: EOShort,
            pub victim_id: EOShort,
            pub caster_direction: EOChar,
            pub damage: EOThree,
            pub hp_percentage: EOChar,
            pub victim_died: EOChar,
            pub spell_id: EOShort,
        }

        impl Admin {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Admin {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.caster_id = reader.get_short();
                self.victim_id = reader.get_short();
                self.caster_direction = reader.get_char();
                self.damage = reader.get_three();
                self.hp_percentage = reader.get_char();
                self.victim_died = reader.get_char();
                self.spell_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.caster_id);
                builder.add_short(self.victim_id);
                builder.add_char(self.caster_direction);
                builder.add_three(self.damage);
                builder.add_char(self.hp_percentage);
                builder.add_char(self.victim_died);
                builder.add_short(self.spell_id);
                builder.get()
            }
        }
    }

    pub mod Chair {
        use super::super::*;

        /// Nearby player sitting on a chair
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub coords: Coords,
            pub direction: Direction,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.coords.deserialize(&reader);
                self.direction = Direction::from_char(reader.get_char());
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.append(&mut self.coords.serialize());
                builder.add_char(self.direction.to_char());
                builder.add_char(0);
                builder.get()
            }
        }

        /// Your character sitting on a chair
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub player_id: EOShort,
            pub coords: Coords,
            pub direction: Direction,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.coords.deserialize(&reader);
                self.direction = Direction::from_char(reader.get_char());
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.append(&mut self.coords.serialize());
                builder.add_char(self.direction.to_char());
                builder.add_char(0);
                builder.get()
            }
        }

        /// Your character standing up from a chair
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {
            pub player_id: EOShort,
            pub coords: Coords,
        }

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }
    }

    pub mod Sit {
        use super::super::*;

        /// Nearby player sitting down
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub coords: Coords,
            pub direction: Direction,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.coords.deserialize(&reader);
                self.direction = Direction::from_char(reader.get_char());
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.append(&mut self.coords.serialize());
                builder.add_char(self.direction.to_char());
                builder.add_char(0);
                builder.get()
            }
        }

        /// Your character standing up
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {
            pub player_id: EOShort,
            pub coords: Coords,
        }

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }

        /// Nearby player standing up
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub player_id: EOShort,
            pub coords: Coords,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }

        /// Your character sitting down
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub player_id: EOShort,
            pub coords: Coords,
            pub direction: Direction,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.coords.deserialize(&reader);
                self.direction = Direction::from_char(reader.get_char());
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.append(&mut self.coords.serialize());
                builder.add_char(self.direction.to_char());
                builder.add_char(0);
                builder.get()
            }
        }
    }

    pub mod Emote {
        use super::super::*;

        /// Nearby player doing an emote
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub emote: Emote,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.emote = Emote::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.emote.to_char());
                builder.get()
            }
        }
    }

    pub mod Effect {
        use super::super::*;

        /// Nearby player doing an effect
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub effect_id: EOThree,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.effect_id = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_three(self.effect_id);
                builder.get()
            }
        }

        /// Map effect
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub effect: MapEffect,
            pub param: EOChar,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.effect = MapEffect::from_char(reader.get_char());
                self.param = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.effect.to_char());
                builder.add_char(self.param);
                builder.get()
            }
        }

        /// Map spike timer
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Report {}

        impl Report {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Report {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'S');
                builder.get()
            }
        }

        /// Map spell effect
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub coords: Coords,
            pub effect_id: EOShort,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
                self.effect_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.add_short(self.effect_id);
                builder.get()
            }
        }

        /// Taking spike or tp drain damage
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum SpecData {
            TPDrain(SpecTPDrain),
            Spikes(SpecSpikes),
        }

        impl Default for SpecData {
            fn default() -> Self {
                Self::TPDrain(SpecTPDrain::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub map_damage_type: MapDamageType,
            pub data: SpecData,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.map_damage_type = MapDamageType::from_char(reader.get_char());
                match self.map_damage_type {
                    MapDamageType::TPDrain => {
                        let mut tp_drain = SpecTPDrain::new();
                        tp_drain.deserialize(&reader);
                        self.data = SpecData::TPDrain(tp_drain);
                    }
                    MapDamageType::Spikes => {
                        let mut spikes = SpecSpikes::new();
                        spikes.deserialize(&reader);
                        self.data = SpecData::Spikes(spikes);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.map_damage_type.to_char());
                match &self.data {
                    SpecData::TPDrain(tp_drain) => {
                        builder.append(&mut tp_drain.serialize());
                    }
                    SpecData::Spikes(spikes) => {
                        builder.append(&mut spikes.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct SpecTPDrain {
            pub tp_damage: EOShort,
            pub tp: EOShort,
            pub max_tp: EOShort,
        }

        impl SpecTPDrain {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for SpecTPDrain {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.tp_damage = reader.get_short();
                self.tp = reader.get_short();
                self.max_tp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.tp_damage);
                builder.add_short(self.tp);
                builder.add_short(self.max_tp);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct SpecSpikes {
            pub damage: EOShort,
            pub hp: EOShort,
            pub max_hp: EOShort,
        }

        impl SpecSpikes {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for SpecSpikes {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.damage = reader.get_short();
                self.hp = reader.get_short();
                self.max_hp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.damage);
                builder.add_short(self.hp);
                builder.add_short(self.max_hp);
                builder.get()
            }
        }

        /// Map drain damage
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetOther {
            pub damage: EOShort,
            pub hp: EOShort,
            pub max_hp: EOShort,
            pub others: Vec<MapDrainDamageOther>,
        }

        impl TargetOther {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetOther {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.damage = reader.get_short();
                self.hp = reader.get_short();
                self.max_hp = reader.get_short();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut map_drain_damage_other = MapDrainDamageOther::new();
                    map_drain_damage_other.deserialize(&reader);
                    self.others.push(map_drain_damage_other);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.damage);
                builder.add_short(self.hp);
                builder.add_short(self.max_hp);
                for i in 0..self.others.len() {
                    builder.append(&mut self.others[i].serialize());
                }
                builder.get()
            }
        }

        /// Nearby character taking spike damage
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Admin {
            pub player_id: EOShort,
            pub hp_percentage: EOChar,
            pub died: EOChar,
            pub damage: EOThree,
        }

        impl Admin {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Admin {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.hp_percentage = reader.get_char();
                self.died = reader.get_char();
                self.damage = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.hp_percentage);
                builder.add_char(self.died);
                builder.add_three(self.damage);
                builder.get()
            }
        }
    }

    pub mod Face {
        use super::super::*;

        /// Nearby player facing a direction
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub direction: Direction,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.direction = Direction::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.direction.to_char());
                builder.get()
            }
        }
    }

    pub mod Players {
        use super::super::*;

        /// Player has appeared in nearby view
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub nearby: NearbyInfo,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.nearby.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.nearby.serialize());
                builder.get()
            }
        }

        /// Nearby player has logged out
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub player_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// #find command reply - offline
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Ping {
            pub name: String,
        }

        impl Ping {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Ping {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.name);
                builder.get()
            }
        }

        /// #find command reply - same map
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Pong {
            pub name: String,
        }

        impl Pong {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Pong {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.name);
                builder.get()
            }
        }

        /// #find command reply - different map
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Net3 {
            pub name: String,
        }

        impl Net3 {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Net3 {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.name);
                builder.get()
            }
        }
    }

    pub mod Walk {
        use super::super::*;

        /// Nearby player has walked
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub direction: Direction,
            pub coords: Coords,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.direction = Direction::from_char(reader.get_char());
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.direction.to_char());
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }

        /// Players, NPCs, and Items appearing in nearby view
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub player_ids: Vec<EOShort>,
            pub npc_indexes: Vec<EOChar>,
            pub items: Vec<ItemMapInfo>,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.player_ids.push(reader.get_short());
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.npc_indexes.push(reader.get_char());
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut item_map_info = ItemMapInfo::new();
                    item_map_info.deserialize(&reader);
                    self.items.push(item_map_info);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.player_ids.len() {
                    builder.add_short(self.player_ids[i]);
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.npc_indexes.len() {
                    builder.add_char(self.npc_indexes[i]);
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.items.len() {
                    builder.append(&mut self.items[i].serialize());
                }
                builder.get()
            }
        }

        /// Your character has been frozen
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {}

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'f');
                builder.get()
            }
        }

        /// Your character has been unfrozen
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {}

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'u');
                builder.get()
            }
        }
    }

    pub mod Bank {
        use super::super::*;

        /// Open banker NPC interface
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub gold_bank: EOInt,
            pub session_token: EOThree,
            pub locker_upgrades: EOChar,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_bank = reader.get_int();
                self.session_token = reader.get_three();
                self.locker_upgrades = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_bank);
                builder.add_three(self.session_token);
                builder.add_char(self.locker_upgrades);
                builder.get()
            }
        }

        /// Update gold counts after deposit/withdraw
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub gold_inventory: EOInt,
            pub gold_bank: EOInt,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_inventory = reader.get_int();
                self.gold_bank = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_inventory);
                builder.add_int(self.gold_bank);
                builder.get()
            }
        }
    }

    pub mod Locker {
        use super::super::*;

        /// Response to adding an item to a bank locker
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub deposited_item: Item,
            pub weight: Weight,
            pub locker_items: Vec<ShortItem>,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.deposited_item.deserialize(&reader);
                self.weight.deserialize(&reader);
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.locker_items.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.deposited_item.serialize());
                builder.append(&mut self.weight.serialize());
                for i in 0..self.locker_items.len() {
                    builder.append(&mut self.locker_items[i].serialize());
                }
                builder.get()
            }
        }

        /// Response to taking an item from a bank locker
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Get {
            pub taken_item: ShortItem,
            pub weight: Weight,
            pub locker_items: Vec<ShortItem>,
        }

        impl Get {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Get {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.taken_item.deserialize(&reader);
                self.weight.deserialize(&reader);
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.locker_items.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.taken_item.serialize());
                builder.append(&mut self.weight.serialize());
                for i in 0..self.locker_items.len() {
                    builder.append(&mut self.locker_items[i].serialize());
                }
                builder.get()
            }
        }

        /// Opening a bank locker
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub locker_coords: Coords,
            pub locker_items: Vec<ShortItem>,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.locker_coords.deserialize(&reader);
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.locker_items.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.locker_coords.serialize());
                for i in 0..self.locker_items.len() {
                    builder.append(&mut self.locker_items[i].serialize());
                }
                builder.get()
            }
        }

        /// Response to buying a locker space upgrade from a banker NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Buy {
            pub gold_amount: EOInt,
            pub locker_upgrades: EOChar,
        }

        impl Buy {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Buy {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
                self.locker_upgrades = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.add_char(self.locker_upgrades);
                builder.get()
            }
        }

        /// Reply to trying to add an item to a full locker
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub locker_max_items: EOChar,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.locker_max_items = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.locker_max_items);
                builder.get()
            }
        }
    }

    pub mod Citizen {
        use super::super::*;

        /// Response to subscribing to a town
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub questions_wrong: EOChar,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.questions_wrong = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.questions_wrong);
                builder.get()
            }
        }

        /// Response to giving up citizenship of a town
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub reply_code: InnUnsubscribeReply,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = InnUnsubscribeReply::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.reply_code.to_char());
                builder.get()
            }
        }

        /// Response from talking to a citizenship NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub vendor_id: EOThree,
            pub current_home_id: EOChar,
            pub session_id: EOShort,
            pub question1: String,
            pub question2: String,
            pub question3: String,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.vendor_id = reader.get_three();
                self.current_home_id = reader.get_char();
                self.session_id = reader.get_short();
                reader.get_byte();
                self.question1 = reader.get_break_string();
                self.question2 = reader.get_break_string();
                self.question3 = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_three(self.vendor_id);
                builder.add_char(self.current_home_id);
                builder.add_short(self.session_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.question1);
                builder.add_break_string(&self.question2);
                builder.add_string(&self.question3);
                builder.get()
            }
        }

        /// Sleeping at an inn
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub gold_amount: EOInt,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }

        /// Reply to requesting sleeping at an inn
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub cost: EOInt,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.cost = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.cost);
                builder.get()
            }
        }
    }

    pub mod Shop {
        use super::super::*;

        /// Response to crafting an item from a shop
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub craft_item_id: EOShort,
            pub weight: Weight,
            pub ingredients: [Item; 4],
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.craft_item_id = reader.get_short();
                self.weight.deserialize(&reader);
                for i in 0..4 {
                    self.ingredients[i].deserialize(&reader);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.craft_item_id);
                builder.append(&mut self.weight.serialize());
                for i in 0..self.ingredients.len() {
                    builder.append(&mut self.ingredients[i].serialize());
                }
                builder.get()
            }
        }

        /// Response to purchasing an item from a shop
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Buy {
            pub gold_amount: EOInt,
            pub bought_item: Item,
            pub weight: Weight,
        }

        impl Buy {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Buy {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
                self.bought_item.deserialize(&reader);
                self.weight.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.append(&mut self.bought_item.serialize());
                builder.append(&mut self.weight.serialize());
                builder.get()
            }
        }

        /// Response to selling an item to a shop
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Sell {
            pub sold_item: ReverseItem,
            pub gold_amount: EOInt,
            pub weight: Weight,
        }

        impl Sell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Sell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.sold_item.deserialize(&reader);
                self.gold_amount = reader.get_int();
                self.weight.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.sold_item.serialize());
                builder.add_int(self.gold_amount);
                builder.append(&mut self.weight.serialize());
                builder.get()
            }
        }

        /// Response from talking to a shop NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub session_id: EOShort,
            pub shop_name: String,
            pub trade_items: Vec<ShopTradeItem>,
            pub craft_items: Vec<ShopCraftItem>,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.shop_name = reader.get_break_string();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut shop_trade_item = ShopTradeItem::new();
                    shop_trade_item.deserialize(&reader);
                    self.trade_items.push(shop_trade_item);
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut shop_craft_item = ShopCraftItem::new();
                    shop_craft_item.deserialize(&reader);
                    self.craft_items.push(shop_craft_item);
                }
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_break_string(&self.shop_name);
                for i in 0..self.trade_items.len() {
                    builder.append(&mut self.trade_items[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.craft_items.len() {
                    builder.append(&mut self.craft_items[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }
    }

    pub mod StatSkill {
        use super::super::*;

        /// Response from talking to a skill master NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub session_id: EOShort,
            pub shop_name: String,
            pub skills: Vec<SkillLearn>,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.shop_name = reader.get_break_string();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut skill_learn = SkillLearn::new();
                    skill_learn.deserialize(&reader);
                    self.skills.push(skill_learn);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_break_string(&self.shop_name);
                for i in 0..self.skills.len() {
                    builder.append(&mut self.skills[i].serialize());
                }
                builder.get()
            }
        }

        /// Response from unsuccessful attempt to learn a skill from a skill master
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            RemoveItems(ReplyRemoveItems),
            WrongClass(ReplyWrongClass),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::RemoveItems(ReplyRemoveItems::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: SkillMasterReply,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = SkillMasterReply::from_char(reader.get_char());
                match self.reply_code {
                    SkillMasterReply::RemoveItems => {
                        let mut remove_items = ReplyRemoveItems::new();
                        remove_items.deserialize(&reader);
                        self.data = ReplyData::RemoveItems(remove_items);
                    }
                    SkillMasterReply::WrongClass => {
                        let mut wrong_class = ReplyWrongClass::new();
                        wrong_class.deserialize(&reader);
                        self.data = ReplyData::WrongClass(wrong_class);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.reply_code.to_char());
                match &self.data {
                    ReplyData::RemoveItems(remove_items) => {
                        builder.append(&mut remove_items.serialize());
                    }
                    ReplyData::WrongClass(wrong_class) => {
                        builder.append(&mut wrong_class.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyRemoveItems {}

        impl ReplyRemoveItems {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyRemoveItems {
            fn deserialize(&mut self, reader: &StreamReader) {}

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyWrongClass {
            pub class_id: EOShort,
        }

        impl ReplyWrongClass {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyWrongClass {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.class_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.class_id);
                builder.get()
            }
        }

        /// Response from learning a skill from a skill master
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub spell_id: EOShort,
            pub gold_amount: EOInt,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }

        /// Response to forgetting a skill at a skill master
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub spell_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.get()
            }
        }

        /// Response to spending stat points
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub stat_points: EOShort,
            pub stats: CharacterStats3,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.stat_points = reader.get_short();
                self.stats.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.stat_points);
                builder.append(&mut self.stats.serialize());
                builder.get()
            }
        }

        /// Response to spending skill points
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub skill_points: EOShort,
            pub spell: Spell,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.skill_points = reader.get_short();
                self.spell.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.skill_points);
                builder.append(&mut self.spell.serialize());
                builder.get()
            }
        }

        /// Response to resetting character
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Junk {
            pub stats: CharacterStats1,
        }

        impl Junk {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Junk {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.stats.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.stats.serialize());
                builder.get()
            }
        }
    }

    pub mod Item {
        use super::super::*;

        /// Reply to using an item
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            Heal(ReplyHeal),
            HairDye(ReplyHairDye),
            EffectPotion(ReplyEffectPotion),
            CureCurse(ReplyCureCurse),
            EXPReward(ReplyEXPReward),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::Heal(ReplyHeal::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub used_item_type: ItemType,
            pub used_item: Item,
            pub weight: Weight,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.used_item_type = ItemType::from_char(reader.get_char());
                self.used_item.deserialize(&reader);
                self.weight.deserialize(&reader);
                match self.used_item_type {
                    ItemType::Heal => {
                        let mut heal = ReplyHeal::new();
                        heal.deserialize(&reader);
                        self.data = ReplyData::Heal(heal);
                    }
                    ItemType::HairDye => {
                        let mut hair_dye = ReplyHairDye::new();
                        hair_dye.deserialize(&reader);
                        self.data = ReplyData::HairDye(hair_dye);
                    }
                    ItemType::EffectPotion => {
                        let mut effect_potion = ReplyEffectPotion::new();
                        effect_potion.deserialize(&reader);
                        self.data = ReplyData::EffectPotion(effect_potion);
                    }
                    ItemType::CureCurse => {
                        let mut cure_curse = ReplyCureCurse::new();
                        cure_curse.deserialize(&reader);
                        self.data = ReplyData::CureCurse(cure_curse);
                    }
                    ItemType::EXPReward => {
                        let mut exp_reward = ReplyEXPReward::new();
                        exp_reward.deserialize(&reader);
                        self.data = ReplyData::EXPReward(exp_reward);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.used_item_type.to_char());
                builder.append(&mut self.used_item.serialize());
                builder.append(&mut self.weight.serialize());
                match &self.data {
                    ReplyData::Heal(heal) => {
                        builder.append(&mut heal.serialize());
                    }
                    ReplyData::HairDye(hair_dye) => {
                        builder.append(&mut hair_dye.serialize());
                    }
                    ReplyData::EffectPotion(effect_potion) => {
                        builder.append(&mut effect_potion.serialize());
                    }
                    ReplyData::CureCurse(cure_curse) => {
                        builder.append(&mut cure_curse.serialize());
                    }
                    ReplyData::EXPReward(exp_reward) => {
                        builder.append(&mut exp_reward.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyHeal {
            pub hp_gain: EOInt,
            pub hp: EOShort,
            pub tp: EOShort,
        }

        impl ReplyHeal {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyHeal {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.hp_gain = reader.get_int();
                self.hp = reader.get_short();
                self.tp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.hp_gain);
                builder.add_short(self.hp);
                builder.add_short(self.tp);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyHairDye {
            pub hair_color: EOChar,
        }

        impl ReplyHairDye {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyHairDye {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.hair_color = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.hair_color);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyEffectPotion {
            pub effect_id: EOShort,
        }

        impl ReplyEffectPotion {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyEffectPotion {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.effect_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.effect_id);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyCureCurse {
            pub stats: ItemCharacterStats,
        }

        impl ReplyCureCurse {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyCureCurse {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.stats.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.stats.serialize());
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyEXPReward {
            pub experience: EOInt,
            /// "A value greater than 0 is 'new level' and indicates the player leveled up."
            pub level_up: EOChar,
            pub stat_points: EOShort,
            pub skill_points: EOShort,
            pub max_hp: EOShort,
            pub max_tp: EOShort,
            pub max_sp: EOShort,
        }

        impl ReplyEXPReward {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyEXPReward {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.experience = reader.get_int();
                self.level_up = reader.get_char();
                self.stat_points = reader.get_short();
                self.skill_points = reader.get_short();
                self.max_hp = reader.get_short();
                self.max_tp = reader.get_short();
                self.max_sp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.experience);
                builder.add_char(self.level_up);
                builder.add_short(self.stat_points);
                builder.add_short(self.skill_points);
                builder.add_short(self.max_hp);
                builder.add_short(self.max_tp);
                builder.add_short(self.max_sp);
                builder.get()
            }
        }

        /// Reply to dropping items on the ground
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Drop {
            pub dropped_item: ShortItem,
            pub item_index: EOShort,
            pub coords: Coords,
            pub weight: Weight,
        }

        impl Drop {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Drop {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.dropped_item.deserialize(&reader);
                reader.get_int();
                self.item_index = reader.get_short();
                self.coords.deserialize(&reader);
                self.weight.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.dropped_item.serialize());
                builder.add_int(0);
                builder.add_short(self.item_index);
                builder.append(&mut self.coords.serialize());
                builder.append(&mut self.weight.serialize());
                builder.get()
            }
        }

        /// Item appeared on the ground
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub item_id: EOShort,
            pub item_index: EOShort,
            pub item_amount: EOThree,
            pub coords: Coords,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.item_id = reader.get_short();
                self.item_index = reader.get_short();
                self.item_amount = reader.get_three();
                self.coords.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.item_id);
                builder.add_short(self.item_index);
                builder.add_three(self.item_amount);
                builder.append(&mut self.coords.serialize());
                builder.get()
            }
        }

        /// Reply to junking items
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Junk {
            pub junked_item: ShortItem,
            pub junked_item_amount: EOInt,
            pub weight: Weight,
        }

        impl Junk {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Junk {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.junked_item.deserialize(&reader);
                self.junked_item_amount = reader.get_int();
                self.weight.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.junked_item.serialize());
                builder.add_int(self.junked_item_amount);
                builder.append(&mut self.weight.serialize());
                builder.get()
            }
        }

        /// Reply to taking items from the ground
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Get {
            pub taken_item_index: EOShort,
            pub taken_item: ShortItem,
            pub weight: Weight,
        }

        impl Get {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Get {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.taken_item_index = reader.get_short();
                self.taken_item.deserialize(&reader);
                self.weight.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.taken_item_index);
                builder.append(&mut self.taken_item.serialize());
                builder.append(&mut self.weight.serialize());
                builder.get()
            }
        }

        /// Receive item (from quest)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Obtain {
            pub item: ShortItem,
            pub cur_weight: EOChar,
        }

        impl Obtain {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Obtain {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.item.deserialize(&reader);
                self.cur_weight = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.item.serialize());
                builder.add_char(self.cur_weight);
                builder.get()
            }
        }

        /// Lose item (from quest)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Kick {
            pub item: Item,
            pub cur_weight: EOChar,
        }

        impl Kick {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Kick {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.item.deserialize(&reader);
                self.cur_weight = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.item.serialize());
                builder.add_char(self.cur_weight);
                builder.get()
            }
        }

        /// Reply to using an item that you don't have
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub item_id: EOShort,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.item_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.item_id);
                builder.get()
            }
        }

        /// Reply to trying to take a protected item from the ground
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {}

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(2);
                builder.get()
            }
        }

        /// Nearby player levelled up from quest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub player_id: EOShort,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }
    }

    pub mod Barber {
        use super::super::*;

        /// Reply from talking to barber NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub session_id: EOInt,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.get()
            }
        }

        /// Reply from purchasing a hair-style
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub gold_amount: EOInt,
            pub change: AvatarChange,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
                self.change.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.append(&mut self.change.serialize());
                builder.get()
            }
        }
    }

    pub mod Board {
        use super::super::*;

        /// Reply to reading a post on a town board
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub post_id: EOShort,
            pub post_body: String,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.post_id = reader.get_short();
                self.post_body = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.post_id);
                builder.add_break_string(&self.post_body);
                builder.get()
            }
        }

        /// Reply to opening a town board
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub board_id: EOChar,
            pub num_posts: EOChar,
            pub posts: Vec<BoardPostListing>,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.board_id = reader.get_char();
                self.num_posts = reader.get_char();
                for _ in 0..self.num_posts {
                    let mut board_post_listing = BoardPostListing::new();
                    board_post_listing.deserialize(&reader);
                    self.posts.push(board_post_listing);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.board_id);
                builder.add_char(self.num_posts);
                for i in 0..self.posts.len() {
                    builder.append(&mut self.posts[i].serialize());
                }
                builder.get()
            }
        }
    }

    pub mod Jukebox {
        use super::super::*;

        /// Reply to successfully requesting a song
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub gold_amount: EOInt,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }

        /// Reply to unsuccessfully requesting a song
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {}

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(1);
                builder.get()
            }
        }

        /// Reply to opening the jukebox listing
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub map_id: EOShort,
            pub jukebox_player: String,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.map_id = reader.get_short();
                self.jukebox_player = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.map_id);
                builder.add_break_string(&self.jukebox_player);
                builder.get()
            }
        }

        /// Someone playing a note with the bard skill nearby
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Msg {
            pub player_id: EOShort,
            pub direction: Direction,
            pub instrument_id: EOChar,
            pub note_id: EOChar,
        }

        impl Msg {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Msg {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.direction = Direction::from_char(reader.get_char());
                self.instrument_id = reader.get_char();
                self.note_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.direction.to_char());
                builder.add_char(self.instrument_id);
                builder.add_char(self.note_id);
                builder.get()
            }
        }

        /// Play jukebox music
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub mfx_id: EOChar,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.mfx_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.mfx_id);
                builder.get()
            }
        }

        /// Play background music
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub mfx_id: EOChar,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.mfx_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.mfx_id);
                builder.get()
            }
        }
    }

    pub mod Warp {
        use super::super::*;

        /// Warp request from server
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum RequestData {
            MapSwitch(RequestMapSwitch),
        }

        impl Default for RequestData {
            fn default() -> Self {
                Self::MapSwitch(RequestMapSwitch::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub warp_type: WarpType,
            pub map_id: EOShort,
            pub data: RequestData,
            pub session_id: EOShort,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.warp_type = WarpType::from_char(reader.get_char());
                self.map_id = reader.get_short();
                match self.warp_type {
                    WarpType::MapSwitch => {
                        let mut map_switch = RequestMapSwitch::new();
                        map_switch.deserialize(&reader);
                        self.data = RequestData::MapSwitch(map_switch);
                    }
                    _ => {}
                }
                self.session_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.warp_type.to_char());
                builder.add_short(self.map_id);
                match &self.data {
                    RequestData::MapSwitch(map_switch) => {
                        builder.append(&mut map_switch.serialize());
                    }
                    _ => {}
                }
                builder.add_short(self.session_id);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct RequestMapSwitch {
            pub map_rid: [EOShort; 2],
            pub map_filesize: EOThree,
        }

        impl RequestMapSwitch {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for RequestMapSwitch {
            fn deserialize(&mut self, reader: &StreamReader) {
                for i in 0..2 {
                    self.map_rid[i] = reader.get_short();
                }
                self.map_filesize = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.map_rid.len() {
                    builder.add_short(self.map_rid[i]);
                }
                builder.add_three(self.map_filesize);
                builder.get()
            }
        }

        /// Reply after accepting a warp
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum AgreeData {
            MapSwitch(AgreeMapSwitch),
        }

        impl Default for AgreeData {
            fn default() -> Self {
                Self::MapSwitch(AgreeMapSwitch::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub warp_type: WarpType,
            pub data: AgreeData,
            pub nearby: NearbyInfo,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.warp_type = WarpType::from_char(reader.get_char());
                match self.warp_type {
                    WarpType::MapSwitch => {
                        let mut map_switch = AgreeMapSwitch::new();
                        map_switch.deserialize(&reader);
                        self.data = AgreeData::MapSwitch(map_switch);
                    }
                    _ => {}
                }
                self.nearby.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.warp_type.to_char());
                match &self.data {
                    AgreeData::MapSwitch(map_switch) => {
                        builder.append(&mut map_switch.serialize());
                    }
                    _ => {}
                }
                builder.append(&mut self.nearby.serialize());
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct AgreeMapSwitch {
            pub map_id: EOShort,
            pub warp_anim: WarpAnimation,
        }

        impl AgreeMapSwitch {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for AgreeMapSwitch {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.map_id = reader.get_short();
                self.warp_anim = WarpAnimation::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.map_id);
                builder.add_char(self.warp_anim.to_char());
                builder.get()
            }
        }
    }

    pub mod Paperdoll {
        use super::super::*;

        /// Reply to requesting a paperdoll
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub info: PaperdollInfo,
            pub paperdoll: PaperdollFull,
            pub paperdoll_icon: PaperdollIcon,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.info.deserialize(&reader);
                self.paperdoll.deserialize(&reader);
                self.paperdoll_icon = PaperdollIcon::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.info.serialize());
                builder.append(&mut self.paperdoll.serialize());
                builder.add_char(self.paperdoll_icon.to_char());
                builder.get()
            }
        }

        /// Reply to unequipping an item
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub change: AvatarChange,
            pub stats: ItemCharacterStats,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.change.deserialize(&reader);
                self.stats.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.change.serialize());
                builder.append(&mut self.stats.serialize());
                builder.get()
            }
        }

        /// Reply to equipping an item
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub change: AvatarChange,
            pub stats: ItemCharacterStats,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.change.deserialize(&reader);
                self.stats.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.change.serialize());
                builder.append(&mut self.stats.serialize());
                builder.get()
            }
        }

        /// Failed to equip an item due to being the incorrect class
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Ping {
            /// "The player's current class ID -- not the item's required class ID"
            pub class_id: EOChar,
        }

        impl Ping {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Ping {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.class_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.class_id);
                builder.get()
            }
        }
    }

    pub mod Book {
        use super::super::*;

        /// Reply to requesting a book
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub info: PaperdollInfo,
            pub paperdoll_icon: PaperdollIcon,
            pub quest_name: Vec<String>,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.info.deserialize(&reader);
                self.paperdoll_icon = PaperdollIcon::from_char(reader.get_char());
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.quest_name.push(reader.get_break_string());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.info.serialize());
                builder.add_char(self.paperdoll_icon.to_char());
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.quest_name.len() {
                    builder.add_break_string(&self.quest_name[i]);
                }
                builder.get()
            }
        }
    }

    pub mod Door {
        use super::super::*;

        /// Door nearby opening
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub coords: Coords,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                builder.add_char(0);
                builder.get()
            }
        }

        /// Reply to trying to open a locked door
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {
            pub key: EOChar,
        }

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.key = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.key);
                builder.get()
            }
        }
    }

    pub mod Chest {
        use super::super::*;

        /// Reply to opening a chest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub coords: Coords,
            pub items: Vec<ShortItem>,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.coords.deserialize(&reader);
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.items.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.coords.serialize());
                for i in 0..self.items.len() {
                    builder.append(&mut self.items[i].serialize());
                }
                builder.get()
            }
        }

        /// Reply to placing an item in to a chest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub added_item: Item,
            pub weight: Weight,
            pub items: Vec<ShortItem>,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.added_item.deserialize(&reader);
                self.weight.deserialize(&reader);
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.items.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.added_item.serialize());
                builder.append(&mut self.weight.serialize());
                for i in 0..self.items.len() {
                    builder.append(&mut self.items[i].serialize());
                }
                builder.get()
            }
        }

        /// Reply to removing an item from a chest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Get {
            pub taken_item: ShortItem,
            pub weight: Weight,
            pub items: Vec<ShortItem>,
        }

        impl Get {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Get {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.taken_item.deserialize(&reader);
                self.weight.deserialize(&reader);
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.items.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.taken_item.serialize());
                builder.append(&mut self.weight.serialize());
                for i in 0..self.items.len() {
                    builder.append(&mut self.items[i].serialize());
                }
                builder.get()
            }
        }

        /// Chest contents updating
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub items: Vec<ShortItem>,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut short_item = ShortItem::new();
                    short_item.deserialize(&reader);
                    self.items.push(short_item);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.items.len() {
                    builder.append(&mut self.items[i].serialize());
                }
                builder.get()
            }
        }

        /// Reply to trying to add an item to a full chest
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {}

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(0);
                builder.get()
            }
        }
    }

    pub mod Refresh {
        use super::super::*;

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub nearby: NearbyInfo,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.nearby.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.nearby.serialize());
                builder.get()
            }
        }
    }

    pub mod Range {
        use super::super::*;

        /// MapInfo for characters and/or npcs
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub nearby: NearbyInfo,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.nearby.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.nearby.serialize());
                builder.get()
            }
        }
    }

    pub mod Party {
        use super::super::*;

        /// Received party invite / join request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub request_type: PartyRequestType,
            pub player_id: EOShort,
            pub player_name: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.request_type = PartyRequestType::from_char(reader.get_char());
                self.player_id = reader.get_short();
                self.player_name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.request_type.to_char());
                builder.add_short(self.player_id);
                builder.add_string(&self.player_name);
                builder.get()
            }
        }

        /// Member list received when party is first joined
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub members: Vec<PartyMember>,
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut party_member = PartyMember::new();
                    party_member.deserialize(&reader);
                    self.members.push(party_member);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.members.len() {
                    builder.append(&mut self.members[i].serialize());
                }
                builder.get()
            }
        }

        /// New player joined the party
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Add {
            pub member: PartyMember,
        }

        impl Add {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Add {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.member.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.member.serialize());
                builder.get()
            }
        }

        /// Player left the party
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Remove {
            pub player_id: EOShort,
        }

        impl Remove {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Remove {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }

        /// Left / disbanded a party
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {}

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }

        /// Party member list update
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {
            pub members: Vec<PartyMember>,
        }

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut party_member = PartyMember::new();
                    party_member.deserialize(&reader);
                    self.members.push(party_member);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.members.len() {
                    builder.append(&mut self.members[i].serialize());
                }
                builder.get()
            }
        }

        /// Update party member's HP
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub player_id: EOShort,
            pub hp_percentage: EOChar,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.hp_percentage = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.hp_percentage);
                builder.get()
            }
        }

        /// Updated experience and level-ups from party experience
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetGroup {
            pub gains: Vec<PartyExpShare>,
        }

        impl TargetGroup {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetGroup {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut party_exp_share = PartyExpShare::new();
                    party_exp_share.deserialize(&reader);
                    self.gains.push(party_exp_share);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.gains.len() {
                    builder.append(&mut self.gains[i].serialize());
                }
                builder.get()
            }
        }

        /// Failed party invite / join request
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            AlreadyInAnotherParty(ReplyAlreadyInAnotherParty),
            AlreadyInYourParty(ReplyAlreadyInYourParty),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::AlreadyInAnotherParty(ReplyAlreadyInAnotherParty::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: PartyReply,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = PartyReply::from_char(reader.get_char());
                match self.reply_code {
                    PartyReply::AlreadyInAnotherParty => {
                        let mut already_in_another_party = ReplyAlreadyInAnotherParty::new();
                        already_in_another_party.deserialize(&reader);
                        self.data = ReplyData::AlreadyInAnotherParty(already_in_another_party);
                    }
                    PartyReply::AlreadyInYourParty => {
                        let mut already_in_your_party = ReplyAlreadyInYourParty::new();
                        already_in_your_party.deserialize(&reader);
                        self.data = ReplyData::AlreadyInYourParty(already_in_your_party);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.reply_code.to_char());
                match &self.data {
                    ReplyData::AlreadyInAnotherParty(already_in_another_party) => {
                        builder.append(&mut already_in_another_party.serialize());
                    }
                    ReplyData::AlreadyInYourParty(already_in_your_party) => {
                        builder.append(&mut already_in_your_party.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyAlreadyInAnotherParty {
            pub player_name: String,
        }

        impl ReplyAlreadyInAnotherParty {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyAlreadyInAnotherParty {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyAlreadyInYourParty {
            pub player_name: String,
        }

        impl ReplyAlreadyInYourParty {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyAlreadyInYourParty {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.player_name);
                builder.get()
            }
        }
    }

    pub mod Guild {
        use super::super::*;

        /// Generic guild reply messages
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            CreateAdd(ReplyCreateAdd),
            CreateAddConfirm(ReplyCreateAddConfirm),
            JoinRequest(ReplyJoinRequest),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::CreateAdd(ReplyCreateAdd::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: GuildReply,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = GuildReply::from_short(reader.get_short());
                match self.reply_code {
                    GuildReply::CreateAdd => {
                        let mut create_add = ReplyCreateAdd::new();
                        create_add.deserialize(&reader);
                        self.data = ReplyData::CreateAdd(create_add);
                    }
                    GuildReply::CreateAddConfirm => {
                        let mut create_add_confirm = ReplyCreateAddConfirm::new();
                        create_add_confirm.deserialize(&reader);
                        self.data = ReplyData::CreateAddConfirm(create_add_confirm);
                    }
                    GuildReply::JoinRequest => {
                        let mut join_request = ReplyJoinRequest::new();
                        join_request.deserialize(&reader);
                        self.data = ReplyData::JoinRequest(join_request);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                match &self.data {
                    ReplyData::CreateAdd(create_add) => {
                        builder.append(&mut create_add.serialize());
                    }
                    ReplyData::CreateAddConfirm(create_add_confirm) => {
                        builder.append(&mut create_add_confirm.serialize());
                    }
                    ReplyData::JoinRequest(join_request) => {
                        builder.append(&mut join_request.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyCreateAdd {
            pub name: String,
        }

        impl ReplyCreateAdd {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyCreateAdd {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.name);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyCreateAddConfirm {
            pub name: String,
        }

        impl ReplyCreateAddConfirm {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyCreateAddConfirm {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_string(&self.name);
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplyJoinRequest {
            pub player_id: EOShort,
            pub name: String,
        }

        impl ReplyJoinRequest {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplyJoinRequest {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_string(&self.name);
                builder.get()
            }
        }

        /// Guild create request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub player_id: EOShort,
            pub guild_identity: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.guild_identity = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_string(&self.guild_identity);
                builder.get()
            }
        }

        /// Guild created
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Create {
            pub leader_id: EOShort,
            pub guild_tag: String,
            pub guild_name: String,
            pub rank_name: String,
            pub gold_amount: EOInt,
        }

        impl Create {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Create {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.leader_id = reader.get_short();
                reader.get_byte();
                self.guild_tag = reader.get_break_string();
                self.guild_name = reader.get_break_string();
                self.rank_name = reader.get_break_string();
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.leader_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.guild_tag);
                builder.add_break_string(&self.guild_name);
                builder.add_break_string(&self.rank_name);
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }

        /// Get guild description reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Take {
            pub description: String,
        }

        impl Take {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Take {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.description = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.description);
                builder.get()
            }
        }

        /// Get guild rank list reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Rank {
            pub ranks: [String; 9],
        }

        impl Rank {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Rank {
            fn deserialize(&mut self, reader: &StreamReader) {
                for i in 0..9 {
                    self.ranks[i] = reader.get_break_string();
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.ranks.len() {
                    builder.add_break_string(&self.ranks[i]);
                }
                builder.get()
            }
        }

        /// Get guild bank reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Sell {
            pub gold_amount: EOInt,
        }

        impl Sell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Sell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }

        /// Deposit guild bank reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Buy {
            pub gold_amount: EOInt,
        }

        impl Buy {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Buy {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }

        /// Talk to guild master NPC reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub session_id: EOThree,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_three(self.session_id);
                builder.get()
            }
        }

        /// Get guild member list reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Tell {
            pub num_members: EOShort,
            pub members: Vec<GuildMember>,
        }

        impl Tell {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Tell {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.num_members = reader.get_short();
                reader.get_byte();
                for _ in 0..self.num_members {
                    let mut guild_member = GuildMember::new();
                    guild_member.deserialize(&reader);
                    self.members.push(guild_member);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.num_members);
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.members.len() {
                    builder.append(&mut self.members[i].serialize());
                }
                builder.get()
            }
        }

        /// Get guild info reply
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Report {
            pub name: String,
            pub tag: String,
            pub create_date: String,
            pub description: String,
            pub wealth: String,
            pub ranks: [String; 9],
            pub num_staff: EOShort,
            pub staff: Vec<GuildStaff>,
        }

        impl Report {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Report {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.name = reader.get_break_string();
                self.tag = reader.get_break_string();
                self.create_date = reader.get_break_string();
                self.description = reader.get_break_string();
                self.wealth = reader.get_break_string();
                for i in 0..9 {
                    self.ranks[i] = reader.get_break_string();
                }
                self.num_staff = reader.get_short();
                reader.get_byte();
                for _ in 0..self.num_staff {
                    let mut guild_staff = GuildStaff::new();
                    guild_staff.deserialize(&reader);
                    self.staff.push(guild_staff);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.name);
                builder.add_break_string(&self.tag);
                builder.add_break_string(&self.create_date);
                builder.add_break_string(&self.description);
                builder.add_break_string(&self.wealth);
                for i in 0..self.ranks.len() {
                    builder.add_break_string(&self.ranks[i]);
                }
                builder.add_short(self.num_staff);
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.staff.len() {
                    builder.append(&mut self.staff[i].serialize());
                }
                builder.get()
            }
        }

        /// Joined guild info
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub recruiter_id: EOShort,
            pub guild_tag: String,
            pub guild_name: String,
            pub rank_name: String,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.recruiter_id = reader.get_short();
                reader.get_byte();
                self.guild_tag = reader.get_break_string();
                self.guild_name = reader.get_break_string();
                self.rank_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.recruiter_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.guild_tag);
                builder.add_break_string(&self.guild_name);
                builder.add_break_string(&self.rank_name);
                builder.get()
            }
        }

        /// Update guild rank
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub rank: EOChar,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.rank = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.rank);
                builder.get()
            }
        }

        /// Left the guild
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Kick {}

        impl Kick {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Kick {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }
    }

    pub mod Spell {
        use super::super::*;

        /// Nearby player chanting a spell
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub player_id: EOShort,
            pub spell_id: EOShort,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.spell_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_short(self.spell_id);
                builder.get()
            }
        }

        /// Nearby player self-casted a spell
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetSelf {
            pub player_id: EOShort,
            pub spell_id: EOShort,
            pub spell_heal_hp: EOInt,
            pub hp_percentage: EOChar,
            pub hp: EOShort,
            pub tp: EOShort,
        }

        impl TargetSelf {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetSelf {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.spell_id = reader.get_short();
                self.spell_heal_hp = reader.get_int();
                self.hp_percentage = reader.get_char();
                self.hp = reader.get_short();
                self.tp = reader.get_short();
                reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_short(self.spell_id);
                builder.add_int(self.spell_heal_hp);
                builder.add_char(self.hp_percentage);
                builder.add_short(self.hp);
                builder.add_short(self.tp);
                builder.add_short(1);
                builder.get()
            }
        }

        /// Nearby player raising their arm to cast a spell (vestigial)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub player_id: EOShort,
            pub direction: Direction,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.direction = Direction::from_char(reader.get_char());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.direction.to_char());
                builder.get()
            }
        }

        /// Nearby player(s) hit by a group heal spell from a player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetGroup {
            pub spell_id: EOShort,
            pub caster_id: EOShort,
            pub caster_tp: EOShort,
            pub spell_heal_hp: EOShort,
            pub players: Vec<GroupHealTargetPlayer>,
        }

        impl TargetGroup {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetGroup {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
                self.caster_id = reader.get_short();
                self.caster_tp = reader.get_short();
                self.spell_heal_hp = reader.get_short();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut group_heal_target_player = GroupHealTargetPlayer::new();
                    group_heal_target_player.deserialize(&reader);
                    self.players.push(group_heal_target_player);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.add_short(self.caster_id);
                builder.add_short(self.caster_tp);
                builder.add_short(self.spell_heal_hp);
                for i in 0..self.players.len() {
                    builder.append(&mut self.players[i].serialize());
                }
                builder.get()
            }
        }

        /// Nearby player hit by a heal spell from a player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetOther {
            pub victim_id: EOShort,
            pub caster_id: EOShort,
            pub caster_direction: Direction,
            pub spell_id: EOShort,
            pub spell_heal_hp: EOInt,
            pub hp_percentage: EOChar,
            pub hp: EOShort,
        }

        impl TargetOther {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetOther {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.victim_id = reader.get_short();
                self.caster_id = reader.get_short();
                self.caster_direction = Direction::from_char(reader.get_char());
                self.spell_id = reader.get_short();
                self.spell_heal_hp = reader.get_int();
                self.hp_percentage = reader.get_char();
                self.hp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.victim_id);
                builder.add_short(self.caster_id);
                builder.add_char(self.caster_direction.to_char());
                builder.add_short(self.spell_id);
                builder.add_int(self.spell_heal_hp);
                builder.add_char(self.hp_percentage);
                builder.add_short(self.hp);
                builder.get()
            }
        }
    }

    pub mod Trade {
        use super::super::*;

        /// Trade request from another player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub unk1: EOChar,
            pub player_id: EOShort,
            pub player_name: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.unk1 = reader.get_char();
                self.player_id = reader.get_short();
                self.player_name = reader.get_end_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.unk1);
                builder.add_short(self.player_id);
                builder.add_string(&self.player_name);
                builder.get()
            }
        }

        /// Trade window opens
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub player_id: EOShort,
            pub player_name: String,
            pub your_player_id: EOShort,
            pub your_name: String,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.player_name = reader.get_break_string();
                self.your_player_id = reader.get_short();
                self.your_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_break_string(&self.player_name);
                builder.add_short(self.your_player_id);
                builder.add_break_string(&self.your_name);
                builder.get()
            }
        }

        /// Trade updated (items changed)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub trade_data: TradeItemData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.trade_data.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.trade_data.serialize());
                builder.get()
            }
        }

        /// Trade updated (items changed while trade was acceptedd)
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Admin {
            pub trade_data: TradeItemData,
        }

        impl Admin {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Admin {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.trade_data.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.trade_data.serialize());
                builder.get()
            }
        }

        /// Trade items updated
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub trade_data: TradeItemData,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.trade_data.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.append(&mut self.trade_data.serialize());
                builder.get()
            }
        }

        /// Own agree state updated
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub agree_state: EOChar,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.agree_state = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.agree_state);
                builder.get()
            }
        }

        /// Partner agree state updated
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub player_id: EOShort,
            pub agree_state: EOChar,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.agree_state = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.agree_state);
                builder.get()
            }
        }

        /// Partner closed trade window
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Close {
            pub player_id: EOShort,
        }

        impl Close {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Close {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.get()
            }
        }
    }

    pub mod NPC {
        use super::super::*;

        /// Nearby NPC hit by a player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub player_id: EOShort,
            pub player_direction: EOChar,
            pub npc_index: EOShort,
            pub damage: EOThree,
            pub hp_percentage: EOShort,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.player_direction = reader.get_char();
                self.npc_index = reader.get_short();
                self.damage = reader.get_three();
                self.hp_percentage = reader.get_short();
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_char(self.player_direction);
                builder.add_short(self.npc_index);
                builder.add_three(self.damage);
                builder.add_short(self.hp_percentage);
                builder.add_char(1);
                builder.get()
            }
        }

        /// Nearby NPC killed by player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub killer_id: EOShort,
            pub killer_direction: EOChar,
            pub npc_index: EOShort,
            pub drop_index: EOShort,
            pub drop_id: EOShort,
            pub drop_coords: Coords,
            pub drop_amount: EOInt,
            pub damage: EOThree,
            pub experience: EOInt,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.killer_id = reader.get_short();
                self.killer_direction = reader.get_char();
                self.npc_index = reader.get_short();
                self.drop_index = reader.get_short();
                self.drop_id = reader.get_short();
                self.drop_coords.deserialize(&reader);
                self.drop_amount = reader.get_int();
                self.damage = reader.get_three();
                self.experience = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.killer_id);
                builder.add_char(self.killer_direction);
                builder.add_short(self.npc_index);
                builder.add_short(self.drop_index);
                builder.add_short(self.drop_id);
                builder.append(&mut self.drop_coords.serialize());
                builder.add_int(self.drop_amount);
                builder.add_three(self.damage);
                builder.add_int(self.experience);
                builder.get()
            }
        }

        /// Reply to request for information about nearby NPCs
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub num_npcs: EOShort,
            pub npcs: Vec<NPCMapInfo>,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.num_npcs = reader.get_short();
                for _ in 0..self.num_npcs {
                    let mut npc_map_info = NPCMapInfo::new();
                    npc_map_info.deserialize(&reader);
                    self.npcs.push(npc_map_info);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.num_npcs);
                for i in 0..self.npcs.len() {
                    builder.append(&mut self.npcs[i].serialize());
                }
                builder.get()
            }
        }

        /// Nearby NPC killed by player and you levelled up
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub killer_id: EOShort,
            pub killer_direction: EOChar,
            pub npc_index: EOShort,
            pub drop_index: EOShort,
            pub drop_id: EOShort,
            pub drop_coords: Coords,
            pub drop_amount: EOInt,
            pub damage: EOThree,
            pub experience: EOInt,
            pub level_up: LevelUpStats,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.killer_id = reader.get_short();
                self.killer_direction = reader.get_char();
                self.npc_index = reader.get_short();
                self.drop_index = reader.get_short();
                self.drop_id = reader.get_short();
                self.drop_coords.deserialize(&reader);
                self.drop_amount = reader.get_int();
                self.damage = reader.get_three();
                self.experience = reader.get_int();
                self.level_up.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.killer_id);
                builder.add_char(self.killer_direction);
                builder.add_short(self.npc_index);
                builder.add_short(self.drop_index);
                builder.add_short(self.drop_id);
                builder.append(&mut self.drop_coords.serialize());
                builder.add_int(self.drop_amount);
                builder.add_three(self.damage);
                builder.add_int(self.experience);
                builder.append(&mut self.level_up.serialize());
                builder.get()
            }
        }

        /// Killing all boss children
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Junk {
            pub npc_id: EOShort,
        }

        impl Junk {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Junk {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_id = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_id);
                builder.get()
            }
        }

        /// NPC chat message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Dialog {
            pub npc_index: EOShort,
            pub message: String,
        }

        impl Dialog {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Dialog {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_index = reader.get_short();
                self.message = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_index);
                builder.add_break_string(&self.message);
                builder.get()
            }
        }

        /// Main NPC update message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub pos: Vec<NPCUpdatePos>,
            pub attack: Vec<NPCUpdateAttack>,
            pub chat: Vec<NPCUpdateChat>,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut npc_update_pos = NPCUpdatePos::new();
                    npc_update_pos.deserialize(&reader);
                    self.pos.push(npc_update_pos);
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut npc_update_attack = NPCUpdateAttack::new();
                    npc_update_attack.deserialize(&reader);
                    self.attack.push(npc_update_attack);
                }
                reader.get_byte();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut npc_update_chat = NPCUpdateChat::new();
                    npc_update_chat.deserialize(&reader);
                    self.chat.push(npc_update_chat);
                }
                reader.get_byte();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.pos.len() {
                    builder.append(&mut self.pos[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.attack.len() {
                    builder.append(&mut self.attack[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.chat.len() {
                    builder.append(&mut self.chat[i].serialize());
                }
                builder.add_byte(EO_BREAK_CHAR);
                builder.get()
            }
        }
    }

    pub mod Cast {
        use super::super::*;

        /// Nearby NPC hit by a spell from a player
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub spell_id: EOShort,
            pub player_id: EOShort,
            pub player_direction: EOChar,
            pub npc_index: EOShort,
            pub damage: EOThree,
            pub hp_percentage: EOShort,
            pub caster_tp: EOShort,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
                self.player_id = reader.get_short();
                self.player_direction = reader.get_char();
                self.npc_index = reader.get_short();
                self.damage = reader.get_three();
                self.hp_percentage = reader.get_short();
                self.caster_tp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.add_short(self.player_id);
                builder.add_char(self.player_direction);
                builder.add_short(self.npc_index);
                builder.add_three(self.damage);
                builder.add_short(self.hp_percentage);
                builder.add_short(self.caster_tp);
                builder.get()
            }
        }

        /// Nearby NPC killed by player spell
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub spell_id: EOShort,
            pub killer_id: EOShort,
            pub killer_direction: EOChar,
            pub npc_index: EOShort,
            pub drop_index: EOShort,
            pub drop_id: EOShort,
            pub drop_coords: Coords,
            pub drop_amount: EOInt,
            pub damage: EOThree,
            pub caster_tp: EOShort,
            pub experience: EOInt,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
                self.killer_id = reader.get_short();
                self.killer_direction = reader.get_char();
                self.npc_index = reader.get_short();
                self.drop_index = reader.get_short();
                self.drop_id = reader.get_short();
                self.drop_coords.deserialize(&reader);
                self.drop_amount = reader.get_int();
                self.damage = reader.get_three();
                self.caster_tp = reader.get_short();
                self.experience = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.add_short(self.killer_id);
                builder.add_char(self.killer_direction);
                builder.add_short(self.npc_index);
                builder.add_short(self.drop_index);
                builder.add_short(self.drop_id);
                builder.append(&mut self.drop_coords.serialize());
                builder.add_int(self.drop_amount);
                builder.add_three(self.damage);
                builder.add_short(self.caster_tp);
                builder.add_int(self.experience);
                builder.get()
            }
        }

        /// Nearby NPC killed by player spell and you levelled up
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub spell_id: EOShort,
            pub killer_id: EOShort,
            pub killer_direction: EOChar,
            pub npc_index: EOShort,
            pub drop_index: EOShort,
            pub drop_id: EOShort,
            pub drop_coords: Coords,
            pub drop_amount: EOInt,
            pub damage: EOThree,
            pub caster_tp: EOShort,
            pub experience: EOInt,
            pub level_up: LevelUpStats,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.spell_id = reader.get_short();
                self.killer_id = reader.get_short();
                self.killer_direction = reader.get_char();
                self.npc_index = reader.get_short();
                self.drop_index = reader.get_short();
                self.drop_id = reader.get_short();
                self.drop_coords.deserialize(&reader);
                self.drop_amount = reader.get_int();
                self.damage = reader.get_three();
                self.caster_tp = reader.get_short();
                self.experience = reader.get_int();
                self.level_up.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.spell_id);
                builder.add_short(self.killer_id);
                builder.add_char(self.killer_direction);
                builder.add_short(self.npc_index);
                builder.add_short(self.drop_index);
                builder.add_short(self.drop_id);
                builder.append(&mut self.drop_coords.serialize());
                builder.add_int(self.drop_amount);
                builder.add_three(self.damage);
                builder.add_short(self.caster_tp);
                builder.add_int(self.experience);
                builder.append(&mut self.level_up.serialize());
                builder.get()
            }
        }
    }

    pub mod Quest {
        use super::super::*;

        /// NPC chat messages
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Report {
            pub npc_id: EOShort,
            pub messages: Vec<String>,
        }

        impl Report {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Report {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.npc_id = reader.get_short();
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.messages.push(reader.get_break_string());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.npc_id);
                for i in 0..self.messages.len() {
                    builder.add_break_string(&self.messages[i]);
                }
                builder.get()
            }
        }

        /// Quest selection dialog
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Dialog {
            pub quest_count: EOChar,
            pub vendor_id: EOShort,
            pub quest_id: EOShort,
            pub session_id: EOShort,
            pub dialog_id: EOShort,
            pub quest_entries: Vec<DialogQuestEntry>,
            pub entries: Vec<DialogEntry>,
        }

        impl Dialog {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Dialog {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.quest_count = reader.get_char();
                self.vendor_id = reader.get_short();
                self.quest_id = reader.get_short();
                self.session_id = reader.get_short();
                self.dialog_id = reader.get_short();
                reader.get_byte();
                for _ in 0..self.quest_count {
                    let mut dialog_quest_entry = DialogQuestEntry::new();
                    dialog_quest_entry.deserialize(&reader);
                    self.quest_entries.push(dialog_quest_entry);
                }
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut dialog_entry = DialogEntry::new();
                    dialog_entry.deserialize(&reader);
                    self.entries.push(dialog_entry);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.quest_count);
                builder.add_short(self.vendor_id);
                builder.add_short(self.quest_id);
                builder.add_short(self.session_id);
                builder.add_short(self.dialog_id);
                builder.add_byte(EO_BREAK_CHAR);
                for i in 0..self.quest_entries.len() {
                    builder.append(&mut self.quest_entries[i].serialize());
                }
                for i in 0..self.entries.len() {
                    builder.append(&mut self.entries[i].serialize());
                }
                builder.get()
            }
        }

        /// Quest history / progress reply
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ListData {
            Progress(ListProgress),
            History(ListHistory),
        }

        impl Default for ListData {
            fn default() -> Self {
                Self::Progress(ListProgress::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {
            pub page: QuestPage,
            pub num_quests: EOShort,
            pub data: ListData,
        }

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.page = QuestPage::from_char(reader.get_char());
                self.num_quests = reader.get_short();
                match self.page {
                    QuestPage::Progress => {
                        let mut progress = ListProgress::new();
                        progress.deserialize(&reader);
                        self.data = ListData::Progress(progress);
                    }
                    QuestPage::History => {
                        let mut history = ListHistory::new();
                        history.deserialize(&reader);
                        self.data = ListData::History(history);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.page.to_char());
                builder.add_short(self.num_quests);
                match &self.data {
                    ListData::Progress(progress) => {
                        builder.append(&mut progress.serialize());
                    }
                    ListData::History(history) => {
                        builder.append(&mut history.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ListProgress {
            pub quests: Vec<QuestProgressEntry>,
        }

        impl ListProgress {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ListProgress {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    let mut quest_progress_entry = QuestProgressEntry::new();
                    quest_progress_entry.deserialize(&reader);
                    self.quests.push(quest_progress_entry);
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.quests.len() {
                    builder.append(&mut self.quests[i].serialize());
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ListHistory {
            pub quests: Vec<String>,
        }

        impl ListHistory {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ListHistory {
            fn deserialize(&mut self, reader: &StreamReader) {
                while !reader.eof() && reader.peek_byte() != EO_BREAK_CHAR {
                    self.quests.push(reader.get_break_string());
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                for i in 0..self.quests.len() {
                    builder.add_break_string(&self.quests[i]);
                }
                builder.get()
            }
        }
    }

    pub mod Arena {
        use super::super::*;

        /// Arena is blocked message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Drop {}

        impl Drop {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Drop {
            fn deserialize(&mut self, reader: &StreamReader) {
                reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(b'N');
                builder.get()
            }
        }

        /// Arena start message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Use {
            pub num_players: EOChar,
        }

        impl Use {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Use {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.num_players = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.num_players);
                builder.get()
            }
        }

        /// Arena kill message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Spec {
            pub player_id: EOShort,
            pub direction: Direction,
            pub num_kills: EOInt,
            pub killer_name: String,
            pub victim_name: String,
        }

        impl Spec {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Spec {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                reader.get_byte();
                self.direction = Direction::from_char(reader.get_char());
                reader.get_byte();
                self.num_kills = reader.get_int();
                reader.get_byte();
                self.killer_name = reader.get_break_string();
                self.victim_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_char(self.direction.to_char());
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_int(self.num_kills);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.killer_name);
                builder.add_break_string(&self.victim_name);
                builder.get()
            }
        }

        /// Arena win message
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Accept {
            pub winner_name: String,
            pub num_kills: EOInt,
            pub killer_name: String,
            pub victim_name: String,
        }

        impl Accept {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Accept {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.winner_name = reader.get_break_string();
                self.num_kills = reader.get_int();
                reader.get_byte();
                self.killer_name = reader.get_break_string();
                self.victim_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_break_string(&self.winner_name);
                builder.add_int(self.num_kills);
                builder.add_byte(EO_BREAK_CHAR);
                builder.add_break_string(&self.killer_name);
                builder.add_break_string(&self.victim_name);
                builder.get()
            }
        }
    }

    pub mod Marriage {
        use super::super::*;

        /// Response from talking to a law NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub session_id: EOThree,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_three();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_three(self.session_id);
                builder.get()
            }
        }

        /// Reply to client Marriage-family packets
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum ReplyData {
            Success(ReplySuccess),
        }

        impl Default for ReplyData {
            fn default() -> Self {
                Self::Success(ReplySuccess::default())
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: MarriageReply,
            pub data: ReplyData,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = MarriageReply::from_short(reader.get_short());
                match self.reply_code {
                    MarriageReply::Success => {
                        let mut success = ReplySuccess::new();
                        success.deserialize(&reader);
                        self.data = ReplyData::Success(success);
                    }
                    _ => {}
                }
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                match &self.data {
                    ReplyData::Success(success) => {
                        builder.append(&mut success.serialize());
                    }
                    _ => {}
                }
                builder.get()
            }
        }

        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct ReplySuccess {
            pub gold_amount: EOInt,
        }

        impl ReplySuccess {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for ReplySuccess {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.gold_amount = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.gold_amount);
                builder.get()
            }
        }
    }

    pub mod Priest {
        use super::super::*;

        /// Response from talking to a priest NPC
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Open {
            pub session_id: EOInt,
        }

        impl Open {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Open {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_int();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.session_id);
                builder.get()
            }
        }

        /// Reply to client Priest-family packets
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub reply_code: PriestReply,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.reply_code = PriestReply::from_short(reader.get_short());
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.reply_code.to_short());
                builder.get()
            }
        }

        /// Wedding request
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Request {
            pub session_id: EOShort,
            pub partner_name: String,
        }

        impl Request {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Request {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.session_id = reader.get_short();
                self.partner_name = reader.get_break_string();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.session_id);
                builder.add_break_string(&self.partner_name);
                builder.get()
            }
        }
    }

    pub mod Recover {
        use super::super::*;

        /// HP/TP update
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub hp: EOShort,
            pub tp: EOShort,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.hp = reader.get_short();
                self.tp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.hp);
                builder.add_short(self.tp);
                builder.get()
            }
        }

        /// Stats update
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct List {
            pub class_id: EOShort,
            pub stats: CharacterStats3,
        }

        impl List {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for List {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.class_id = reader.get_short();
                self.stats.deserialize(&reader);
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.class_id);
                builder.append(&mut self.stats.serialize());
                builder.get()
            }
        }

        /// Karma/experience update
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Reply {
            pub experience: EOInt,
            pub karma: EOShort,
            /// "A value greater than 0 is 'new level' and indicates the player leveled up."
            pub level_up: EOChar,
            pub stat_points: EOShort,
            pub skill_points: EOShort,
        }

        impl Reply {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Reply {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.experience = reader.get_int();
                self.karma = reader.get_short();
                self.level_up = reader.get_char();
                self.stat_points = reader.get_short();
                self.skill_points = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_int(self.experience);
                builder.add_short(self.karma);
                builder.add_char(self.level_up);
                builder.add_short(self.stat_points);
                builder.add_short(self.skill_points);
                builder.get()
            }
        }

        /// Updated stats when levelling up from party experience
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct TargetGroup {
            pub stat_points: EOShort,
            pub skill_points: EOShort,
            pub max_hp: EOShort,
            pub max_tp: EOShort,
            pub max_sp: EOShort,
        }

        impl TargetGroup {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for TargetGroup {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.stat_points = reader.get_short();
                self.skill_points = reader.get_short();
                self.max_hp = reader.get_short();
                self.max_tp = reader.get_short();
                self.max_sp = reader.get_short();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.stat_points);
                builder.add_short(self.skill_points);
                builder.add_short(self.max_hp);
                builder.add_short(self.max_tp);
                builder.add_short(self.max_sp);
                builder.get()
            }
        }

        /// Nearby player gained HP
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Agree {
            pub player_id: EOShort,
            pub heal_hp: EOShort,
            pub hp_percentage: EOChar,
        }

        impl Agree {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Agree {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.player_id = reader.get_short();
                self.heal_hp = reader.get_short();
                self.hp_percentage = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_short(self.player_id);
                builder.add_short(self.heal_hp);
                builder.add_char(self.hp_percentage);
                builder.get()
            }
        }
    }

    pub mod Music {
        use super::super::*;

        /// Sound effect
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct Player {
            pub sound_id: EOChar,
        }

        impl Player {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl Serializeable for Player {
            fn deserialize(&mut self, reader: &StreamReader) {
                self.sound_id = reader.get_char();
            }

            fn serialize(&self) -> Vec<EOByte> {
                let mut builder = StreamBuilder::new();
                builder.add_char(self.sound_id);
                builder.get()
            }
        }
    }
}

// WARNING! This file was generated automatically. Do NOT edit it manually.
// https://github.com/sorokya/eo_protocol_parser
