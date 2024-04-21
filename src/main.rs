use std::{env, fs, process::exit};

fn run_file(path: &str) -> Result<(), String>{
   match fs::read_to_string(path) {
    Err(msg) => return Err(msg.to_string()),
    _ => return Ok(()),
   }
    
}


fn run_prompt(){}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage : jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR : \n{}", msg)
            }
        }
    } else {
        run_prompt();
    }

    dbg!(args);
}
