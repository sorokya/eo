/// EO represented byte
pub type EOByte = u8;
/// EO represented char
pub type EOChar = u8;
/// EO represented short
pub type EOShort = u16;
/// EO represented int using three bytes
pub type EOThree = u32;
/// EO represented int using four bytes
pub type EOInt = u32;

/// Maximum value EO stores in a single byte
pub const MAX1: EOInt = 253;
/// Maximum value EO stores across two bytes
pub const MAX2: EOInt = 64009;
/// Maximum value EO stores across three bytes
pub const MAX3: EOInt = 16194277;
/// Maximum value EO stores across four bytes
pub const MAX4: EOInt = 4097152080;
/// Special byte used in EO data streams for marking breaks in data
pub const EO_BREAK_CHAR: EOByte = 0xFF;

/// Returns an encoded byte array from `number`
///
/// EO uses a maximum of four bytes to represent a number
/// in a data stream.
///
/// The value is spread across the four bytes based on if the
/// value is greater than the defined maximum for each amount of bytes
/// see [MAX1], [MAX2], [MAX3]
///
/// The four bytes are initialized with a value of 254.
/// This is used later in [decode_number] for translating to a 0 value.
///
/// Bytes 4, 3, and 2 are set as follows if `number` is greater than or equal to
/// the corresponding `MAX` constant.
///
/// `bytes[x] = (number / MAX_x) + 1`
///
/// the number is then set to be the remainder of the division as follows
///
/// `number %= MAX_x`
///
/// Byte 1 is simply the remaining `number` plus one.
///
/// `bytes[0] = number + 1`
///
/// # Panics
///
/// This function will panic if `number` exceeds [MAX4].
///
/// # Examples
///
/// ## Number less than MAX1
/// ```
/// use eo::data::encode_number;
///
/// let result = encode_number(42);
/// assert_eq!(result, [43, 254, 254, 254]);
/// ```
/// since 42 is less than MAX1 it is simply incremented by 1
/// and the remaining bytes are set to 254
///
/// ## Number less than MAX2
/// ```
/// use eo::data::encode_number;
/// let result = encode_number(533);
/// assert_eq!(result, [28, 3, 254, 254]);
/// ```
///
/// since 533 is grater than MAX1 byte 2 is set to
///
/// `(533 / MAX1) + 1 // 3`
///
/// byte 1 is set to the the remainder + 1
///
/// `(533 % MAX1) + 1 // 28`
///
/// and the remaining bytes are set to 254
///
/// ## Number less than MAX3
/// ```
/// use eo::data::encode_number;
/// let result = encode_number(888888);
/// assert_eq!(result, [100, 225, 14, 254]);
/// ```
///
/// since 888888 is grater than MAX2 byte 3 is set to
///
/// `(888888 / MAX2) + 1 // 14`
///
/// byte 2 is set to
///
/// `((888888 % MAX2) / MAX1) + 1 // 225`
///
/// byte 1 is set to the the remainder + 1
///
/// `(888888 % MAX2 % MAX1) + 1 // 100`
///
/// and the last byte is set to 254
///
/// ## Number less than MAX4
/// ```
/// use eo::data::encode_number;
/// let result = encode_number(18994242);
/// assert_eq!(result, [15, 189, 44, 2]);
/// ```
///
/// since 18994242 is grater than MAX3 byte 4 is set to
///
/// `(18994242 / MAX3) + 1 // 2`
///
/// byte 3 is set to
///
/// `((18994242 % MAX3) / MAX2) + 1 // 44`
///
/// byte 2 is set to
///
/// `((18994242 % MAX3 % MAX2) / MAX1) + 1 // 189`
///
/// byte 1 is set to the the remainder + 1
///
/// `(18994242 % MAX3 % MAX2 % MAX1) + 1 // 15`
pub fn encode_number(mut number: EOInt) -> [EOByte; 4] {
    if number > MAX4 {
        panic!("Cannot encode {} it exceeds max value of {}", number, MAX4);
    }

    let mut bytes: [EOByte; 4] = [254, 254, 254, 254];
    let original_number = number;

    if original_number >= MAX3 {
        bytes[3] = (number / MAX3) as EOByte + 1;
        number %= MAX3;
    }

    if original_number >= MAX2 {
        bytes[2] = (number / MAX2) as EOByte + 1;
        number %= MAX2;
    }

    if original_number >= MAX1 {
        bytes[1] = (number / MAX1) as EOByte + 1;
        number %= MAX1;
    }

    bytes[0] = number as EOByte + 1;

    bytes
}

