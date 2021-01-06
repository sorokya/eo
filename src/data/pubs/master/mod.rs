const EMF_DATA_SIZE: usize = 7;
const EMF_SKILL_DATA_SIZE: usize = 28;

mod master_record;
pub use master_record::MasterRecord;
mod master_skill_record;
pub use master_skill_record::MasterSkillRecord;
mod master_file;
pub use master_file::MasterFile;
