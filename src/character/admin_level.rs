/// describes a characters admin level
///
/// new characters default as Player
///
/// The Guide, Guardian, GameMaster, HighGameMaster options
/// are usually used as a rank system for admins within the
/// game.
///
/// You could use these levels to restrict access to certain
/// commands and features within the game to only specified
/// admin levels.
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum AdminLevel {
    Player = 0,
    Guide = 1,
    Guardian = 2,
    GameMaster = 3,
    HighGameMaster = 4,
}
