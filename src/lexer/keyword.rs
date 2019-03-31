use std::str::FromStr;
use std::convert::TryFrom;


pub const KEYWORD_ASYNC: &[char]      = &['a', 's', 'y', 'n', 'c'];
pub const KEYWORD_AWAIT: &[char]      = &['a', 'w', 'a', 'i', 't'];
pub const KEYWORD_BREAK: &[char]      = &['b', 'r', 'e', 'a', 'k'];
pub const KEYWORD_CASE: &[char]       = &['c', 'a', 's', 'e'];
pub const KEYWORD_CATCH: &[char]      = &['c', 'a', 't', 'c', 'h'];
pub const KEYWORD_CLASS: &[char]      = &['c', 'l', 'a', 's', 's'];
pub const KEYWORD_CONST: &[char]      = &['c', 'o', 'n', 's', 't'];
pub const KEYWORD_CONTINUE: &[char]   = &['c', 'o', 'n', 't', 'i', 'n', 'u', 'e'];
pub const KEYWORD_DEBUGGER: &[char]   = &['d', 'e', 'b', 'u', 'g', 'g', 'e', 'r'];
pub const KEYWORD_DEFAULT: &[char]    = &['d', 'e', 'f', 'a', 'u', 'l', 't'];
pub const KEYWORD_DELETE: &[char]     = &['d', 'e', 'l', 'e', 't', 'e'];
pub const KEYWORD_DO: &[char]         = &['d', 'o'];
pub const KEYWORD_ELSE: &[char]       = &['e', 'l', 's', 'e'];
pub const KEYWORD_EXPORT: &[char]     = &['e', 'x', 'p', 'o', 'r', 't'];
pub const KEYWORD_EXTENDS: &[char]    = &['e', 'x', 't', 'e', 'n', 'd', 's'];
pub const KEYWORD_FINALLY: &[char]    = &['f', 'i', 'n', 'a', 'l', 'l', 'y'];
pub const KEYWORD_FOR: &[char]        = &['f', 'o', 'r'];
pub const KEYWORD_FUNCTION: &[char]   = &['f', 'u', 'n', 'c', 't', 'i', 'o', 'n'];
pub const KEYWORD_IF: &[char]         = &['i', 'f'];
pub const KEYWORD_IMPORT: &[char]     = &['i', 'm', 'p', 'o', 'r', 't'];
pub const KEYWORD_IN: &[char]         = &['i', 'n'];
pub const KEYWORD_INSTANCEOF: &[char] = &['i', 'n', 's', 't', 'a', 'n', 'c', 'e', 'o', 'f'];
pub const KEYWORD_NEW: &[char]        = &['n', 'e', 'w'];
pub const KEYWORD_RETURN: &[char]     = &['r', 'e', 't', 'u', 'r', 'n'];
pub const KEYWORD_SUPER: &[char]      = &['s', 'u', 'p', 'e', 'r'];
pub const KEYWORD_SWITCH: &[char]     = &['s', 'w', 'i', 't', 'c', 'h'];
pub const KEYWORD_THIS: &[char]       = &['t', 'h', 'i', 's'];
pub const KEYWORD_THROW: &[char]      = &['t', 'h', 'r', 'o', 'w'];
pub const KEYWORD_TRY: &[char]        = &['t', 'r', 'y'];
pub const KEYWORD_TYPEOF: &[char]     = &['t', 'y', 'p', 'e', 'o', 'f'];
pub const KEYWORD_VAR: &[char]        = &['v', 'a', 'r'];
pub const KEYWORD_VOID: &[char]       = &['v', 'o', 'i', 'd'];
pub const KEYWORD_WHILE: &[char]      = &['w', 'h', 'i', 'l', 'e'];
pub const KEYWORD_WITH: &[char]       = &['w', 'i', 't', 'h'];
pub const KEYWORD_YIELD: &[char]      = &['y', 'i', 'e', 'l', 'd'];
pub const KEYWORD_LET: &[char]        = &['l', 'e', 't'];
pub const KEYWORD_STATIC: &[char]     = &['s', 't', 'a', 't', 'i', 'c'];
pub const KEYWORD_ENUM: &[char]       = &['e', 'n', 'u', 'm'];
pub const KEYWORD_IMPLEMENTS: &[char] = &['i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 's'];
pub const KEYWORD_PROTECTED: &[char]  = &['p', 'r', 'o', 't', 'e', 'c', 't', 'e', 'd'];
pub const KEYWORD_INTERFACE: &[char]  = &['i', 'n', 't', 'e', 'r', 'f', 'a', 'c', 'e'];
pub const KEYWORD_PRIVATE: &[char]    = &['p', 'r', 'i', 'v', 'a', 't', 'e'];
pub const KEYWORD_PUBLIC: &[char]     = &['p', 'u', 'b', 'l', 'i', 'c'];


// https://www.ecma-international.org/ecma-262/9.0/index.html#sec-keywords
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Async,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,

    Let,
    Static,

    // FutureReservedWord
    // https://www.ecma-international.org/ecma-262/9.0/index.html#sec-future-reserved-words
    Enum,
    // Use of the following tokens within strict mode code is also reserved
    Implements,
    Package,
    Protected,
    Interface,
    Private,
    Public,
}