/// Returns a decoded number from an EO Byte array
///
/// EO uses a maximum of four bytes to represent a number
/// in a data stream.
///
/// You can provide any number of [EOByte]s in `bytes`
/// but only the first four are used.
///
/// If you provide less than four than the remaining bytes default to 254
///
/// The byte array is iterated over and any byte of 254 is changed to 1, and
/// each byte is decremented by 1.
///
/// The result is then calculated like so
///
/// `(b4 * MAX3) + (b3 * MAX2) + (b2 * MAX1) + b1`
///
/// # Examples
/// ```
/// use eo::data::decode_number;
/// let result = decode_number(&[43, 254, 254, 254]);
/// assert_eq!(result, 42);
/// ```
///
/// * bytes with `254` are swapped to `1`
/// `[43, 1, 1, 1]`
/// * bytes are decremented by 1
/// `[42, 0, 0, 0]`
/// * bytes are multiplied by MAX's and summed
/// `(0 * MAX3) + (0 * MAX2) + (0 * MAX1) + 42 == 42`
///
pub fn decode_number(bytes: &[EOByte]) -> EOInt {
    let mut data: [EOInt; 4] = [254, 254, 254, 254];
    for i in 0..4 {
        if bytes.len() > i && bytes[i] != 0 {
            data[i] = bytes[i].into();
        }
        if data[i] == 254 {
            data[i] = 1;
        }
        data[i] -= 1;
    }

    (data[3] * MAX3) + (data[2] * MAX2) + (data[1] * MAX1) + data[0]
}

pub fn encode_map_string(s: &str, length: usize) -> Bytes {
    let mut buf = BytesMut::with_capacity(length);
    buf.put_bytes(0xFF, length);
    for (i, c) in s.chars().enumerate() {
        buf[i] = c as u8;
    }

    let mut flippy = buf.len() % 2 == 1;
    for c in buf.iter_mut() {
        if flippy {
            if (0x22..=0x4F).contains(c) {
                *c = 0x71 - *c;
            } else if (0x50..=0x7E).contains(c) {
                *c = 0xCD - *c;
            }
        } else if (0x22..=0x7E).contains(c) {
            *c = 0x9F - *c;
        }
        flippy = !flippy;
    }

    buf.reverse();
    buf.freeze()
}

pub fn decode_map_string(bytes: Bytes) -> String {
    let mut buf = BytesMut::with_capacity(bytes.len());
    buf.put(bytes);
    buf.reverse();

    let mut chars = BytesMut::with_capacity(buf.len());
    chars.put_bytes(0xFF, buf.len());
    let mut flippy = buf.len() % 2 == 1;

    for (i, c) in buf.iter_mut().enumerate() {
        if *c == 0xFF {
            chars.truncate(i);
            break;
        }

        if flippy {
            if (0x22..=0x4F).contains(c) {
                *c = 0x71 - *c;
            } else if (0x50..=0x7E).contains(c) {
                *c = 0xCD - *c;
            }
        } else if (0x22..=0x7E).contains(c) {
            *c = 0x9F - *c;
        }
        chars[i] = *c;
        flippy = !flippy;
    }

    String::from_utf8_lossy(&chars).to_string()
}

mod stream_builder;
use bytes::{Bytes, BytesMut, BufMut};
pub use stream_builder::StreamBuilder;

mod stream_reader;
pub use stream_reader::StreamReader;

mod serializeable;
pub use serializeable::Serializeable;

#[cfg(test)]
mod tests {
    use super::{EOByte, EOInt};
    #[test]
    fn encode_number() {
        let mut test_numbers: Vec<(EOInt, [EOByte; 4])> = Vec::with_capacity(11);
        test_numbers.push((0, [1, 254, 254, 254]));
        test_numbers.push((5, [6, 254, 254, 254]));
        test_numbers.push((42, [43, 254, 254, 254]));
        test_numbers.push((253, [1, 2, 254, 254]));
        test_numbers.push((533, [28, 3, 254, 254]));
        test_numbers.push((9001, [147, 36, 254, 254]));
        test_numbers.push((64009, [1, 1, 2, 254]));
        test_numbers.push((888888, [100, 225, 14, 254]));
        test_numbers.push((7162531, [102, 228, 112, 254]));
        test_numbers.push((16194277, [1, 1, 1, 2]));
        test_numbers.push((18994242, [15, 189, 44, 2]));

        for d in test_numbers {
            let bytes = super::encode_number(d.0);
            assert_eq!(bytes, d.1);
        }
    }
    #[test]
    fn decode_number() {
        let mut test_numbers: Vec<(EOInt, [EOByte; 4])> = Vec::with_capacity(11);
        test_numbers.push((0, [1, 254, 254, 254]));
        test_numbers.push((5, [6, 254, 254, 254]));
        test_numbers.push((42, [43, 254, 254, 254]));
        test_numbers.push((253, [1, 2, 254, 254]));
        test_numbers.push((533, [28, 3, 254, 254]));
        test_numbers.push((9001, [147, 36, 254, 254]));
        test_numbers.push((64009, [1, 1, 2, 254]));
        test_numbers.push((888888, [100, 225, 14, 254]));
        test_numbers.push((7162531, [102, 228, 112, 254]));
        test_numbers.push((16194277, [1, 1, 1, 2]));
        test_numbers.push((18994242, [15, 189, 44, 2]));

        for d in test_numbers {
            let number = super::decode_number(&d.1);
            assert_eq!(number, d.0);
        }
    }
}
