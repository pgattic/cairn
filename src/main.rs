use std::env;
use std::fs;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_file: String = if args.len() > 1 {args.get(1).expect("owie").to_string()} else {"test.crn".to_string()};

    let contents: String = fs::read_to_string(target_file).expect("Failed to read file");

    let code_lines = contents.split("\n");
    let modified_lines  = code_lines // comments
        .map(|line| line.split("#").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join(" ");
    let code: String = modified_lines.replace(&['(', ')'][..], "");
    let code: Vec<&str> = code.split_whitespace().collect();

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

    println!("{:?}", functions);
    execute(&functions);

}

fn execute(code: &HashMap<&str, Vec<&str>>) {
    let mut stack: Vec<BigInt> = Vec::new();
    let mut instructions: Vec<&str> = code["main"].iter().cloned().collect();

    while !instructions.is_empty() {
        println!("{:?} {:?}", stack, instructions);
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
                    "putl" => {
                        if let Some(value) = stack.pop() {
                            print!("{}\n", value);
                        }
                    }
                    "putc" => {
                        if let Some(value) = stack.pop() {
                            if let Some(ch) = char::from_u32(value.try_into().unwrap()) {
                                print!("{}", ch);
                            }
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
                    &_ => { // Code-defined procedures
                        if let Some(values) = code.get(c_instr) {
                            for value in values.iter().rev() {
                                instructions.insert(0, *value);
                            }
                        } else {
                            println!("Unresolved Symbol: {}", c_instr);
                        }
                    }
                }
            }
        }
    }
}