impl Keyword {
    pub fn is_future_reserved(&self) -> bool {
        // https://www.ecma-international.org/ecma-262/9.0/index.html#prod-FutureReservedWord
        use self::Keyword::*;

        match *self {
            Enum => true,
            Implements
            | Package
            | Protected
            | Interface
            | Private
            | Public => true,
            _ => false,
        }
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Keyword::*;

        match s {
            "async" => Ok(Async),
            "await" => Ok(Await),
            "break" => Ok(Break),
            "case" => Ok(Case),
            "catch" => Ok(Catch),
            "class" => Ok(Class),
            "const" => Ok(Const),
            "continue" => Ok(Continue),
            "debugger" => Ok(Debugger),
            "default" => Ok(Default),
            "delete" => Ok(Delete),
            "do" => Ok(Do),
            "else" => Ok(Else),
            "export" => Ok(Export),
            "extends" => Ok(Extends),
            "finally" => Ok(Finally),
            "for" => Ok(For),
            "function" => Ok(Function),
            "if" => Ok(If),
            "import" => Ok(Import),
            "in" => Ok(In),
            "instanceof" => Ok(Instanceof),
            "new" => Ok(New),
            "return" => Ok(Return),
            "super" => Ok(Super),
            "switch" => Ok(Switch),
            "this" => Ok(This),
            "throw" => Ok(Throw),
            "try" => Ok(Try),
            "typeof" => Ok(Typeof),
            "var" => Ok(Var),
            "void" => Ok(Void),
            "while" => Ok(While),
            "with" => Ok(With),
            "yield" => Ok(Yield),
            "let" => Ok(Let),
            "static" => Ok(Static),
            "enum" => Ok(Enum),
            "implements" => Ok(Implements),
            "package" => Ok(Package),
            "protected" => Ok(Protected),
            "interface" => Ok(Interface),
            "private" => Ok(Private),
            "public" => Ok(Public),
            _ => Err(())
        }
    }
}

impl TryFrom<&[char]> for Keyword {
    type Error = ();
    
    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        use self::Keyword::*;

        match value {
            KEYWORD_ASYNC => Ok(Async),
            KEYWORD_AWAIT => Ok(Await),
            KEYWORD_BREAK => Ok(Break),
            KEYWORD_CASE => Ok(Case),
            KEYWORD_CATCH => Ok(Catch),
            KEYWORD_CLASS => Ok(Class),
            KEYWORD_CONST => Ok(Const),
            KEYWORD_CONTINUE => Ok(Continue),
            KEYWORD_DEBUGGER => Ok(Debugger),
            KEYWORD_DEFAULT => Ok(Default),
            KEYWORD_DELETE => Ok(Delete),
            KEYWORD_DO => Ok(Do),
            KEYWORD_ELSE => Ok(Else),
            KEYWORD_EXPORT => Ok(Export),
            KEYWORD_EXTENDS => Ok(Extends),
            KEYWORD_FINALLY => Ok(Finally),
            KEYWORD_FOR => Ok(For),
            KEYWORD_FUNCTION => Ok(Function),
            KEYWORD_IF => Ok(If),
            KEYWORD_IMPORT => Ok(Import),
            KEYWORD_IN => Ok(In),
            KEYWORD_INSTANCEOF => Ok(Instanceof),
            KEYWORD_NEW => Ok(New),
            KEYWORD_RETURN => Ok(Return),
            KEYWORD_SUPER => Ok(Super),
            KEYWORD_SWITCH => Ok(Switch),
            KEYWORD_THIS => Ok(This),
            KEYWORD_THROW => Ok(Throw),
            KEYWORD_TRY => Ok(Try),
            KEYWORD_TYPEOF => Ok(Typeof),
            KEYWORD_VAR => Ok(Var),
            KEYWORD_VOID => Ok(Void),
            KEYWORD_WHILE => Ok(While),
            KEYWORD_WITH => Ok(With),
            KEYWORD_YIELD => Ok(Yield),
            KEYWORD_LET => Ok(Let),
            KEYWORD_STATIC => Ok(Static),
            KEYWORD_ENUM => Ok(Enum),
            KEYWORD_IMPLEMENTS => Ok(Implements),
            KEYWORD_PROTECTED => Ok(Package),
            KEYWORD_INTERFACE => Ok(Protected),
            KEYWORD_PRIVATE => Ok(Interface),
            KEYWORD_PUBLIC => Ok(Private),
            _ => Err(()),
        }
    }
}

