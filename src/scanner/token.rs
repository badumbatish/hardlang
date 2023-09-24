#[derive(Debug, PartialEq)]
pub enum TOKEN {
    // BASIC MATH OPERATION
    ADD, // +
    SUB, // -
    MUL, // *
    DIV, // /
    MOD, // %

    AddAssign, // +=
    AddIncr,   // ++
    SubAssign, // -=
    SubDecr,   // --
    MulAssign, // *=


    AND, // &&
    OR,  // ||
    XOR, // ^
    SHL, // <<
    SHR, // >>

    EQUAL,   // ==
    LESS,    // <
    GREATER, // >
    ASSIGN,  // =
    NOT,     // !

    AndAssign,    // &=
    OrAssign,     // !=
    XorAssign,    // ^=
    ShlAssign,    // <<=
    ShrAssign,    // >>=
    AndNotAssign, // &^=

    // EXP AND FloorDiv
    EXP,      // **
    FloorDiv, // /_
    CeilDiv,  // /^

    // COMMENTS
    SingleComment, //
    // BlockComment(Vec<char>),
    EOF,     //
    INVALID, //
}
