

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

pub fn decode(buffer: &[u8], idx: usize) -> u32 {
    let first = buffer[idx];
    match first {
          0 ... 127 => first as u32,
        192 ... 223 => {
            let a = ((first & 0b_0001_1111u8) as u16);
            let b = ((buffer[idx + 1] & 0b_0011_1111u8) as u16);
            let n =  (a << 6) | b;
            
            // println!("{:#b}", a);
            // println!("{:#b}", b);
            // println!("{:#b}", n);
            n as u32
        },
        224 ... 239 => {
            let a = ((first & 0b_0000_1111u8) as u32);
            let b = ((buffer[idx + 1] & 0b_0011_1111u8) as u32);
            let c = ((buffer[idx + 2] & 0b_0011_1111u8) as u32);
            let n =  (((a << 6) | b) << 6) | c;

            // println!("{:#b}", a);
            // println!("{:#b}", b);
            // println!("{:#b}", c);
            // println!("{:#b}", n);
            n
        },
        240 ... 255 => {
            //      1 0000 000000000000110100001000
            // 0 0001 0000 0011 0100 1000
            let a = ((first & 0b_0000_0111u8) as u32);
            let b = ((buffer[idx + 1] & 0b_0011_1111u8) as u32);
            let c = ((buffer[idx + 2] & 0b_0011_1111u8) as u32);
            let d = ((buffer[idx + 3] & 0b_0011_1111u8) as u32);

            let n = ( ( ( ( (a << 6) | b) << 6) | c) << 6) | d;

            // println!("{:#b}", a);
            // println!("{:#b}", b);
            // println!("{:#b}", c);
            // println!("{:#b}", d);
            // println!("{:#b}", n);
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

    assert_eq!(std::char::from_u32(take(&buffer, 0)), Some('$'));
    assert_eq!(std::char::from_u32(take(&buffer, 1)), Some('¢'));
    assert_eq!(std::char::from_u32(take(&buffer, 3)), Some('ह'));
    assert_eq!(std::char::from_u32(take(&buffer, 6)), Some('€'));
    assert_eq!(std::char::from_u32(take(&buffer, 9)), Some('𐍈'));
}

