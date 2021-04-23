use rand::Rng;

use crate::data::{EOChar, EOInt, EOShort};

#[derive(Debug, Default)]
pub struct Sequencer {
    sequence_start: EOInt,
    upcoming_sequence_start: EOInt,
    sequence: EOInt,
}

impl Sequencer {
    pub fn init_new_sequence(&mut self) {
        let mut rng = rand::thread_rng();
        self.sequence_start = rng.gen_range(0, 1757);
    }

    pub fn too_big_for_account_reply(&self) -> bool {
        self.sequence_start > 240
    }

    pub fn account_reply_new_sequence(&mut self) {
        let mut rng = rand::thread_rng();
        self.sequence_start = rng.gen_range(0, 240);
    }

    pub fn ping_new_sequence(&mut self) {
        let mut rng = rand::thread_rng();
        self.upcoming_sequence_start = rng.gen_range(0, 1757);
    }

    pub fn pong_new_sequence(&mut self) {
        self.sequence_start = self.upcoming_sequence_start;
    }

    pub fn get_init_sequence_bytes(&self) -> (EOShort, EOChar) {
        let mut rng = rand::thread_rng();
        let s1_max = (self.sequence_start + 13) / 7;
        let s1_min = std::cmp::max(0, (self.sequence_start - 252 + 13 + 6) / 7);
        let s1 = rng.gen_range(s1_min, s1_max);
        let s2 = self.sequence_start - s1 * 7 + 13;
        (s1 as EOShort, s2 as EOChar)
    }

    pub fn get_update_sequence_bytes(&self) -> (EOShort, EOChar) {
        let mut rng = rand::thread_rng();
        let s1_max = self.upcoming_sequence_start + 252;
        let s1_min = self.upcoming_sequence_start;
        let s1 = rng.gen_range(s1_min, s1_max);
        let s2 = s1 - self.upcoming_sequence_start;
        (s1 as EOShort, s2 as EOChar)
    }

    pub fn gen_sequence(&mut self) -> EOInt {
        let result = self.sequence_start as EOInt + self.sequence;
        self.sequence = (self.sequence + 1) % 10;
        result
    }

    pub fn gen_upcoming_sequence(&mut self) -> EOInt {
        let result = self.upcoming_sequence_start as EOInt + self.sequence;
        self.sequence = (self.sequence + 1) % 10;
        result
    }
}
