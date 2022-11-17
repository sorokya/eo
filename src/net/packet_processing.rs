use rand::prelude::*;

use super::{Action, Family};
use crate::data::{EOByte, EOInt};

/// flattens a packet [Family] and [Action] into a single [EOInt]
///
/// This is useful for identifying pairs of packet [Family]s, and [Action]s.
/// It is assumed there are a max of 255 actions per packet family.
///
/// # Example
/// ```
/// use eo::net::{Family, Action, packet_id_hash};
///
/// assert_eq!(packet_id_hash(Family::Connection, Action::Request), 257)
/// ```
pub fn packet_id_hash(family: Family, action: Action) -> EOInt {
    ((family as EOInt) << 8) | action as EOInt
}

/// used for encoding/decoding packet data
///
/// Packets are encrypted in three steps:
/// 1. Flipping
/// 2. Interleaving
/// 3. "dickwinding"
///
/// ## Flipping
/// Each byte of the packet has their most significant bits flipped
/// ```text
/// for i in 0..length {
///     bytes[i] ^= 0x80;
/// }
/// ```
///
/// ## Interleaving
/// Bytes are "woven" in to each-other e.g.
/// ```text
/// abcde -> aebdc
///   or
/// abcdef -> afbecd
/// ```
///
/// ## Dickwinding
/// This was named by Sausage and first implemented in the EOProxy project.
/// There are two numbers sent from the server to the client on connect
/// between 6 and 12 that represent a "send packet swap multiple"
/// and a "receive packet swap multiple".
///
/// Any two bytes next to each other in the packet data that are
/// divisible by that number are swapped.
///
/// For more details see [Packet](https://eoserv.net/wiki/wiki?page=Packet)
///
/// # Examples
///
/// Encoding a local chat message
/// ```
/// use eo::net::PacketProcessor;
///
/// // Talk_Report packet with a message
/// let mut packet_bytes = [
///     21, 18, 145, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33
/// ];
/// // Data: ‘Hello, world!
/// let processor = PacketProcessor::with_multiples(0, 6);
/// processor.encode(&mut packet_bytes);
/// assert_eq!(packet_bytes, [
///     149, 161, 146, 228, 17, 242, 200, 236, 229, 239, 236, 247, 236, 160, 239, 172
/// ]);
/// // Encoded data: •¡’äòÈìåïì÷ì ï¬
/// ```
///
#[derive(Default)]
pub struct PacketProcessor {
    pub decode_multiple: EOByte,
    pub encode_multiple: EOByte,
}

