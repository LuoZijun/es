use crate::unicode_xid::UnicodeXID;

// https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
pub static ASCII: [u8; 128] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
//        \t    \n                \r
//        TAB   LF    VT    FF    CR
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
//  SP
    b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'',
    b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
    b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z', b'[', b'\\', b']', b'^', b'_',
    b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
    b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
    b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
    b'x', b'y', b'z', b'{', b'|', b'}', b'~', 0x7f,

// //                                NEL
//     0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
//     0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
//     0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
//     0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
// //  NBSP
//     0xa0, 

    //       0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
    // 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
    // 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
    // 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
    // 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
    // 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
    // 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7,
    // 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
    // 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7,
    // 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
    // 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
    // 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];




// Format-Control Code Point
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-unicode-format-control-characters
pub const ZWNJ  : char = '\u{200C}';  // identifier part
pub const ZWJ   : char = '\u{200D}';  // identifier part
pub const ZWNBSP: char = '\u{FEFF}';  // whitespace

// White Space Code Points
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-white-space
pub const TAB: char  = '\u{0009}';    // '\t'
pub const VT: char   = '\u{000B}';    // '\v'
pub const FF: char   = '\u{000C}';    // '\f'
pub const SP: char   = '\u{0020}';    // ' '
pub const NBSP: char = '\u{00A0}';

// Line Terminator Code Points
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-line-terminators
pub const CR: char = '\u{000D}';   // \r
pub const LF: char = '\u{000A}';   // \n
pub const LS: char = '\u{2028}';   // 0xe2, 0x80, 0xa8
pub const PS: char = '\u{2029}';   // 0xe2, 0x80, 0xa9

pub const CR_LF: &[char]  = &[ CR, LF, ];


// Single Escape Character
pub const BACKSPACE: char = '\u{0008}';  // \b

pub const SLASH: u8 = b'/';


#[derive(Debug)]
pub enum Category {
    WhiteSpace,
    LineTerminator,
    // ECMAScript Punctuator start
    Punctuator,
    // Alphabetic: identifier_start or identifier_part
    Identifier,
    // 0 ... 9
    // a ... f
    // A ... F
    Numberic,
}


pub trait ESChar {
    fn is_es_whitespace(self) -> bool;
    fn is_es_line_terminator(self) -> bool;
    fn is_es_punctuator_start(self) -> bool;
    fn is_es_keyword_start(self) -> bool;
    fn is_es_keyword_break(self) -> bool;
    fn is_es_identifier_start(self) -> bool;
    fn is_es_identifier_part(self) -> bool;
    fn is_es_identifier_break(self) -> bool;
    fn is_es_binary_digit(self) -> bool;
    fn is_es_octal_digit(self) -> bool;
    fn is_es_hex_digit(self) -> bool;
    fn is_es_decimal_digit(self) -> bool;
    fn is_es_digit_break(self) -> bool;
    fn is_es_single_escape_character(self) -> bool;
}


impl ESChar for char {
    #[inline]
    fn is_es_line_terminator(self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-line-terminators
        match self {
            CR | LF | LS | PS => true,
            _ => false,
        }
    }

    #[inline]
    fn is_es_whitespace(self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-white-space
        match self {
            TAB | VT | FF | SP | NBSP | ZWNBSP => true,
            _ => {
                if self.is_es_line_terminator() {
                    false
                } else {
                    self.is_whitespace()
                }
            },
        }
    }

    #[inline]
    fn is_es_punctuator_start(self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-punctuators
        match self {
              '{' | '}' | '(' | ')' | '[' | ']' 
            | '.' | ';' | ':' | ',' | '?'
            | '/' | '!' | '|' | '&' | '^'
            | '<' | '>' | '~' | '%' | '=' 
            | '+' | '-' | '*' | '`' | '#' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_es_keyword_start(self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-keywords
        match self {
              'a' | 'b' | 'c' | 'd' | 'e' | 'f' 
            | 'i' | 'l' | 'n' | 'p' | 'r' | 's' 
            | 't' | 'v' | 'w' | 'y' => true,
            _ => false
        }
    }

    #[inline]
    fn is_es_keyword_break(self) -> bool {
        self.is_es_identifier_break()
    }

    #[inline]
    fn is_es_identifier_start(self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-IdentifierStart
        match self {
            '_' | '$' => true,
            _ => UnicodeXID::is_xid_start(self),
        }
    }

    #[inline]
    fn is_es_identifier_part(self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-IdentifierPart
        match self {
            '$' | ZWNJ | ZWJ => true,
            _ => UnicodeXID::is_xid_continue(self),
        }
    }

    #[inline]
    fn is_es_identifier_break(self) -> bool {
        if self.is_es_identifier_start() || self.is_es_identifier_part() {
            return false;
        }

        if self.is_es_line_terminator() 
            || self.is_es_whitespace() 
            || self.is_es_punctuator_start() {
            true
        } else {
            false
        }
    }

    #[inline]
    fn is_es_binary_digit(self) -> bool {
        self == '0' || self == '1'
    }

    #[inline]
    fn is_es_octal_digit(self) -> bool {
        match self {
            '0' ... '7' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_es_hex_digit(self) -> bool {
        self.is_ascii_hexdigit()
    }

    #[inline]
    fn is_es_decimal_digit(self) -> bool {
        self.is_ascii_digit()
    }

    #[inline]
    fn is_es_digit_break(self) -> bool {
        if self.is_es_line_terminator() || self.is_es_whitespace() {
            return true;
        }

        match self {
            '\0' | '}' | ')' | ']'
            | ';' | ':' | ',' | '?'
            | '/' | '|' | '&' | '^'
            | '<' | '>' | '='
            | '+' | '-' | '*' | '%' => true,
            _ => false,
        }
    }

    #[inline]
    fn is_es_single_escape_character(self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-SingleEscapeCharacter
        // one of ' " \ b f n r t v
        match self {
            '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => true ,
            _ => false,
        }
    }
}
