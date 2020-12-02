pub type EOByte = u8;
pub type EOChar = u8;
pub type EOShort = u16;
pub type EOThree = u32;
pub type EOInt = u32;

const MAX1: EOInt = 253;
const MAX2: EOInt = 64009;
const MAX3: EOInt = 16194277;

const EO_BREAK_CHAR: EOByte = 0xFF;

fn encode_number(mut number: EOInt) -> Vec<EOByte> {
    let mut bytes: Vec<EOByte> = vec![254; 4];
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

fn decode_number(buf: &[EOByte]) -> EOInt {
    println!("{:?}", buf);
    let mut data = vec![1; 4];
    for i in 0..4 {
        if buf.len() > i {
            data[i] = buf[i];
        }
        if data[i] == 254 {
            data[i] = 1;
        }
        if data[i] == 0 {
            data[i] = 128;
        }
        data[i] -= 1;
    }

    (data[3] as EOInt * MAX3)
        + (data[2] as EOInt * MAX2)
        + (data[1] as EOInt * MAX1)
        + data[0] as EOInt
}

mod stream_builder;
pub use stream_builder::StreamBuilder;

mod stream_reader;
pub use stream_reader::StreamReader;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_number() {
        let mut test_numbers: Vec<(EOInt, Vec<EOByte>)> = Vec::with_capacity(11);
        test_numbers.push((0, vec![1, 254, 254, 254]));
        test_numbers.push((5, vec![6, 254, 254, 254]));
        test_numbers.push((42, vec![43, 254, 254, 254]));
        test_numbers.push((253, vec![1, 2, 254, 254]));
        test_numbers.push((533, vec![28, 3, 254, 254]));
        test_numbers.push((9001, vec![147, 36, 254, 254]));
        test_numbers.push((64009, vec![1, 1, 2, 254]));
        test_numbers.push((888888, vec![100, 225, 14, 254]));
        test_numbers.push((7162531, vec![102, 228, 112, 254]));
        test_numbers.push((16194277, vec![1, 1, 1, 2]));
        test_numbers.push((18994242, vec![15, 189, 44, 2]));

        for d in test_numbers {
            let bytes = super::encode_number(d.0);
            assert_eq!(bytes, d.1);
        }
    }
    #[test]
    fn decode_number() {
        let mut test_numbers: Vec<(EOInt, Vec<EOByte>)> = Vec::with_capacity(11);
        test_numbers.push((0, vec![1, 254, 254, 254]));
        test_numbers.push((5, vec![6, 254, 254, 254]));
        test_numbers.push((42, vec![43, 254, 254, 254]));
        test_numbers.push((253, vec![1, 2, 254, 254]));
        test_numbers.push((533, vec![28, 3, 254, 254]));
        test_numbers.push((9001, vec![147, 36, 254, 254]));
        test_numbers.push((64009, vec![1, 1, 2, 254]));
        test_numbers.push((888888, vec![100, 225, 14, 254]));
        test_numbers.push((7162531, vec![102, 228, 112, 254]));
        test_numbers.push((16194277, vec![1, 1, 1, 2]));
        test_numbers.push((18994242, vec![15, 189, 44, 2]));

        for d in test_numbers {
            let number = super::decode_number(&d.1);
            assert_eq!(number, d.0);
        }
    }
}
