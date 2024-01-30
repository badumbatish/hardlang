use crate::scanner::token::TOKEN;
use std::{cell::RefCell, rc::Rc};
/// AST module for Parser: We parse via recursive descent.
///
/// For now, an AST consist left node, right node, a TOKEN that represents an operator, and a TOKEN
/// that represents an value after being evaluated

#[derive(Debug)]
pub struct AstNode {
    pub op: TOKEN,
    pub left: Option<AstNodeRef>,
    pub right: Option<AstNodeRef>,
    pub value: TOKEN,
}


pub type AstNodeRef = Rc<RefCell<AstNode>>;

pub struct AstTree {
    root_node : AstNodeRef
}

impl AstTree {

}

#[cfg(test)]
mod ast_node_test {
    #[test]
    fn test_that() {
        assert!(true);
    }
}

#[cfg(test)]
mod ast_tree_test {
    #[test]
    fn test_this() {
        assert!(true)
    }
}