use lexer::eschar::{
    ESChar,
    CR, LF, LS, PS,
    TAB, VT, FF, SP, NBSP, ZWNBSP,
    BACKSPACE, 
};


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EscapeSequenceKind {
    // CharacterEscapeSequence
    // NullCharacter , 0 [lookahead ∉ DecimalDigit]
    // HexEscapeSequence
    // UnicodeEscapeSequence
    String,
    // LineTerminator
    // CharacterEscapeSequence
    // NullCharacter
    // HexEscapeSequence
    // UnicodeEscapeSequence
    TemplateString,
    // UnicodeEscapeSequence
    Identifier,
    JSXText,
}


#[inline]
pub fn single_escape_character(c: char) -> char {
    // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-SingleEscapeCharacter
    // one of ' " \ b f n r t v
    debug_assert_eq!(c.is_es_single_escape_character(), true);
    match c {
        '\'' => c,
        '"'  => c,
        '\\' => c,
        'b'  => BACKSPACE,
        'f'  => FF,        // FORM FEED
        'n'  => LF,        // LINE FEED
        'r'  => CR,        // CARRIAGE RETURN
        't'  => TAB,       // CHARACTER TABULATION
        'v'  => VT,        // LINE TABULATION
        _    => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EscapeErrorKind {
    Overflow,
    InvalidDigit,
    InvalidCodePoint,
    InvalidEscapeSequence,
    InvalidCharacter,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct EscapeError {
    kind: EscapeErrorKind,
    offset: usize,
}

impl EscapeError {
    pub const fn new(kind: EscapeErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    pub fn kind(&self) -> &EscapeErrorKind {
        &self.kind
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}


fn hex_to_char(digits: &[char], pre_offset: usize) -> Result<char, EscapeError> {
    let digits_len = digits.len() as u32;

    if digits_len == 0 {
        return Err(EscapeError::new(EscapeErrorKind::InvalidDigit, pre_offset));
    }

    let radix = 16u32;

    let mut offset: u32 = 0;
    let mut acc: u32 = 0;

    while offset < digits_len {
        let digit: u32 = digits[offset as usize]
                            .to_digit(radix)
                            .ok_or(EscapeError::new(EscapeErrorKind::InvalidDigit, pre_offset + offset as usize))?;

        let idx = digits_len - 1 - offset;
        match radix.checked_pow(idx) {
            Some(n) => match digit.checked_mul(n) {
                Some(n) => {
                    // acc += digit * radix.pow(idx);
                    acc += n;
                },
                None => {
                    return Err(EscapeError::new(EscapeErrorKind::Overflow, pre_offset + offset as usize));
                },
            },
            None => {
                return Err(EscapeError::new(EscapeErrorKind::Overflow, pre_offset + offset as usize));
            },
        }
        
        offset += 1;
    }

    std::char::from_u32(acc).ok_or(EscapeError::new(EscapeErrorKind::InvalidCodePoint, pre_offset + offset as usize))
}

#[inline]
pub fn escape(input: &[char], kind: EscapeSequenceKind) -> Result<Vec<char>, EscapeError> {
    let input_len: usize = input.len();
    let mut output: Vec<char> = Vec::with_capacity(input_len);
    let mut idx: usize = 0;
    
    loop {
        if idx >= input_len {
            break;
        }

        let c = input[idx];

        match c {
            '\\' => {
                idx += 1;
                let c = input.get(idx).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?;
                match c {
                    '0' => {
                        // NullCharacter
                        // \0
                        if kind == EscapeSequenceKind::Identifier {
                            return Err(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx));
                        }

                        // lookahead
                        match input.get(idx+1) {
                            Some(c) => {
                                if c.is_ascii_digit() {
                                    return Err(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx));
                                }
                            },
                            None => { }
                        }

                        output.push('\0');
                    }
                    'x' => {
                        // HexEscapeSequence
                        // \x HexDigit HexDigit
                        if kind == EscapeSequenceKind::Identifier {
                            return Err(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx));
                        }

                        idx += 1;
                        let buffer: [char; 2] = [
                            *(input.get(idx).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?),
                            *(input.get(idx+1).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?),
                        ];

                        let c2 = hex_to_char(&buffer, idx)?;
                        idx += 1;

                        output.push(c2);
                    },
                    'u' => {
                        // UnicodeEscapeSequence
                        // \u HexDigit HexDigit HexDigit HexDigit
                        // \u { HexDigit HexDigit HexDigit ... }
                        idx += 1;
                        let c = input.get(idx).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?;

                        if c != &'{' {
                            let buffer: [char; 4] = [
                                *(input.get(idx).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?),
                                *(input.get(idx+1).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?),
                                *(input.get(idx+2).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?),
                                *(input.get(idx+3).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?),
                            ];
                            
                            let c2 = hex_to_char(&buffer, idx)?;
                            idx += 3;

                            output.push(c2);
                        } else {
                            idx += 1;

                            let start = idx;

                            #[allow(unused_assignments)]
                            let mut end = idx;

                            loop {
                                idx += 1;
                                let c2 = input.get(idx).ok_or(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx))?;
                                if c2 == &'}' {
                                    end = idx;
                                    break;
                                }
                            }
                            
                            let c2 = hex_to_char(&input[start..end], start)?;

                            output.push(c2);
                        }
                    },
                    _ => {
                        if c.is_es_line_terminator() {
                            if kind == EscapeSequenceKind::Identifier {
                                return Err(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx));
                            }

                            // LineTerminatorSequence
                            if c == &CR {
                                // lookahead LF
                                match input.get(idx+1) {
                                    Some(&LF) => {
                                        idx += 1;

                                        output.push(CR);
                                        output.push(LF);
                                    },
                                    _ => {
                                        output.push(CR);
                                    }
                                }
                            } else {
                                output.push(*c);
                            }
                        } else if c.is_es_single_escape_character() {
                            // SingleEscapeCharacter
                            if kind == EscapeSequenceKind::Identifier {
                                return Err(EscapeError::new(EscapeErrorKind::InvalidEscapeSequence, idx));
                            }

                            output.push(single_escape_character(*c));
                        } else {
                            // NonEscapeCharacter
                            output.push(*c);
                        }
                    }
                }
            },
            _ => {
                if c.is_es_line_terminator() {
                    // LineTerminator

                    if kind != EscapeSequenceKind::TemplateString {
                        // NOTE: Only TemplateString allow LineTerminator
                        return Err(EscapeError::new(EscapeErrorKind::InvalidCharacter, idx));
                    }

                    if c == CR {
                        // lookahead LF
                        match input.get(idx+1) {
                            Some(&LF) => {
                                idx += 1;

                                output.push(CR);
                                output.push(LF);
                            },
                            _ => {
                                output.push(CR);
                            }
                        }
                    } else {
                        output.push(c);
                    }
                } else {
                    // Normal SourceCode
                    output.push(c);
                }
            }
        }

        idx += 1;
    }

    Ok(output)
}


