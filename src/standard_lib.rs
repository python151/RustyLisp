use std::{collections::HashMap, io};
use std::io::{Write};
use std::io::{Read};

use crate::symbol::{self, FunctionStruct, Symbol};

pub fn construct_symbol_table() -> HashMap::<String, symbol::Symbol> {
    let mut symbol_table = HashMap::<String, symbol::Symbol>::new();

    // Effectively macros the true and false identifiers to the bool symbol
    symbol_table.insert("true".to_string(), Symbol::Bool(true));
    symbol_table.insert("false".to_string(), Symbol::Bool(false));

    // Manually adds each of our standard library functions
    // TODO: Come up with a better way to do this
    symbol_table.insert("assert".to_string(), Symbol::Function(FunctionStruct {
        name: "assert".to_string(),
        function: assert
    })); 
    symbol_table.insert("print".to_string(), Symbol::Function(FunctionStruct {
        name: "print".to_string(),
        function: print
    })); 
    symbol_table.insert("input".to_string(), Symbol::Function(FunctionStruct {
        name: "input".to_string(),
        function: input
    })); 
    symbol_table.insert("str_to_double".to_string(), Symbol::Function(FunctionStruct {
        name: "str_to_double".to_string(),
        function: string_to_double
    }));
    symbol_table.insert("concat".to_string(), Symbol::Function(FunctionStruct {
        name: "concat".to_string(),
        function: concat
    })); 
    symbol_table.insert("equals".to_string(), Symbol::Function(FunctionStruct {
        name: "equals".to_string(),
        function: equals
    })); 
    symbol_table.insert("nequals".to_string(), Symbol::Function(FunctionStruct {
        name: "not equals".to_string(),
        function: not_equals
    })); 
    symbol_table.insert("not".to_string(), Symbol::Function(FunctionStruct {
        name: "not".to_string(),
        function: logical_not
    })); 
    symbol_table.insert("if".to_string(), Symbol::Function(FunctionStruct {
        name: "if".to_string(),
        function: if_statement
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

fn assert(args: Vec<Symbol>) -> Symbol {
    if args.clone().len() < 1 {
        return Symbol::Bool(true);
    }

    let binding = args.clone();
    let fst = binding.first().unwrap();
    for arg in args {
        if fst.clone() != arg {
            panic!("Assertion error! {} != {}", fst, arg);
        }
    }

    return Symbol::Bool(true);
}

fn print(args: Vec<Symbol>) -> Symbol {
    for arg in args {
        print!("{} ", arg);
    }
    println!("");

    return Symbol::Bool(true)
}

fn string_to_double(args: Vec<Symbol>) -> Symbol {
    if args.clone().len() != 1 {
        panic!("str_to_double must be given exactly one argument");
    }
    if let Symbol::Str(x) = args.first().unwrap() {
        if x.parse::<f64>().is_ok() {
            return symbol::Symbol::Double(x.parse::<f64>().unwrap());
        } else {
            panic!("str_to_double can't parse value {}", x)
        }
    }
    panic!("str_to_double must be given an argument of type String, not {}", args.first().unwrap());
}

fn input(args: Vec<Symbol>) -> Symbol {
    if args.clone().len() > 1 {
        panic!("input must be given either 0 or 1 arguments");
    }

    if args.clone().len() == 1 {
        print!("{}", args.first().unwrap());
        io::stdout().flush().unwrap();
    }

    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    return Symbol::Str(buf.trim().to_string());
}

fn equals(args: Vec<Symbol>) -> Symbol {
    if args.clone().len() < 1 {
        return Symbol::Bool(true);
    } 

    let binding = args.clone();
    let fst = binding.first().unwrap();
    for arg in args {
        if fst.clone() != arg {
            return Symbol::Bool(false);
        }
    }

    return Symbol::Bool(true);
}

fn logical_not(args: Vec<Symbol>) -> Symbol {
    if args.clone().len() != 1 {
        panic!("Not only accepts 1 argument, not {}!", args.clone().len());
    }
    
    let sym = args.first().unwrap();

    if let Symbol::Bool(x) = sym {
        return Symbol::Bool(!x);
    }

    panic!("Not only accepts Bool types, and {} is not a bool", sym);
}

fn not_equals(args: Vec<Symbol>) -> Symbol {
    logical_not(vec![equals(args)])
}

fn if_statement(args: Vec<Symbol>) -> Symbol {
    if args.clone().len() != 3 {
        panic!("If statements must have exactly 3 arguments, not {}!", args.clone().len());
    }

    if let Symbol::Bool(condition) = args.first().unwrap() {
        if *condition {
            args.get(1).unwrap().clone()
        } else {
            args.last().unwrap().clone()
        }
    } else {
        panic!("First argument to if must be of type Bool, not {}!", args.first().unwrap());
    }
}

fn concat(args: Vec<Symbol>) -> Symbol {
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

fn add(args: Vec<Symbol>) -> Symbol {
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

fn subtract(args: Vec<Symbol>) -> Symbol {
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

fn divide(args: Vec<Symbol>) -> Symbol {
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

fn multiply(args: Vec<Symbol>) -> Symbol {
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