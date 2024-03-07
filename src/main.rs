use crate::parser::ast::{AstNode, AstNodeRef};
use crate::scanner::token::TOKEN;

mod parser;
mod reader;
mod scanner;

fn main() {
    let _a = TOKEN::NUM("0.0".to_string());

    let b : TOKEN = TOKEN::ADD;
    let c : Option<AstNodeRef> = None;
    let d : Option<AstNodeRef> = None;
    let e : TOKEN = TOKEN::NUM("0.0".to_string());

    let _ast : AstNode = AstNode { op: b, left: c, right: d, value: e };
}
