use std::{borrow::Borrow, collections::HashMap};

use crate::{symbol::{self, FunctionStruct, Symbol}, AST::get_first};

pub fn construct_symbol_table() -> HashMap::<String, symbol::Symbol> {
    let mut symbol_table = HashMap::<String, symbol::Symbol>::new();

    // Effectively macros the true and false identifiers to the bool symbol
    symbol_table.insert("true".to_string(), Symbol::Bool(true));
    symbol_table.insert("false".to_string(), Symbol::Bool(false));

    // Manually adds each of our standard library functions
    // TODO: Come up with a much better way to do this
    symbol_table.insert("assert".to_string(), Symbol::Function(FunctionStruct {
        name: "assert".to_string(),
        function: assert
    })); 
    symbol_table.insert("concat".to_string(), Symbol::Function(FunctionStruct {
        name: "concat".to_string(),
        function: concat
    })); 
    symbol_table.insert("add".to_string(), Symbol::Function(FunctionStruct {
        name: "add".to_string(),
        function: add
    })); 
    symbol_table.insert("sub".to_string(), Symbol::Function(FunctionStruct {
        name: "subtract".to_string(),
        function: subtract
    })); 
    symbol_table.insert("mul".to_string(), Symbol::Function(FunctionStruct {
        name: "multiply".to_string(),
        function: multiply
    })); 
    symbol_table.insert("div".to_string(), Symbol::Function(FunctionStruct {
        name: "divide".to_string(),
        function: divide
    })); 
    
    return symbol_table;
}

pub fn assert(args: Vec<Symbol>) -> Symbol {
    if args.clone().len() < 1 {
        return Symbol::Bool(true);
    }

    let binding = args.clone();
    let fst = binding.first().unwrap();
    for arg in args {
        if (fst.clone() != arg) {
            panic!("Assertion error! {} != {}", fst, arg);
        }
    }

    return Symbol::Bool(true);
}

pub fn concat(args: Vec<Symbol>) -> Symbol {
    let mut total = "".to_string();

    for arg in args {
        match arg {
            Symbol::Str(x) => {
                total += &x;
            }
            _ =>  {
                panic!("Add only accepts str types, not {:?}", arg);
            }
        }
    }

    return Symbol::Str(total);
}

pub fn add(args: Vec<Symbol>) -> Symbol {
    let mut total: f64 = 0.;
    for arg in args {
        match arg {
            Symbol::Int(x) => {
                total += f64::from(x);
            }
            Symbol::Double(x) => {
                total += x;
            }
            _ =>  {
                panic!("Add only accepts Int and Double types, not {:?}", arg);
            }
        }
    }
    return Symbol::Double(total);
}

pub fn subtract(args: Vec<Symbol>) -> Symbol {
    if args.len() != 2 {
        panic!("Subtract function must take 2 arguments, but is given {}", args.len());
    }

    let mut total: f64 = 0.;
    let mut flag = false;
    for arg in args {
        match arg {
            Symbol::Int(x) => {
                if flag { total -= f64::from(x); } else { total += f64::from(x); }
            }
            Symbol::Double(x) => {
                if flag { total -= x; } else { total += x; }
            }
            _ =>  {
                panic!("Subtract only accepts Int and Double types, not {:?}", arg);
            }
        }
        flag = true;
    }
    return Symbol::Double(total);
}

pub fn divide(args: Vec<Symbol>) -> Symbol {
    if args.len() != 2 {
        panic!("Divide function must take 2 arguments, but is given {}", args.len());
    }

    let mut total: f64 = 0.;
    let mut flag = false;
    for arg in args {
        match arg {
            Symbol::Int(x) => {
                if flag { total /= f64::from(x); } else { total += f64::from(x); }
            }
            Symbol::Double(x) => {
                if flag { total /= x; } else { total += x; }
            }
            _ =>  {
                panic!("Divide only accepts Int and Double types, not {:?}", arg);
            }
        }
        flag = true;
    }
    return Symbol::Double(total);
}

pub fn multiply(args: Vec<Symbol>) -> Symbol {
    let mut product: f64 = 1.;
    for arg in args {
        match arg {
            Symbol::Int(x) => {
                product *= f64::from(x);
            }
            Symbol::Double(x) => {
                product *= x;
            }
            _ =>  {
                panic!("Mutiply only accepts Int and Double types, not {:?}", arg);
            }
        }
    }
    return Symbol::Double(product);
}