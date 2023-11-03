use crate::scanner::token::TOKEN;
use std::{cell::RefCell, rc::Rc};
/// AST module for Parser: We parse via recursive descent.
///
/// For now, an AST consist left node, right node, a TOKEN that represents an operator, and a TOKEN
/// that represents an value after being evaluated

#[derive(Debug)]
pub struct AstNode {
    op: TOKEN,
    left: Option<AstNodeRef>,
    right: Option<AstNodeRef>,
    value: TOKEN,
}

type AstNodeRef = Rc<RefCell<AstNode>>;

pub struct AstTree {
    root_node : AstNodeRef
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