use std::fmt;


// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-native-error-types-used-in-this-standard
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ErrorKind {
    LexicalError,
    ParseError,
    SyntaxError,

    EvalError,
    RangeError,
    ReferenceError,
    TypeError,
    URIError,

    // non-standard
    InternalError,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorKind::*;

        match *self {
            LexicalError | ParseError | SyntaxError => write!(f, "SyntaxError"),
            EvalError => write!(f, "EvalError"),
            RangeError => write!(f, "RangeError"),
            ReferenceError => write!(f, "ReferenceError"),
            TypeError => write!(f, "TypeError"),
            URIError=> write!(f, "URIError"),
            InternalError => write!(f, "InternalError"),
        }
    }
}


#[derive(PartialEq, Eq, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    filename: Option<String>,
    line_number: Option<usize>,
    column_number: Option<usize>,
    line: Option<String>,
}

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self { 
            kind,
            message: message.into(),
            filename: None,
            line_number: None,
            column_number: None,
            line: None,
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn filename(&self) -> &str {
        match self.filename {
            None => panic!("Ensure Stack Infomation is added."),
            Some(ref filename) => &filename
        }
    }

    pub fn line_number(&self) -> usize {
        match self.line_number {
            None => panic!("Ensure Stack Infomation is added."),
            Some(line_number) => line_number
        }
    }

    pub fn column_number(&self) -> usize {
        match self.column_number {
            None => panic!("Ensure Stack Infomation is added."),
            Some(column_number) => column_number
        }
    }

    // NOTE: Set Stack infomation
    pub fn set_stack<F: Into<String>>(&mut self, 
                                      filename: F,
                                      line_number: usize,
                                      column_number: usize,
                                      line: Option<String>) {
        self.filename = Some(filename.into());
        self.line_number = Some(line_number);
        self.column_number = Some(column_number);

        if line.is_some() {
            self.line = line;
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "{}: {:?}", self.kind, self.message);
        let _ = writeln!(f, "    {}:{}:{}", self.filename(), self.line_number(), self.column_number());

        match self.line {
            Some(ref line) => { write!(f, "{}", line); },
            None => { },
        }

        Ok(())
    }
}