#[inline]
pub fn unescape_string(input: &[char]) -> Result<Vec<char>, EscapeError> {
    escape(input, EscapeSequenceKind::String)
}

#[inline]
pub fn unescape_template(input: &[char]) -> Result<Vec<char>, EscapeError> {
    escape(input, EscapeSequenceKind::TemplateString)
}

#[inline]
pub fn unescape_identifier(input: &[char]) -> Result<Vec<char>, EscapeError> {
    escape(input, EscapeSequenceKind::Identifier)
}


#[test]
fn test_unescape_string() {
    let input = r#"我\u{9999999999969}"#.chars().collect::<Vec<char>>();
    let output = unescape_string(&input);
    assert_eq!(output.is_err(), true);

    let input = r#"我\u{69}"#.chars().collect::<Vec<char>>();
    let output = unescape_string(&input);
    assert_eq!(output, Ok("我i".chars().collect::<Vec<char>>()));

    let input = r#"我\u{69}
"#.chars().collect::<Vec<char>>();
    let output = unescape_string(&input);
    assert_eq!(output.is_err(), true);

    let input = r#"我\u{69}\n\
"#.chars().collect::<Vec<char>>();
    let output = unescape_string(&input);
    assert_eq!(output, Ok("我i\n\n".chars().collect::<Vec<char>>()));
}

#[test]
fn test_unescape_template() {
    let input = r#"我\u{69}\u0069\x69
\n"#.chars().collect::<Vec<char>>();
    let output = unescape_template(&input);
    assert_eq!(output, Ok("我iii\n\n".chars().collect::<Vec<char>>()));
}

#[test]
fn test_unescape_identifier() {
    let input = r#"我\u{69}\u0069"#.chars().collect::<Vec<char>>();
    let output = unescape_identifier(&input);
    assert_eq!(output, Ok("我ii".chars().collect::<Vec<char>>()));

    let input = r#"\x69"#.chars().collect::<Vec<char>>();
    let output = unescape_identifier(&input);
    assert_eq!(output.is_err(), true);

    let input = r#"\n"#.chars().collect::<Vec<char>>();
    let output = unescape_identifier(&input);
    assert_eq!(output.is_err(), true);

    let input = r#"\a"#.chars().collect::<Vec<char>>();
    let output = unescape_identifier(&input);
    assert_eq!(output, Ok("a".chars().collect::<Vec<char>>()));
}
