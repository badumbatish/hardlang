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

    NUM(f64), // Number : always floating point 64 bit
    IDENTIFIER(String), // Identifier: a, b, c, d, e, f
}

#[derive(Debug)]
pub struct TokenPair {
    token: TOKEN,
    str : String
}

impl TokenPair {
    // Constructor
    fn new(token: TOKEN, s: &str) -> Self {
        TokenPair {
            token,
            str: String::from(s),
        }
    }

}