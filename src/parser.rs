use std::collections::btree_map::Range;
use std::thread::scope;
use std::{borrow::Borrow, collections::HashMap};
use regex::Regex;
use std::rc::Rc;
use std::cell::RefCell;

use crate::standard_lib::{add, subtract};
use crate::symbol::{self, FunctionStruct, Symbol};
use crate::AST::{self, wrap_symbol, AST_Node};

pub fn parse(line: String) -> Rc<RefCell<AST_Node>>  {
    let mut symbol_table = HashMap::<String, symbol::Symbol>::new();
    symbol_table.insert("add".to_string(), Symbol::Function(FunctionStruct {
        name: "add".to_string(),
        function: add
    })); 
    symbol_table.insert("sub".to_string(), Symbol::Function(FunctionStruct {
        name: "subtract".to_string(),
        function: subtract
    })); 

    let str_tokens = tokenize_line(&line, &mut symbol_table);

    let validated_tokens = validate_parenthesis(str_tokens.clone());

    return ast_parse(validated_tokens, &mut symbol_table);
}

// Converts a token stream and symbol_table into an AST
// Responsibility: Construction of AST
fn ast_parse(token_stream: Vec<String>, symbol_table: &mut HashMap<String, symbol::Symbol>) -> Rc<RefCell<AST_Node>> {
    let mut scope = 0;
    let mut nodes = Vec::<Rc<RefCell<AST_Node>>>::new();
    for token in &token_stream {
        if token == "(" {
            scope += 1;
            nodes.insert(0, wrap_symbol(Symbol::Sexp));

            if scope > 1 {
                // Only access the first node directly without cloning
                nodes[1].borrow_mut().subnodes.push(nodes[0].clone());
            }
        } else if scope == 1 && token == ")" {
            return nodes[0].clone();
        } else if token == ")" {
            scope -= 1;
            nodes.remove(0);
        } else {
            println!("token: {}", token);
            let sym = symbol_table[token].clone();
            nodes[0].borrow_mut().subnodes.push(wrap_symbol(sym.clone())); 
        }
    }
    panic!("BROKEN!!! Gonna fix this, but did you have the correct number of parenthesis?"); // TODO: Actually write an error message lol
}

// TODO: When we start defining a library and functions perform type checking and argument validation
fn validate_parenthesis(token_stream: Vec<String>) -> Vec<String> {
    let mut x: Vec<String>  = token_stream.clone();
    if token_stream.first().is_some() && token_stream.first().unwrap() != "(" {
        x.insert(0, "(".to_string());
    }
    if token_stream.last().is_some() && token_stream.last().unwrap() != ")" {
        x.push(")".to_string());
    }
    
    let mut scope = 0;
    for token in token_stream {
        if token == "(" {
            scope += 1;
        } else if token == ")" {
            scope -= 1;
        }

        if scope < 0 {
            panic!("Oopsie whoopsie, looks like someone can't even fucking type. Unexpected closing parenthesis.");
        }
    }
    if scope > 0 {
        panic!("Oopsie whoopsie, looks like someone can't even fucking type. Too many opening parenthesis or missing closing parenthesis.");
    }

    x
}

// Performs simple lexing (converts string to token stream)
// Parses strings, int, and double literals and converts them to symbols
// Also populates the symbol_table
// Responsibility: Lexing, Parsing of literals
fn tokenize_line(line: &String, symbol_table: &mut HashMap<String, symbol::Symbol>) -> Vec<String> {
    let mut line_strings_replaced = parse_strings(line, symbol_table);

    // Performs basic lexing
    let mut result = Vec::new();
    let re = Regex::new(r"([()\[\]{},;])").unwrap(); // Match all the symbols
    line_strings_replaced = re.replace_all(&line_strings_replaced, " $1 ").to_string();
    result = line_strings_replaced.split_whitespace().map(|x| x.to_string()).collect();

    // Replaces int with identifiers and adds these to the symbol table
    result = handle_comments(result); // Removes all tokens between two ;'s (second is optional to comment entire rest of line)
    result = parse_double(result, symbol_table);
    result = parse_int(result, symbol_table);

    validate_identifiers(result.clone(), symbol_table);
    
    return result.iter().map(|x| (**x).to_string()).collect();
}

