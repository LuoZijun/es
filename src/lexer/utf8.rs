
// UTF-8 ranges and tags for encoding characters
const TAG_CONT: u8     = 0b1000_0000;
const TAG_TWO_B: u8    = 0b1100_0000;
const TAG_THREE_B: u8  = 0b1110_0000;
const TAG_FOUR_B: u8   = 0b1111_0000;
const MAX_ONE_B: u32   =     0x80;
const MAX_TWO_B: u32   =    0x800;
const MAX_THREE_B: u32 = 0x10000;


#[inline]
pub fn utf8_width(byte: u8) -> u8 {
    // https://en.wikipedia.org/wiki/UTF-8#Description
    match byte {
          0 ... 127 => 1,
        192 ... 223 => 2,
        224 ... 239 => 3,
        240 ... 255 => 4,
        _ => panic!("Invalid byte sequences"),
    }
}

#[inline]
pub fn len_utf8(ch: char) -> u8 {
    // https://en.wikipedia.org/wiki/UTF-8#Description
    let code = ch as u32;
    if code < MAX_ONE_B {
        1
    } else if code < MAX_TWO_B {
        2
    } else if code < MAX_THREE_B {
        3
    } else {
        4
    }
}

pub fn encode(c: char, buffer: &mut [u8; 4]) -> u8 {
    match c {
        '\u{0000}' ... '\u{007f}' => {
            buffer[0] = c as u8;

            1
        },
        '\u{0080}' ... '\u{07ff}' => {
            let n = c as u32;

            buffer[0] = (n >> 6 & 0x1F) as u8 | TAG_TWO_B;
            buffer[1] = (n & 0x3F) as u8 | TAG_CONT;;

            2
        },
        '\u{0800}' ... '\u{ffff}' => {
            let n = c as u32;

            buffer[0] = (n >> 12 & 0x0F) as u8 | TAG_THREE_B;
            buffer[1] = (n >>  6 & 0x3F) as u8 | TAG_CONT;
            buffer[2] = (n & 0x3F) as u8 | TAG_CONT;
            
            3
        },
        '\u{10000}' ... '\u{10ffff}' => {
            let n = c as u32;
            buffer[0] = (n >> 18 & 0x07) as u8 | TAG_FOUR_B;
            buffer[1] = (n >> 12 & 0x3F) as u8 | TAG_CONT;
            buffer[2] = (n >>  6 & 0x3F) as u8 | TAG_CONT;
            buffer[3] = (n & 0x3F) as u8 | TAG_CONT;

            4
        }
    }
}

pub fn decode(buffer: &[u8], idx: usize) -> u32 {
    let first = buffer[idx];
    match first {
          0 ... 127 => first as u32,
        192 ... 223 => {
            let a = (first & 0b_0001_1111u8) as u16;
            let b = (buffer[idx + 1] & 0b_0011_1111u8) as u16;
            let n =  (a << 6) | b;

            n as u32
        },
        224 ... 239 => {
            let a = (first & 0b_0000_1111u8) as u32;
            let b = (buffer[idx + 1] & 0b_0011_1111u8) as u32;
            let c = (buffer[idx + 2] & 0b_0011_1111u8) as u32;
            let n =  (((a << 6) | b) << 6) | c;

            n
        },
        240 ... 255 => {
            let a = (first & 0b_0000_0111u8) as u32;
            let b = (buffer[idx + 1] & 0b_0011_1111u8) as u32;
            let c = (buffer[idx + 2] & 0b_0011_1111u8) as u32;
            let d = (buffer[idx + 3] & 0b_0011_1111u8) as u32;

            let n = ( ( ( ( (a << 6) | b) << 6) | c) << 6) | d;

            n
        },
        _ => panic!("Invalid byte sequences"),
    }
}


#[test]
fn test_decode_utf8_seqs() {
    let buffer: Vec<u8> = vec![
        0b00100100,
        0b11000010, 0b10100010,
        0b11100000, 0b10100100, 0b10111001,
        0b11100010, 0b10000010, 0b10101100,
        0b11110000, 0b10010000, 0b10001101, 0b10001000,
    ];

    assert_eq!(std::char::from_u32(decode(&buffer, 0)), Some('$'));
    assert_eq!(std::char::from_u32(decode(&buffer, 1)), Some('¢'));
    assert_eq!(std::char::from_u32(decode(&buffer, 3)), Some('ह'));
    assert_eq!(std::char::from_u32(decode(&buffer, 6)), Some('€'));
    assert_eq!(std::char::from_u32(decode(&buffer, 9)), Some('𐍈'));
}

