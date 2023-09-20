use crate::reader::reader::{Reader, ReaderGeneral};

// In case of multiple readers implementation, please use the next line to set the reader
// type Reader = Reader
#[derive(Debug)]
pub enum TOKEN {
    // BASIC MATH OPERATION
    ADD, // +
    SUB,  // -
    MUL, // *
    DIV, // /
    MOD, // %

    AND, // &&
    OR, // ||
    XOR, // ^
    SHL, // <<
    SHR, // >>

    EQUAL,    // ==
    LESS,   // <
    GREATER,   // >
    ASSIGN, // =
    NOT,   // !


    AndAssign, // &=
    OrAssign, // !=
    XorAssign, // ^=
    ShlAssign, // <<=
    ShrAssign, // >>=
    AndNotAssign, // &^=

    // EXP AND FloorDiv
    EXP, // **
    FloorDiv, // /_
    CeilDiv, // /^

    // COMMENTS
    SingleComment, //
    // BlockComment(Vec<char>),


}

pub struct Lexer {
    reader : Reader,
}

impl Lexer {
    // Create a new Lexer instance
    pub fn new(file_path : &str) -> Self {
        Self {
            reader : Reader::new(file_path)
        }
    }
    pub fn next_token() {

    }  // Match the read character and assign appropriate type

}


