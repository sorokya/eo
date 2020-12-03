/// used to differentiate different parts of the players "avatar"
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum AvatarSlot {
    Clothes = 1,
    Hair = 2,
    HairColor = 3,
}
