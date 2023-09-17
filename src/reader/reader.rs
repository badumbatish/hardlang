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
    input: Vec<char>,           // Source code
    pub position: usize,        // Reading position
    pub read_position: usize,   // Current moving reading position
    pub ch: char                // Current read character
}

impl Lexer {
    fn new() {}             // Create a new Lexer instance
    pub fn read_char() {}   // Read next char, update positions
    pub fn next_token() {}  // Match the read character and assign appropriate type
}


pub fn say_hello() {
    println!("Hello World");
}