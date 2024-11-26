use crate::{BuiltInCommand, Command};
use std::io;
use num_bigint::BigInt;
use num_bigint::Sign::Plus;
use std::collections::HashMap;

pub fn execute(code: HashMap<String, Vec<Command>>) {
    let mut stack: Vec<BigInt> = Vec::new();
    let main_name = "main".to_string();
    if !code.contains_key(&main_name) { // I can has main function?
        return;
    }
    let mut instructions = code[&main_name].clone();
    instructions.reverse();
    let mut call_stack = vec![("main".to_string(), instructions.len())];

    let kwargs: Vec<String> = std::env::args().collect();

    while !instructions.is_empty() {
        //instructions.reverse();
        //println!("{:?}, {:?}", stack, instructions);
        //instructions.reverse();
        //println!("{:?}", call_stack);
        let c_instr = instructions.pop().unwrap();
        call_stack.last_mut().unwrap().1 -= 1;
        match c_instr {
            Command::Integer(val) => stack.push(val),
            Command::BuiltIn(cmd) => match cmd {
                BuiltInCommand::Put => {
                    if let Some(value) = stack.pop() {
                        print!("{}", value);
                    }
                }
                BuiltInCommand::PutLine => {
                    if let Some(value) = stack.pop() {
                        println!("{}", value);
                    } else {
                        println!();
                    }
                }
                BuiltInCommand::Print => {
                    if let Some(value) = stack.pop() {
                        let (_, bytes_be) = value.to_bytes_be();
                        print!("{}", String::from_utf8(bytes_be).unwrap());
                    }
                }
                BuiltInCommand::PrintLine => {
                    if let Some(value) = stack.pop() {
                        let (_, bytes_be) = value.to_bytes_be();
                        println!("{}", String::from_utf8(bytes_be).unwrap());
                    }
                }
                BuiltInCommand::ReadLine => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("error: unable to read user input");
                    if let Some('\n') = input.chars().next_back() {
                        input.pop();
                    }
                    if let Some('\r') = input.chars().next_back() {
                        input.pop();
                    }
                    stack.push(BigInt::from_bytes_be(Plus, input.as_bytes()));
                }
                BuiltInCommand::Exit => {
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
                BuiltInCommand::Arg => {
                    if let Some(value) = stack.pop() {
                        let arg_i: usize = *value.to_u32_digits().1.first().unwrap() as usize;
                        stack.push(BigInt::from_bytes_be(Plus, kwargs[arg_i].as_bytes()))
                    }
                }
                BuiltInCommand::Argc => {
                    stack.push(kwargs.len().into());
                }
                BuiltInCommand::Decrement => { // a -> a-1
                    if let Some(value) = stack.last_mut() {
                        *value -= 1;
                    }
                }
                BuiltInCommand::Increment => { // a -> a+1
                    if let Some(value) = stack.last_mut() {
                        *value += 1;
                    }
                }
                BuiltInCommand::Sum => { // a b -> a+b
                    if let Some(value) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            *last += value;
                        }
                    }
                }
                BuiltInCommand::Difference => { // a b -> a-b
                    if let Some(value) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            *last -= value;
                        }
                    }
                }
                BuiltInCommand::Product => { // a b -> ab
                    if let Some(value) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            *last *= value;
                        }
                    }
                }
                BuiltInCommand::Quotient => { // a b -> a/b
                    if let Some(value) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            *last /= value;
                        }
                    }
                }
                BuiltInCommand::Modulo => { // a b -> a%b
                    if let Some(value) = stack.pop() {
                        if let Some(last) = stack.last_mut() {
                            *last %= value;
                        }
                    }
                }
                BuiltInCommand::Equality => { // a b -> a=b (1 for true)
                    if let Some(value) = stack.pop() {
                        if let Some(value_2) = stack.pop() {
                            stack.push(if value == value_2 {BigInt::from(1)} else {BigInt::from(0)});
                        }
                    }
                }
                BuiltInCommand::Inequality => { // a b -> a!=b (1 for true)
                    if let Some(value) = stack.pop() {
                        if let Some(value_2) = stack.pop() {
                            stack.push(if value != value_2 {BigInt::from(1)} else {BigInt::from(0)});
                        }
                    }
                }
                BuiltInCommand::GreaterThan => {
                    if let Some(value) = stack.pop() {
                        if let Some(value_2) = stack.pop() {
                            stack.push(if value > value_2 {BigInt::from(1)} else {BigInt::from(0)});
                        }
                    }
                }
                BuiltInCommand::LessThan => {
                    if let Some(value) = stack.pop() {
                        if let Some(value_2) = stack.pop() {
                            stack.push(if value < value_2 {BigInt::from(1)} else {BigInt::from(0)});
                        }
                    }
                }
                BuiltInCommand::GreaterEqual => {
                    if let Some(value) = stack.pop() {
                        if let Some(value_2) = stack.pop() {
                            stack.push(if value >= value_2 {BigInt::from(1)} else {BigInt::from(0)});
                        }
                    }
                }
                BuiltInCommand::LessEqual => {
                    if let Some(value) = stack.pop() {
                        if let Some(value_2) = stack.pop() {
                            stack.push(if value <= value_2 {BigInt::from(1)} else {BigInt::from(0)});
                        }
                    }
                }
                BuiltInCommand::Drop => { // a ->
                    stack.pop();
                }
                BuiltInCommand::Duplicate => { // a -> a a
                    if let Some(value) = stack.last() {
                        stack.push(value.clone());
                    } else {
                        stack.push(BigInt::from(0));
                    }
                }
                BuiltInCommand::Swap => { // a b -> b a
                    let len = stack.len();
                    stack.swap(len-1, len-2);
                }
                BuiltInCommand::Over => { // a b -> a b a
                    let val = if stack.len() > 2 {
                        stack[stack.len()-2].clone()
                    } else {
                        BigInt::from(0)
                    };
                    stack.push(val);
                }
                BuiltInCommand::Rotate => { // a b c -> b c a
                    let len = stack.len();
                    stack[len-3..].rotate_left(1);
                }
                BuiltInCommand::NoOp => ()
            },
            Command::UserDef(name) => {
                if let Some(values) = code.get(&name) {
                    if values.len() > 0 {
                        call_stack.push((name.clone(), values.len()));
                        for value in values.iter().rev() {
                            instructions.push(value.clone());
                        }
                    }
                } else {
                    eprintln!("ERROR: Unresolved Symbol: \"{}\"", name);
                    let mut indentation = 0;
                    while call_stack.len() > 0 {
                        indentation += 1;
                        let stack_call = call_stack.remove(0);
                        let instr_ptr = code.get(&stack_call.0).unwrap().len() - stack_call.1;
                        eprintln!("{}in \"${}\", instruction {}", "  ".repeat(indentation), stack_call.0, instr_ptr);
                    }
                    std::process::exit(1);
                }
            },
            Command::Branch(expression) => {
                let val = stack.pop().unwrap();
                call_stack.last_mut().unwrap().1 += 1;
                match val.try_into() {
                    Ok(choice) => {
                        if expression.contains_key(&choice) {
                            instructions.push(expression.get(&choice).unwrap().clone());
                        } else {
                            let max_opt = expression.keys().max().unwrap();
                            if choice > *max_opt {
                                instructions.push(expression.get(&max_opt).unwrap().clone());
                            }
                        }
                    },
                    Err(_) => { // Assuming the number is too great for a usize (but negatives?...)
                        let max_opt = expression.keys().max().unwrap();
                        instructions.push(expression.get(&max_opt).unwrap().clone());
                    }
                }
            }
        }
        while call_stack.len() > 0 && call_stack.last().unwrap().1 == 0 {
            call_stack.pop();
        }
    }
}


