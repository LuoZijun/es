
// https://github.com/open-i18n/rust-unic/blob/master/unic/ucd/common/tables/white_space.rsv
// const WHITE_SPACES: &[ char ] = [
//     //  TAB         LF          VT          FF          CR
//     //   \t         \n          \v          \f          \r
//     '\u{0009}', '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}',
//     //   SP
//     '\u{0020}',
//     '\u{0085}',
//     //  NBSP
//     '\u{00A0}',
//     '\u{1680}',
//     '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}',
//     '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', 
//     '\u{200A}', 
//     //   LS         PS
//     '\u{2028}', '\u{2029}', 
//     '\u{202F}',
//     '\u{205F}',
//     '\u{3000}',
//     // ZWNBSP
//     '\u{FEFF}',
// ];


// White Space Code Points
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-white-space
pub const TAB: char  = '\u{0009}';    // '\t'
pub const VT: char   = '\u{000B}';    // '\v'
pub const FF: char   = '\u{000C}';    // '\f'
pub const SP: char   = '\u{0020}';    // ' '
pub const NBSP: char = '\u{00A0}';
pub const ZWNBSP: char = '\u{FEFF}';  // whitespace

// Line Terminator Code Points
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-line-terminators
pub const CR: char = '\u{000D}';   // \r
pub const LF: char = '\u{000A}';   // \n
pub const LS: char = '\u{2028}';   // 0xe2, 0x80, 0xa8
pub const PS: char = '\u{2029}';   // 0xe2, 0x80, 0xa9
// pub const CR_LF: &[char]  = &[ CR, LF, ];


#[inline]
pub fn is_es_line_terminator(ch: char) -> bool {
    // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-line-terminators
    match ch {
        CR | LF | LS | PS => true,
        _ => false,
    }
}

#[inline]
pub fn is_es_whitespace(ch: char) -> bool {
    // // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-white-space
    // https://github.com/open-i18n/rust-unic/blob/master/unic/ucd/common/tables/white_space.rsv
    match ch {
        TAB | VT | FF | SP | NBSP | ZWNBSP
        | '\u{0085}'
        | '\u{1680}'
        | '\u{2000}' ..= '\u{200A}'
        | '\u{202F}'
        | '\u{205F}'
        | '\u{3000}' => true,
        _ => false,
    }
}
