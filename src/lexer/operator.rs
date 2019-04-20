

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperatorKind {
    Prefix,
    Infix,
    Postfix,
}


// UNARY_OPERATORS
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrefixOperator {
    Await,     // await
    Delete,    // delete
    Void,      // void
    TypeOf,    // typeof
    /// The unary positive operator +.
    Positive,  // +
    /// The unary negation operator -.
    Negative,  // -
    BitNot,    // ~
    Not,       // !

    Increment, // ++
    Decrement, // --
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InfixOperator {
    // BINARY_OPERATORS
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Rem,      // %
    Pow,      // **
    BitShl,   // <<
    BitShr,   // >>
    BitUShr,  // >>>
    And,      // &&
    Or,       // ||
    BitAnd,   // &
    BitXor,   // ^
    BitOr,    // |

    // COMPARE_OPERATORS
    Gt,        // >
    Lt,        // <
    GtEq,      // >=
    LtEq,      // <=
    Eq,        // ==
    Neq,       // !=
    StrictEq,  // ===
    StrictNeq, // !==

    InstanceOf,  // instanceof
    In,          // in
}

impl InfixOperator {
    pub fn precedence(&self) -> u8 {
        use self::InfixOperator::*;

        match *self {
            Pow => 15,
            Mul | Div | Rem => 14,
            Add | Sub => 13,
            BitShl | BitShr | BitUShr => 12,
            Lt | LtEq | Gt | GtEq | In | InstanceOf => 11,
            Eq | Neq | StrictEq | StrictNeq => 10,
            BitAnd => 9,
            BitXor => 8,
            BitOr => 7,
            And => 6,
            Or => 5,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PostfixOperator {
    Increment, // ++
    Decrement, // --
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AssignmentOperator {
    Assign,         //    =
    AddAssign,      //   +=
    SubAssign,      //   -=
    MulAssign,      //   *=
    DivAssign,      //   /=
    RemAssign,      //   %=
    PowAssign,      //  **=

    BitAndAssign,   //   &=
    BitOrAssign,    //   |=
    BitXorAssign,   //   ^=
    BitShlAssign,   //  <<=
    BitShrAssign,   //  >>=
    BitUShrAssign,  // >>>=
}

