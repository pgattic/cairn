use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use num_bigint::Sign::Plus;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_file: String = if args.len() > 1 {args.get(1).expect("owie").to_string()} else {"test.crn".to_string()};

    let contents: String = fs::read_to_string(target_file).expect("Failed to read file");

    let code_lines = contents.split("\n");
    let modified_lines  = code_lines // comments
        .map(|line| line.split("#").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join(" ");
    let code: Vec<String> = split_code(&modified_lines);

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

    _ = execute(functions);
}

fn split_code(input: &String) -> Vec<String> {
    let input: Vec<char> = input.chars().collect();
    let mut result: Vec<String> = Vec::new();
    let mut in_str = false;
    let mut curr_str: String = "".to_string();
    for i in input {
        if i == '"' {
            in_str = !in_str;
        } else if !in_str {
            if i == ' ' || i == '\n' {
                if curr_str.len() > 0 {
                    result.push(curr_str);
                    curr_str = "".to_string();
                }
                continue;
            }
        }
        curr_str.push(i);
    }
    result
}

fn execute(code: HashMap<&str, Vec<&str>>) -> Result<(), String> {
    let mut stack: Vec<BigInt> = Vec::new();
    let mut instructions: Vec<&str> = code["main"].iter().cloned().collect();

    while !instructions.is_empty() {
        let c_instr = instructions[0];
        instructions.remove(0);
    //    println!("{}", c_instr);
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
                            print!("{}\n", value);
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
                        if let Some(&ref last_value) = stack.last() {
                            stack.push(last_value.clone());
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
                    s if s.starts_with('?') => {
                        if c_instr.contains(':') {
                            let options: Vec<&str> = c_instr[1..].split(":").collect();
                            if let Some(value) = stack.last() {
                                let index: usize = {
                                    if *value < BigInt::from(0) {
                                        0
                                    } else if *value >= BigInt::from(options.len()) {
                                        options.len()-1
                                    } else {
                                        value.to_usize().expect("YEET")
                                    }
                                };
                                if !options[index].is_empty() {
                                    instructions.insert(0, options[index]);
                                }
                            }
                        }
                    }
                    s if s.starts_with('"') => { // a string!
                        let value = &s[1..s.len()-1];
                        stack.push(BigInt::from_bytes_be(Plus, value.as_bytes()));
                    }
                    &_ => { // Code-defined procedures
                        if let Some(values) = code.get(c_instr) {
                            for value in values.iter().rev() {
                                instructions.insert(0, *value);
                            }
                        } else {
                            println!("Unresolved Symbol: {}", c_instr);
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

