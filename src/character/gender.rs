/// describes the character gender
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Gender {
    Female = 0,
    Male = 1,
}

impl Default for Gender {
    fn default() -> Gender {
        Gender::Female
    }
}
