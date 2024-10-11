use crate::evaluator::eval;
use crate::AST::{get_first, AST_Node};
use crate::symbol::Symbol;
use core::panic;
use std::rc::Rc;
use std::cell::RefCell;

pub fn if_statement(nodes: Vec<Rc<RefCell<AST_Node>>>) -> Symbol {
    if nodes.clone().len() != 3 {
        panic!("If statements must have exactly 3 arguments, not {}!", nodes.clone().len());
    }

    let condition: bool;
    if let Symbol::Bool(val) = nodes.clone().first().unwrap().borrow().sym {
        condition = val;
    } else if let Symbol::Sexp = nodes.clone().first().unwrap().borrow().sym {
        if let Symbol::Bool(val) = eval(nodes.first().unwrap().clone()).borrow().sym.clone() {
            condition = val;
        } else {
            panic!("First argument to if statement must be of type Bool");
        }
    } else {
        panic!("First argument to if statement must be of type Bool");
    }

    if condition {
        return eval(nodes.get(1).unwrap().clone()).borrow().sym.clone();
    } else {
        return eval(nodes.last().unwrap().clone()).borrow().sym.clone();
    }
}