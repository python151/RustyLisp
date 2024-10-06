use crate::symbol::{self, Symbol};
use crate::AST;
use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct AST_Node {
    pub sym: Symbol,
    pub subnodes: Vec<Rc<RefCell<AST_Node>>>
}

pub fn wrap_symbol(sym: Symbol) -> Rc<RefCell<AST_Node>> {
    Rc::new(RefCell::new(AST_Node { sym: sym, subnodes: (Vec::<Rc<RefCell<AST_Node>>>::new()) }))
}

pub fn get_first(node: Rc<RefCell<AST_Node>>) -> Symbol {
    let head_borrow: &RefCell<AST_Node> = node.borrow();
    let head_borrow_1 = head_borrow.borrow();
    
    let first: &RefCell<AST_Node> = head_borrow_1.subnodes.first().unwrap().borrow();
    return first.borrow().sym.clone();
}

pub fn print_ast(prefix: String, head: Rc<RefCell<AST_Node>>) {
    // Borrow the head node to access its subnodes
    let head_borrow: &RefCell<AST_Node> = head.borrow();

    println!("{}{:?}", prefix, head_borrow.borrow().sym.clone());
    
    // Iterate over the subnodes
    for node in head_borrow.borrow().subnodes.clone() {
        print_ast(format!("\t{}", prefix), node);
    }
}
