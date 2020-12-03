/// differentiates what kind of information the user is requesting about a guild
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum GuildInfoType {
    Description = 1,
    Ranks = 2,
    Bank = 3,
}