impl PacketProcessor {
    /// creates a new PacketProcessor with random encode/decode multiples
    pub fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            decode_multiple: rng.gen_range(6, 12),
            encode_multiple: rng.gen_range(6, 12),
        }
    }
    /// creates a new PacketProcessor with the provided encode/decode multiples
    pub fn with_multiples(decode_multiple: EOByte, encode_multiple: EOByte) -> Self {
        Self {
            decode_multiple,
            encode_multiple,
        }
    }
    /// sets the internal encode/decode multiples to the values provided
    pub fn set_multiples(&mut self, decode_multiple: EOByte, encode_multiple: EOByte) {
        self.decode_multiple = decode_multiple;
        self.encode_multiple = encode_multiple;
    }
    fn swap_multiples(&self, bytes: &mut [EOByte], multiple: EOByte) {
        let bytes_length = bytes.len();
        let mut sequence_length: usize = 0;
        for i in 0..bytes_length {
            if bytes[i] % multiple == 0 {
                sequence_length += 1;
            } else {
                if sequence_length > 1 {
                    let start = i - sequence_length;
                    let end = start + sequence_length - 1;
                    for i in 0..sequence_length / 2 {
                        bytes.swap(start + i, end - i);
                    }
                }
                sequence_length = 0;
            }
        }
        if sequence_length > 1 {
            let start = bytes_length - sequence_length;
            let end = start + sequence_length - 1;
            for i in 0..sequence_length / 2 {
                bytes.swap(start + i, end - i);
            }
        }
    }
    fn valid_for_decode(&self, bytes: &[EOByte]) -> bool {
        self.decode_multiple != 0 && bytes[0] != Action::Init as EOByte && bytes[1] != Family::Init as EOByte
    }

    fn valid_for_encode(&self, bytes: &[EOByte]) -> bool {
        self.encode_multiple != 0 && bytes[0] != Action::Init as EOByte && bytes[1] != Family::Init as EOByte
    }
    /// decodes a packet byte array in place
    ///
    /// Init_Init packets are ignored
    pub fn decode(&self, bytes: &mut [EOByte]) {
        if self.valid_for_decode(bytes) {
            let length = bytes.len();
            let mut buf: Vec<EOByte> = vec![0; length];
            let big_half = ((length + 1) / 2) as usize;
            let little_half = (length / 2) as usize;
            for i in 0..big_half {
                buf[i] = bytes[i * 2];
            }
            for i in 0..little_half {
                buf[length - 1 - i] = bytes[(i * 2) + 1];
            }
            for i in 0..length {
                bytes[i] = buf[i] ^ 0x80;
                if bytes[i] == 0 {
                    bytes[i] = 128;
                } else if bytes[i] == 128 {
                    bytes[i] = 0;
                }
            }
            self.swap_multiples(bytes, self.decode_multiple);
        }
    }
    /// encodes a packet byte array in place
    ///
    /// Init_Init packets are ignored
    #[allow(clippy::needless_range_loop)]
    pub fn encode(&self, bytes: &mut [EOByte]) {
        if self.valid_for_encode(bytes) {
            let length = bytes.len();
            let mut buf: Vec<EOByte> = vec![0; length];
            self.swap_multiples(bytes, self.encode_multiple);
            for i in 0..length {
                if bytes[i] == 0 {
                    bytes[i] = 128;
                } else if bytes[i] == 128 {
                    bytes[i] = 0;
                }
            }
            let big_half = ((length + 1) / 2) as usize;
            let little_half = (length / 2) as usize;
            for i in 0..big_half {
                buf[i * 2] = bytes[i];
            }
            for i in 0..little_half {
                buf[(i * 2) + 1] = bytes[length - 1 - i];
            }
            for i in 0..length {
                bytes[i] = buf[i] ^ 0x80;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, Family, PacketProcessor};
    #[test]
    fn packet_id_hash() {
        assert_eq!(super::packet_id_hash(Family::Init, Action::Init), 65535);
        assert_eq!(
            super::packet_id_hash(Family::Connection, Action::Request),
            257
        );
        assert_eq!(super::packet_id_hash(Family::Welcome, Action::Reply), 1283);
    }
    #[test]
    fn decode() {
        let mut bytes = [
            149, 161, 146, 228, 17, 242, 200, 236, 229, 239, 236, 247, 236, 160, 239, 172,
        ];
        let packet_processor = PacketProcessor::with_multiples(6, 0);
        packet_processor.decode(&mut bytes);
        assert_eq!(
            [21, 18, 145, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33],
            bytes
        );
    }
    #[test]
    fn encode() {
        let mut bytes = [
            3, 5, 2, 254, 2, 254, 2, 254, 254, 254, 6, 254, 249, 18, 77, 177, 145, 91, 254, 251,
            44, 36, 34, 239, 2, 2, 254, 254, 254, 52, 2, 48, 178, 184, 46, 33, 254, 155, 78, 178,
            222, 9, 254, 114, 111, 98, 111, 116, 255, 255, 255, 255, 3, 32, 32, 32, 1, 18, 197, 93,
            11, 254, 22, 2, 254, 254, 168, 254, 168, 254, 243, 254, 243, 254, 55, 254, 1, 254, 52,
            254, 242, 4, 25, 254, 29, 254, 11, 254, 12, 254, 8, 254, 42, 254, 9, 254, 8, 254, 14,
            254, 3, 254, 1, 254, 83, 254, 238, 254, 15, 2, 44, 254, 170, 2, 51, 254, 181, 254, 172,
            2, 203, 2, 43, 254, 43, 254, 41, 254, 41, 254, 55, 254, 55, 254, 1, 77, 254, 5, 254,
            25, 25, 1, 254, 1, 254, 1, 254, 3, 254, 1, 255,
        ];
        let packet_processor = PacketProcessor::with_multiples(0, 10);
        packet_processor.encode(&mut bytes);
        assert_eq!(
            [
                131, 127, 133, 129, 130, 126, 126, 131, 130, 126, 126, 129, 130, 126, 126, 129,
                126, 126, 126, 129, 134, 153, 126, 153, 121, 126, 146, 133, 205, 126, 49, 205, 17,
                129, 219, 126, 126, 183, 123, 126, 172, 183, 164, 126, 162, 169, 111, 126, 130,
                169, 130, 126, 126, 171, 126, 126, 126, 171, 180, 130, 130, 75, 176, 130, 50, 44,
                56, 126, 174, 53, 161, 126, 126, 179, 27, 130, 206, 42, 50, 126, 94, 172, 137, 130,
                126, 143, 242, 126, 239, 110, 226, 126, 239, 211, 244, 126, 127, 129, 127, 126,
                127, 131, 127, 126, 131, 142, 160, 126, 160, 136, 160, 126, 129, 137, 146, 126, 69,
                170, 221, 126, 139, 136, 126, 126, 150, 140, 130, 126, 126, 139, 126, 126, 40, 157,
                126, 126, 40, 153, 126, 132, 115, 114, 126, 126, 115, 180, 126, 126, 183, 129, 126
            ],
            bytes
        );
    }
    #[test]
    fn do_not_encode_init() {
        let mut bytes = [
            255, 255, 21, 191, 11, 1, 1, 29, 113, 10, 50, 57, 55, 50, 54, 53, 48, 55, 56,
        ];
        let packet_processor = PacketProcessor::with_multiples(0, 0);
        packet_processor.encode(&mut bytes);
        assert_eq!(
            [255, 255, 21, 191, 11, 1, 1, 29, 113, 10, 50, 57, 55, 50, 54, 53, 48, 55, 56],
            bytes
        );
    }
    #[test]
    fn do_not_decode_init() {
        let mut bytes = [
            255, 255, 21, 191, 11, 1, 1, 29, 113, 10, 50, 57, 55, 50, 54, 53, 48, 55, 56,
        ];
        let packet_processor = PacketProcessor::with_multiples(0, 0);
        packet_processor.decode(&mut bytes);
        assert_eq!(
            [255, 255, 21, 191, 11, 1, 1, 29, 113, 10, 50, 57, 55, 50, 54, 53, 48, 55, 56],
            bytes
        );
    }
    #[test]
    fn set_multiples() {
        let mut packet_processor = PacketProcessor::new();
        packet_processor.set_multiples(12, 6);
        assert_eq!(packet_processor.decode_multiple, 12);
        assert_eq!(packet_processor.encode_multiple, 6);
    }
}
