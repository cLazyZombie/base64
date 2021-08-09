fn main() {}

fn encode(s: &str) -> String {
    //let encoded: Vec<char> = s
    let s: String = s
        .as_bytes()
        .chunks(3)
        .map(rawbyte_x3_to_base64_x4)
        .flat_map(|bits| bits.iter().map(base64_to_char).collect::<Vec<_>>())
        .collect();
    s
}

fn rawbyte_x3_to_base64_x4(b: &[u8]) -> [u8; 4] {
    let mut result = [0b1000_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000];

    match b.len() {
        1 => {
            result[0] = (b[0] & 0b_1111_1100) >> 2;
            result[1] = (b[0] & 0b_0000_0011) << 4;
        }
        2 => {
            result[0] = (b[0] & 0b_1111_1100) >> 2;
            result[1] = ((b[0] & 0b_0000_0011) << 4) + ((b[1] & 0b_1111_0000) >> 4);
            result[2] = (b[1] & 0b_0000_1111) << 2;
        }
        3 => {
            result[0] = (b[0] & 0b_1111_1100) >> 2;
            result[1] = ((b[0] & 0b_0000_0011) << 4) + ((b[1] & 0b_1111_0000) >> 4);
            result[2] = ((b[1] & 0b_0000_1111) << 2) + ((b[2] & 0b_1100_0000) >> 6);
            result[3] = b[2] & 0b_0011_1111;
        }
        _ => {}
    }

    result
}

fn decode(s: &str) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    let s = chars
        .chunks(4)
        .flat_map(base64char_x4_to_rawbytes)
        .map(|u| u as char)
        .collect();
    s
}

fn base64_to_char(b: &u8) -> char {
    if b & 0b_1000_0000 != 0 {
        '='
    } else {
        match *b {
            0..=25 => char::from_u32((*b + b'A') as u32).unwrap(),
            26..=51 => char::from_u32((*b - 26 + b'a') as u32).unwrap(),
            52..=61 => char::from_u32((*b - 52 + b'0') as u32).unwrap(),
            62 => '+',
            63 => '/',
            _ => panic!("invalid b {}", *b),
        }
    }
}

fn base64char_x4_to_rawbytes(chars: &[char]) -> Vec<u8> {
    assert_eq!(chars.len(), 4);

    if chars[2] == '=' && chars[3] == '=' {
        let b1 = ((base64char_to_byte(chars[0]) & 0b_11_1111) << 2)
            + ((base64char_to_byte(chars[1]) & 0b_11_0000) >> 4);
        vec![b1]
    } else if chars[3] == '=' {
        let b1 = ((base64char_to_byte(chars[0]) & 0b_11_1111) << 2)
            + ((base64char_to_byte(chars[1]) & 0b_11_0000) >> 4);
        let b2 = ((base64char_to_byte(chars[1]) & 0b_00_1111) << 4)
            + ((base64char_to_byte(chars[2]) & 0b_11_1100) >> 2);
        vec![b1, b2]
    } else {
        let b1 = ((base64char_to_byte(chars[0]) & 0b_11_1111) << 2)
            + ((base64char_to_byte(chars[1]) & 0b_11_0000) >> 4);
        let b2 = ((base64char_to_byte(chars[1]) & 0b_00_1111) << 4)
            + ((base64char_to_byte(chars[2]) & 0b_11_1100) >> 2);
        let b3 = ((base64char_to_byte(chars[2]) & 0b_00_0011) << 6)
            + (base64char_to_byte(chars[3]) & 0b_00111111);
        vec![b1, b2, b3]
    }
}

fn base64char_to_byte(c: char) -> u8 {
    match c {
        'A'..='Z' => (c as u32 - 'A' as u32) as u8,
        'a'..='z' => (c as u32 - 'a' as u32) as u8 + 26,
        '0'..='9' => (c as u32 - '0' as u32) as u8 + 52,
        '+' => 62,
        '/' => 63,
        _ => panic!("invalid c {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let origin = "Pancake";
        let encoded = encode(origin);
        assert_eq!("UGFuY2FrZQ==", encoded);
    }

    #[test]
    fn bytes3_to_64bit_4_test() {
        let input = [b'e'];
        let result = rawbyte_x3_to_base64_x4(&input);
        assert_eq!(0b011001, result[0]);
        assert_eq!(0b010000, result[1]);

        let input = [b'e', b'f'];
        let result = rawbyte_x3_to_base64_x4(&input);
        assert_eq!(0b011001, result[0]);
        assert_eq!(0b010110, result[1]);
        assert_eq!(0b011000, result[2]);

        let input = [b'e', b'f', b'g'];
        let result = rawbyte_x3_to_base64_x4(&input);
        assert_eq!(0b011001, result[0]);
        assert_eq!(0b010110, result[1]);
        assert_eq!(0b011001, result[2]);
        assert_eq!(0b100111, result[3]);

        let input = [b'P', b'a', b'n'];
        let result = rawbyte_x3_to_base64_x4(&input);
        assert_eq!(0b010100, result[0]);
        assert_eq!(0b000110, result[1]);
        assert_eq!(0b000101, result[2]);
        assert_eq!(0b101110, result[3]);
    }

    #[test]
    fn base64_4chars_to_bytes_test() {
        assert_eq!(
            base64char_x4_to_rawbytes(&['Y', '2', 'F', 'r']),
            vec![b'c', b'a', b'k']
        );
        assert_eq!(
            base64char_x4_to_rawbytes(&['U', 'G', 'F', 'u']),
            vec![b'P', b'a', b'n']
        );
    }

    #[test]
    fn decode_test() {
        assert_eq!(decode("UGFuY2FrZQ=="), "Pancake");
    }
}
