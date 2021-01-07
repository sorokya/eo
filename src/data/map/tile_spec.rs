#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TileSpec {
    Wall = 0,
    ChairDown = 1,
    ChairLeft = 2,
    ChairRight = 3,
    ChairUp = 4,
    ChairDownRight = 5,
    ChairUpLeft = 6,
    ChairAll = 7,
    UnknownDoor = 8,
    Chest = 9,
    SpecUnknown1 = 10,
    SpecUnknown2 = 11,
    SpecUnknown3 = 12,
    SpecUnknown4 = 13,
    SpecUnknown5 = 14,
    SpecUnknown6 = 15,
    BankVault = 16,
    NPCBoundary = 17,
    MapEdge = 18,
    FakeWall = 19,
    Board1 = 20,
    Board2 = 21,
    Board3 = 22,
    Board4 = 23,
    Board5 = 24,
    Board6 = 25,
    Board7 = 26,
    Board8 = 27,
    Jukebox = 28,
    Jump = 29,
    Water = 30,
    SpecUnknown7 = 31,
    Arena = 32,
    AmbientSource = 33,
    Spikes1 = 34,
    Spikes2 = 35,
    Spikes3 = 36,
}

impl Default for TileSpec {
    fn default() -> Self {
        Self::Wall
    }
}
