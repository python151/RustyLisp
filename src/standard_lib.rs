use std::{arch::x86_64, default};

use crate::symbol::{self, Symbol};

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