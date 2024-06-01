use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use num_bigint::Sign::Plus;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("[{}]: Shell mode is a planned feature. See https://github.com/pgattic/cairn/issues/1 for progress updates.", args[0]);
        eprintln!("Please specify a file.");
        std::process::exit(1);
    }

    let target_file = &args[1];

    let contents: String = match fs::read_to_string(target_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{}: can't open file '{}': {}", args[0], target_file, err);
            std::process::exit(2);
        }
    };

    let code: Vec<&str> = split_code(&contents);

    let mut functions: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut current_func = "";

    for c in &code {
        if c.starts_with('$') {
            current_func = &c[1..];
            functions.insert(current_func, Vec::new());
        } else if !current_func.is_empty() {
            functions.entry(current_func).or_insert_with(Vec::new).push(c);
        }
    }

    //println!("{:?}", functions);
    execute(functions);
}

fn split_code(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut in_comment = false;
    let mut in_str = false;
    let mut word_start = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '#' if !in_str => in_comment = true,
            '\n' if in_comment => {
                in_comment = false;
                word_start = i + 1;
            }
            '"' if !in_comment => in_str = !in_str,
            ' ' | '\n' if !in_str && !in_comment => {
                if i - word_start > 0 {
                    result.push(&input[word_start..i]);
                }
                word_start = i + 1;
            }
            _ => {}
        }
    }
    if in_str {
        eprintln!("ERROR: Unclosed string");
        std::process::exit(1);
    }
    if word_start < input.len() { // Check for no ending whitespace
        result.push(&input[word_start..]);
    }
    result
}

