use std::fmt;


/*
Prop:
    filename: String,
    line_number: usize,
    column_number: usize,

Prototype:
    name: String
    message: String
    toString: Function
*/

#[derive(Debug, Clone)]
pub struct Error {
    line_number: usize,
    column_number: usize,
    filename: String,
    message: String,
    stack: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

// https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Errors
// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-native-error-types-used-in-this-standard
#[derive(Debug, Clone)]
pub enum NativeError {
    Error(String),
    // ParserError and CompilerError
    EarlySyntaxError {
        line_number: usize,
        column_number: usize,
        message: String,
    },
    SyntaxError(String),
    /// This exception is not currently used within this specification.
    /// This object remains for compatibility with previous editions of this specification.
    EvalError {
        line_number: usize,
        column_number: usize,
        native_error: Box<NativeError>,
    },
    RangeError(String),
    ReferenceError(String),
    TypeError(String),
    URIError(String),

    // non-standard
    InternalError(String),
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::NativeError::*;

        match self {
            Error(message) => write!(f, "Error: {}", message),
            EarlySyntaxError{ message, .. } => write!(f, "SyntaxError: {}", message),
            SyntaxError(message) => write!(f, "SyntaxError: {}", message),
            EvalError { line_number, column_number, native_error } => write!(f, "{}", native_error),
            RangeError(message) => write!(f, "RangeError: {}", message),
            ReferenceError(message) => write!(f, "ReferenceError: {}", message),
            TypeError(message) => write!(f, "TypeError: {}", message),
            URIError(message) => write!(f, "URIError: {}", message),
            InternalError(message) => write!(f, "InternalError: {}", message),
        }
    }
}

impl NativeError {

    #[inline]
    pub fn error<T: Into<String>>(message: T) -> Self {
        NativeError::Error(message.into())
    }

    #[inline]
    pub fn syntax_error<T: Into<String>>(message: T) -> Self {
        NativeError::SyntaxError(message.into())
    }

    #[inline]
    pub fn eval_error(line_number: usize,
                      column_number: usize,
                      error: NativeError) -> Self {
        NativeError::EvalError {
            line_number,
            column_number,
            native_error: Box::new(error),
        }
    }

    #[inline]
    pub fn range_error<T: Into<String>>(message: T) -> Self {
        NativeError::RangeError(message.into())
    }

    #[inline]
    pub fn reference_error<T: Into<String>>(message: T) -> Self {
        NativeError::ReferenceError(message.into())
    }
    
    #[inline]
    pub fn type_error<T: Into<String>>(message: T) -> Self {
        NativeError::TypeError(message.into())
    }

    #[inline]
    pub fn uri_error<T: Into<String>>(message: T) -> Self {
        NativeError::URIError(message.into())
    }

    #[inline]
    pub fn internal_error<T: Into<String>>(message: T) -> Self {
        NativeError::InternalError(message.into())
    }
}