// Replaces strings with identifiers to make our lives easier
// Adds these to the symbol table
// TODO: Handle escape characters
fn parse_strings(line: &String, symbol_table: &mut HashMap<String, symbol::Symbol>) -> String {
    let mut is_string = false;
    let mut cur_line = "".to_string();
    let mut cur_str = "".to_string();
    let mut i = 0;
    for c in line.chars() {
        if c == '\"' && !is_string {
            is_string = true;
        } else if c == '\"' {
            let name = String::from(format!("str{}", i));
            is_string = false;
            i += 1;
            let sym = symbol::Symbol::Str(cur_str.clone());
            symbol_table.insert(name.clone(), sym); // Insert the symbol
            cur_line = format!("{}{}", cur_line, name);
        } else if is_string {
            cur_str = format!("{}{}", cur_str, c);
        } else {
            cur_line = format!("{}{}", cur_line, c);
        }
    }
    if is_string {
        panic!("Uneven number of quotations you fucker, thought I'd just miss that one did you?")
    }
    cur_line
}

// TODO: Implement checks to ensure this will actually fit into a 32 bit integer
// Also, consider making arbitrarily sized integers
fn parse_int(tokens: Vec<String>, symbol_table: &mut HashMap<String, symbol::Symbol>) -> Vec<String> {
    let mut i = 0;
    let ints: Vec<String> = tokens
        .iter()
        .map(|x| {
            // Check if the token can be parsed as an integer
            if x.parse::<i64>().is_ok() {
                let name = format!("int{}", i);
                let sym = symbol::Symbol::Int(x.parse::<i32>().unwrap());
                symbol_table.insert(name.clone(), sym); // Insert the symbol
                i += 1;
                name // Return the name directly
            } else {
                x.to_string()
            }
        })
        .collect();

    ints
}


// TODO: Implement checks to ensure this will actually fit into a 64 bit float
// Also, consider making arbitrarily sized floats
fn parse_double(tokens: Vec<String>, symbol_table: &mut HashMap<String, symbol::Symbol>) -> Vec<String> {
    let mut i = 0;
    let doubles: Vec<String> = tokens
        .iter()
        .map(|x| {
            // Check if the token is a valid double by manually checking for a decimal point
            if x.parse::<f64>().is_ok() && x.contains('.') {
                let name = format!("double{}", i);
                let sym = symbol::Symbol::Double(x.parse::<f64>().unwrap());
                symbol_table.insert(name.clone(), sym); // Insert the symbol
                i += 1;
                name // Return the name directly
            } else {
                x.to_string()
            }
        })
        .collect();

    doubles
}


fn handle_comments(tokens: Vec<String>) -> Vec<String> {
    let mut is_commment = false;
    let toks: Vec<String> = tokens
        .iter()
        .map(|x| {
            if x == ";" {
                is_commment = !is_commment;
                return None;
            }

            if !is_commment {
                Some(x.to_string())
            } else {
                None
            }
        }).filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    toks
}

fn validate_identifiers(tokens: Vec<String>, symbol_table: &mut HashMap<String, symbol::Symbol>) {
    let re_identifier = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let identifiers: Vec<_> = tokens
            .iter()
            .map(|x| {
                if re_identifier.is_match(&x) && (symbol_table.contains_key(x)) {
                    return Some(x);
                } else if (re_identifier.is_match(&x)) {
                    panic!("I'm not a mind reader you idiot! \"{}\" is undefined.", x);
                } else if "()".contains(x) {
                    return None;
                } else {
                    panic!("What do you expect me to do with this??? \"{}\" is not a valid symbol!", x);
                }
            })
            .collect();
}