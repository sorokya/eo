/// Used to verify the server's authenticity to the client.
pub fn stupid_hash(mut value: crate::data::EOThree) -> crate::data::EOThree {
    value += 1;
    110905 + (value % 9 + 1) * ((11092004 - value) % ((value % 11 + 1) * 119)) * 119 + value % 2004
}

mod packet_processor;
pub use packet_processor::PacketProcessor;

mod client_sequencer;
pub use client_sequencer::ClientSequencer;

mod server_sequencer;
pub use server_sequencer::ServerSequencer;
