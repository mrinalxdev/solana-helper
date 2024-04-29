mod scanner;
use crate::scanner::*;

use std::io::BufRead;
use std::{env, fs, io, process::exit};

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(contents: &str) -> Result<(), String> {
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!(">");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            }
            Err(_) => return Err("Couldnt read line".to_string()),
        }
        println!("You wrote : {}", buffer);
        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage : jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR : \n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR : \n{}", msg);
                exit(1);
            }
        }
    }
}
