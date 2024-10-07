use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::File;
use std::io::ErrorKind;


use parser::parse;
use AST::print_ast;
use evaluator::eval;

mod symbol;
mod AST;
mod parser;
mod evaluator;
mod standard_lib;

fn interpret_line(str: String) {
    let ast = parse(str);
    print_ast(">".to_string(), ast.clone());
    let evaluated = eval(ast);
    print_ast(">".to_string(), evaluated);
}


fn read_lines(filename: &String) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn has_read_permission(file_path: &str) -> bool {
    match File::open(file_path) {
        Ok(_) => true,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => false,
            _ => false, // handle other errors like file not found
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If flag is given check that it has a second argument
        let flag = &args[1];
        if flag != "-f" || args.len() != 3 {
            println!("You dumb. Get gud and use -f '/path/to/file.lmao_why_are_you_using_this'");
            return;
        }

        // Parse the file_path and do some simple formatting checks
        let file_path = &args[2];
        if file_path.len() < 27 || file_path[file_path.len()-27..] != *"lmao_why_are_you_using_this" {
            println!("Wtf do you think you're doing? You HAVE TO use the .lmao_why_are_you_using_this extension!! 😉");
            return;
        }

        // Checks that file exists and that we can open it
        if !Path::new(file_path).exists() {
            println!("I can't read this. This stupid thing doesn't even exist (Are you pointing to the right file?)");
            return;
        }
        if !has_read_permission(&file_path) {
            println!("Bro forgor to give me read permission. I guess I'll cry now... (maybe run \"chmod u+r /path/to/file.lmao_why_are_you_using_this\" on unix based stuff, if you're a windows user... i dunno, cry?)");
            return;
        }
        let _ = match File::open(&file_path) {
            Err(why) => panic!("Ughhh, I couldn't open {}: {}", file_path, why),
            Ok(file) => file,
        };

        // Actually reads file and starts interpretting
        let lines = read_lines(&file_path);
        lines.iter().for_each(|str: &String| interpret_line(str.clone()));
    } else {
        // If no flags are given create interactive shell
        println!("Welcome to my LISP interpreter, or as I like to call it LAUGH (Lisp but an An UnGodly Hellscape)");
        println!("Don't worry, I've constructed this language to have extremely... insightful? error messages.");
        println!("I mean, you gain insight into something I guess... probably");
        loop {
            print!("laugh> ");
            io::stdout().flush().unwrap();
    
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            
            interpret_line(input);
        }
    }
}

