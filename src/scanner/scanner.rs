use crate::reader::reader;
use crate::reader::reader::Reader;

#[derive(Debug)]
pub enum TOKEN {
    // BASIC MATH OPERATION
    ADD(char),
    SUB(char),
    MUL(char),
    DIV(char),
    MOD(char),

    // EXP AND FloorDiv
    EXP(Vec<char>),
    FloorDiv(Vec<char>),

    // COMMENTS
    SingleComment(Vec<char>),
    BlockComment(Vec<char>),

}

pub struct Lexer {

}

impl Lexer {
    pub fn new() {}             // Create a new Lexer instance
    pub fn next_token() {}  // Match the read character and assign appropriate type
}


pub fn say_hello() {
    println!("Hello World");
}