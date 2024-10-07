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

// TODO: Make this not shit
pub fn subtract(args: Vec<Symbol>) -> Symbol {
    if args.len() != 2 {
        panic!("Subtract function must take 2 arguments, but is given {}", args.len());
    }

    let mut total: f64 = 0.;
    let mut flag = false;
    for arg in args {
        match arg {
            Symbol::Int(x) => {
                if flag {
                    total -= f64::from(x);
                    flag = true;
                } else {
                    total += f64::from(x);
                    flag = true
                }
            }
            Symbol::Double(x) => {
                if flag {
                    total -= x;
                    flag = true;
                } else {
                    total += x;
                    flag = true
                }
            }
            _ =>  {
                panic!("Subtract only accepts Int and Double types, not {:?}", arg);
            }
        }
    }
    return Symbol::Double(total);
}

// TODO: Make this not shit
pub fn divide(args: Vec<Symbol>) -> Symbol {
    if args.len() != 2 {
        panic!("Divide function must take 2 arguments, but is given {}", args.len());
    }

    let mut total: f64 = 0.;
    let mut flag = false;
    for arg in args {
        match arg {
            Symbol::Int(x) => {
                if flag {
                    total /= f64::from(x);
                    flag = true;
                } else {
                    total += f64::from(x);
                    flag = true
                }
            }
            Symbol::Double(x) => {
                if flag {
                    total /= x;
                    flag = true;
                } else {
                    total += x;
                    flag = true
                }
            }
            _ =>  {
                panic!("Divide only accepts Int and Double types, not {:?}", arg);
            }
        }
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