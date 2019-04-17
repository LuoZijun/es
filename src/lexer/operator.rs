

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

