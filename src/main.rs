mod token;
mod scanner;

use std::fs;
use std::env;

use token::*;
use scanner::*;

fn error(line: i32, message: String) {
    report(line, String::from(""), message);
}
fn report(line: i32, where_: String, message: String) {
    println!("[line {}] Error {}: {}", line, where_, message);
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token.to_string());
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    run(contents);
}

fn run_prompt() {
    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        run(input);
    }
}

fn main() {
    let mut had_error = false;

    let args: Vec<String> = env::args().skip(1).collect(); 
    if args.len() > 1 {
        println!("Usage : cargo run <filename>");
        return;
    } else if args.len() == 1 {
        run_file(args[0].clone());
    } else {
        run_prompt();
    }
}
