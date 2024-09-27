use std::borrow::Borrow;
use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::File;


fn interpret_line(str: String) {
    print!("{}", str);
}


fn read_lines(filename: &String) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let flag = &args[1];
        let mut file_path = String::new();
        if flag != "-f" {
            println!("You dumb. Get gud and use -f '/path/to/file.lmao_why_are_you_using_this'");
            return;
        }
        file_path = args[2].clone();
        if file_path.len() < 27 || file_path[file_path.len()-27..] != *"lmao_why_are_you_using_this" {
            println!("Wtf do you think you're doing? You HAVE TO use the .lmao_why_are_you_using_this extension!!");
            return;
        }
        if !Path::new::<String>(file_path.borrow()).exists() {
            println!("I can't read this. This stupid thing doesn't even exist.");
            return;
        }
        let mut file = match File::open(&file_path) {
            Err(why) => panic!("Ughhh, I couldn't open {}: {}", file_path, why),
            Ok(file) => file,
        };
        let lines = read_lines(&file_path);

        lines.iter().for_each(|str: &String| interpret_line(str.clone()));
    } else {
        loop {
            print!("lisp> ");
            io::stdout().flush().unwrap();
    
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            
            interpret_line(input);
        }
    }
}