fn execute(code: HashMap<&str, Vec<&str>>) {
    let mut stack: Vec<BigInt> = Vec::new();
    if !code.contains_key("main") { // I can has main function?
        return;
    }
    let mut instructions: Vec<&str> = code["main"].clone();
    instructions.reverse();
    let mut last_func: &str = "main";

    while !instructions.is_empty() {
        // println!("{:?} {:?}", stack, instructions);
        let c_instr = instructions.pop().unwrap();
        match c_instr.parse::<BigInt>() {
            Ok(val) => {
                stack.push(val);
            }
            Err(_) => {
                match c_instr { // Built-in procedures
                    "put" => {
                        if let Some(value) = stack.pop() {
                            print!("{}", value);
                        }
                    }
                    "putln" => {
                        if let Some(value) = stack.pop() {
                            println!("{}", value);
                        } else {
                            println!();
                        }
                    }
                    "print" => {
                        if let Some(value) = stack.pop() {
                            let (_, bytes_be) = value.to_bytes_be();
                            print!("{}", String::from_utf8(bytes_be).unwrap());
                        }
                    }
                    "println" => {
                        if let Some(value) = stack.pop() {
                            let (_, bytes_be) = value.to_bytes_be();
                            println!("{}", String::from_utf8(bytes_be).unwrap());
                        }
                    }
                    "readln" => {
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("error: unable to read user input");
                        if let Some('\n')=input.chars().next_back() {
                            input.pop();
                        }
                        if let Some('\r')=input.chars().next_back() {
                            input.pop();
                        }
                        stack.push(BigInt::from_bytes_be(Plus, input.as_bytes()));
                    }
                    "exit" => {
                        if let Some(value) = stack.pop() {
                            match value.try_into() {
                                Ok(val_u32) => {
                                    std::process::exit(val_u32);
                                },
                                Err(_) => {
                                    std::process::exit(1);
                                }
                            }
                        } else {
                            std::process::exit(0);
                        }
                    }
                    "--" => {
                        if let Some(value) = stack.last_mut() {
                            *value -= 1;
                        }
                    }
                    "++" => {
                        if let Some(value) = stack.last_mut() {
                            *value += 1;
                        }
                    }
                    "+" => {
                        if let Some(value) = stack.pop() {
                            if let Some(last) = stack.last_mut() {
                                *last += value;
                            }
                        }
                    }
                    "-" => {
                        if let Some(value) = stack.pop() {
                            if let Some(last) = stack.last_mut() {
                                *last -= value;
                            }
                        }
                    }
                    "*" => {
                        if let Some(value) = stack.pop() {
                            if let Some(last) = stack.last_mut() {
                                *last *= value;
                            }
                        }
                    }
                    "div" => {
                        if let Some(value) = stack.pop() {
                            if let Some(last) = stack.last_mut() {
                                *last /= value;
                            }
                        }
                    }
                    "%" => {
                        if let Some(value) = stack.pop() {
                            if let Some(last) = stack.last_mut() {
                                *last %= value;
                            }
                        }
                    }
                    "=" => {
                        if let Some(value) = stack.pop() {
                            if let Some(value_2) = stack.pop() {
                                stack.push(if value == value_2 {BigInt::from(1)} else {BigInt::from(0)});
                            }
                        }
                    }
                    "!=" => {
                        if let Some(value) = stack.pop() {
                            if let Some(value_2) = stack.pop() {
                                stack.push(if value != value_2 {BigInt::from(1)} else {BigInt::from(0)});
                            }
                        }
                    }
                    ">" => {
                        if let Some(value) = stack.pop() {
                            if let Some(value_2) = stack.pop() {
                                stack.push(if value > value_2 {BigInt::from(1)} else {BigInt::from(0)});
                            }
                        }
                    }
                    "<" => {
                        if let Some(value) = stack.pop() {
                            if let Some(value_2) = stack.pop() {
                                stack.push(if value < value_2 {BigInt::from(1)} else {BigInt::from(0)});
                            }
                        }
                    }
                    ">=" => {
                        if let Some(value) = stack.pop() {
                            if let Some(value_2) = stack.pop() {
                                stack.push(if value >= value_2 {BigInt::from(1)} else {BigInt::from(0)});
                            }
                        }
                    }
                    "<=" => {
                        if let Some(value) = stack.pop() {
                            if let Some(value_2) = stack.pop() {
                                stack.push(if value <= value_2 {BigInt::from(1)} else {BigInt::from(0)});
                            }
                        }
                    }
                    "_" => {
                        stack.pop();
                    }
                    "dup" => {
                        if let Some(value) = stack.last() {
                            stack.push(value.clone());
                        } else {
                            stack.push(BigInt::from(0));
                        }
                    }
                    "swp" => {
                        if let Some(value) = stack.pop() {
                            if let Some(value_2) = stack.pop() {
                                stack.push(value);
                                stack.push(value_2);
                            }
                        }
                    }
                    "over" => {
                        let val: BigInt = if stack.len() > 2 {
                            stack.remove(stack.len()-2)
                        } else {
                            BigInt::from(0)
                        };
                        stack.push(val);
                    }
                    s if s.starts_with('?') => {
                        if c_instr.contains(':') {
                            let options: Vec<&str> = c_instr[1..].split(":").collect();
                            if let Some(value) = stack.pop() {
                                let index: usize = {
                                    if value < BigInt::from(0) {
                                        0
                                    } else if value >= BigInt::from(options.len()) {
                                        options.len()-1
                                    } else {
                                        value.to_usize().expect("YEET")
                                    }
                                };
                                if !options[index].is_empty() {
                                    instructions.push(options[index]);
                                }
                            }
                        }
                    }
                    s if s.starts_with('"') => { // a string!
                        let value = &s[1..s.len()-1];
                        stack.push(BigInt::from_bytes_be(Plus, value.as_bytes()));
                    }
                    _ => { // Code-defined procedures
                        if let Some(values) = code.get(c_instr) {
                            last_func = c_instr;
                            for value in values.iter().rev() {
                                instructions.push(*value);
                            }
                        } else {
                            eprintln!("ERROR: Unresolved Symbol: \"{}\"", c_instr);
                            eprintln!("  In \"${}\"", last_func);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }
}

