/// describes the kinds of emotions a character can display
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Emote {
    Happy = 0,
    Depressed = 1,
    Sad = 3,
    Angry = 4,
    Confused = 5,
    Surprised = 6,
    Hearts = 7,
    Moon = 8,
    Suicidal = 9,
    Embarrassed = 10,
    Drunk = 11,
    Trade = 12,
    LevelUp = 13,
    Playful = 14,
}
