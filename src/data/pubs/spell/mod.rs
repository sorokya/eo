const ESF_DATA_SIZE: usize = 51;

mod spell_type;
pub use spell_type::SpellType;
mod spell_target_type;
pub use spell_target_type::SpellTargetType;
mod spell_target_restrict;
pub use spell_target_restrict::SpellTargetRestrict;
mod spell_record;
pub use spell_record::SpellRecord;
mod spell_file;
pub use spell_file::SpellFile;
