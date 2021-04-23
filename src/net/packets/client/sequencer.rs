use crate::data::{encode_number, EOByte, EOInt};

#[derive(Debug, Default)]
pub struct Sequencer {
    sequence_start: EOInt,
    sequence_increment: EOInt,
}

impl Sequencer {
    pub fn set_init_sequence(&mut self, s1: EOInt, s2: EOInt) {
        self.sequence_start = s1 * 7 - 11 + s2 - 2;
    }

    pub fn set_new_initial_sequence_number(&mut self, s1: EOInt, s2: EOInt) {
        self.sequence_start = s1 - s2;
    }

    fn get_next_sequence_number(&mut self) -> EOInt {
        self.sequence_start + self.sequence_increment
    }

    fn set_next_sequence_increment(&mut self) {
        self.sequence_increment = (self.sequence_increment + 1) % 10
    }

    pub fn get_sequence_bytes(&mut self) -> Vec<EOByte> {
        self.set_next_sequence_increment();
        let sequence_number = self.get_next_sequence_number();
        encode_number(sequence_number).to_vec()
    }
}
