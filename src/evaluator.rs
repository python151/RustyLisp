use std::rc::Rc;
use std::cell::RefCell;

use crate::AST::{get_first, wrap_symbol, AST_Node};
use crate::symbol::Symbol;

pub fn eval(node: Rc<RefCell<AST_Node>>) -> Rc<RefCell<AST_Node>> {
    let sym: Symbol = get_first(node.clone());
    if let Symbol::Function(fun) = sym {
        // Remove the function node
        let args: Vec<_> = node.borrow().subnodes[1..].to_vec();

        // If the child node is a value symbol, extract symbol, else evaulate the sexp to get new symbol
        let symbol_args: Vec<Symbol> = args.iter().map(|arg|
            if matches!(arg.clone().borrow().sym, Symbol::Sexp) {
                eval(arg.clone()).borrow().sym.clone()
            } else {
                arg.borrow().sym.clone()
            }
        ).collect();
        
        // Pass symbols into function pointer
        return wrap_symbol((fun.function)(symbol_args));
    } else if let Symbol::Statement(statement) = sym {
        // Remove the statement node
        let args: Vec<_> = node.borrow().subnodes[1..].to_vec();

        // Call statement
        return wrap_symbol((statement.statement)(args));
    } else {
        panic!("Attempted to call expression with no function type, cannot evaluate!");
    }
}