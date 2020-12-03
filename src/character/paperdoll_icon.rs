/// this is the icon that shows up when you view a players paperdoll
///
/// it describes wether the player is in a party or not,
/// and if they're a Game Master
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum PaperdollIcon {
    Normal = 0,
    GameMaster = 4,
    HighGameMaster = 5,
    Party = 6,
    GameMasterParty = 9,
    HighGameMasterParty = 10,
}
