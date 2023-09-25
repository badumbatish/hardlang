use crate::scanner::token::TOKEN;
use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
pub struct AstNode {
    op: TOKEN,
    left: Option<AstNodeRef>,
    right: Option<AstNodeRef>,
    value: TOKEN,
}

type AstNodeRef = Rc<RefCell<AstNode>>;
