#[derive(Debug)]
pub enum TOKEN {
    // BASIC MATH OPERATION
    ADD(char),
    SUB(char),
    MUL(char),
    DIV(char),
    MOD(char),

    // EXP AND FloorDiv
    EXP(str),
    FloorDiv(str),

    // COMMENTS
    SingleComment(str),
    BlockComment(str),

}
pub fn say_hello() {
    println!("Hello World");
}