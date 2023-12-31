#[derive(Debug, PartialEq)]
pub enum TOKEN {
    // TODO: Needs to granularize this pile of token into different pieces
    // TODO: Needs to start assigning types to TOKEN so we can record the value
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
    DivAssign, // /=

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

    IDENTIFIER, // Identifier: a, b, c, d, e, f
}
