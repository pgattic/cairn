pub mod command;
pub mod program;
use crate::command::{BuiltInCommand, Command};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Shell mode is a planned feature.");
        println!("See https://github.com/pgattic/cairn/issues/1 for progress updates.");
        println!();
        eprintln!("[{}]: Please specify a file.", args[0]);
        std::process::exit(0x01);
    }

    let target_file = &args[1];

    let contents: String = match fs::read_to_string(target_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{}: can't open file '{}': {}", args[0], target_file, err);
            std::process::exit(0x02);
        }
    };

    let functions = Command::split_code(contents);

    //println!("{:?}", functions);

    program::execute(functions);
}

fn calculate_hash(t: &str) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

