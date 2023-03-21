use rand::Rng;

use crate::data::{EOChar, EOInt, EOShort};

#[derive(Debug, Default)]
pub struct ServerSequencer {
    sequence_start: EOInt,
    upcoming_sequence_start: EOInt,
    sequence: EOInt,
}

impl ServerSequencer {
    pub fn init_new_sequence(&mut self) {
        let mut rng = rand::thread_rng();
        self.sequence_start = rng.gen_range(0, 240);
    }

    pub fn get_sequence_start(&self) -> EOInt {
        self.sequence_start
    }

    pub fn ping_new_sequence(&mut self) {
        let mut rng = rand::thread_rng();
        self.upcoming_sequence_start = rng.gen_range(0, 240);
    }

    pub fn pong_new_sequence(&mut self) {
        self.sequence_start = self.upcoming_sequence_start;
    }

    pub fn get_init_sequence_bytes(&self) -> (EOShort, EOChar) {
        let mut rng = rand::thread_rng();
        let s1_max = (self.sequence_start + 13) / 7;
        let s1_min = std::cmp::max(0, (self.sequence_start as i32 - 252 + 13 + 6) / 7) as u32;
        let s1 = rng.gen_range(s1_min, s1_max);
        let s2 = (self.sequence_start as i32 - s1 as i32 * 7 + 13) as u32;
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
        self.sequence = (self.sequence + 1) % 10;
        self.sequence_start as EOInt + self.sequence
    }
}
