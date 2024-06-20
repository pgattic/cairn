use std::hash::{DefaultHasher, Hash, Hasher};
use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_bigint::Sign::Plus;

#[derive(Clone)]
#[derive(Debug)]
enum Command {
    Integer(BigInt),
    BuiltIn(BuiltInCommand),
    UserDef(u64),
    Branch(HashMap<usize, Command>)
}

#[derive(Clone)]
#[derive(Debug)]
enum BuiltInCommand {
    Put,
    PutLine,
    Print,
    PrintLine,
    ReadLine,
    Exit,

    Increment,
    Decrement,
    Sum,
    Difference,
    Product,
    Quotient,
    Modulo,
    Equality,
    Inequality,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,

    Drop,
    Duplicate,
    Swap,
    Over,
    Rotate,
    NoOp,
}

impl Command {
    pub fn from_str(s: &str) -> Self {
        if let Ok(value) = s.parse::<BigInt>() {
            return Command::Integer(value);
        }

        match s {
            "put"       => Self::BuiltIn(BuiltInCommand::Put),
            "putln"     => Self::BuiltIn(BuiltInCommand::PutLine),
            "print"     => Self::BuiltIn(BuiltInCommand::Print),
            "println"   => Self::BuiltIn(BuiltInCommand::PrintLine),
            "readln"    => Self::BuiltIn(BuiltInCommand::ReadLine),
            "exit"      => Self::BuiltIn(BuiltInCommand::Exit),
            "++"        => Self::BuiltIn(BuiltInCommand::Increment),
            "--"        => Self::BuiltIn(BuiltInCommand::Decrement),
            "+"         => Self::BuiltIn(BuiltInCommand::Sum),
            "-"         => Self::BuiltIn(BuiltInCommand::Difference),
            "*"         => Self::BuiltIn(BuiltInCommand::Product),
            "div"       => Self::BuiltIn(BuiltInCommand::Quotient),
            "%"         => Self::BuiltIn(BuiltInCommand::Modulo),
            "="         => Self::BuiltIn(BuiltInCommand::Equality),
            "!="        => Self::BuiltIn(BuiltInCommand::Inequality),
            "<"         => Self::BuiltIn(BuiltInCommand::LessThan),
            ">"         => Self::BuiltIn(BuiltInCommand::GreaterThan),
            "<="        => Self::BuiltIn(BuiltInCommand::LessEqual),
            ">="        => Self::BuiltIn(BuiltInCommand::GreaterEqual),
            "_"         => Self::BuiltIn(BuiltInCommand::Drop),
            "dup"       => Self::BuiltIn(BuiltInCommand::Duplicate),
            "swp"       => Self::BuiltIn(BuiltInCommand::Swap),
            "over"      => Self::BuiltIn(BuiltInCommand::Over),
            "rot"       => Self::BuiltIn(BuiltInCommand::Rotate),
            "nop"       => Self::BuiltIn(BuiltInCommand::NoOp),
            s if s.starts_with('?') => {
                let choices = s[1..].split(":");
                let mut result = HashMap::new();
                for (i, c) in choices.enumerate() {
                    if !c.is_empty() {
                        result.insert(i, Self::from_str(c));
                    } else {
                        result.insert(i, Self::BuiltIn(BuiltInCommand::NoOp));
                    }
                }
                Self::Branch(result)
            }
            s if s.starts_with('"') => { // a string!
                let value = &s[1..s.len()-1];
                Self::Integer(BigInt::from_bytes_be(Plus, value.as_bytes()))
            }
            _ => Self::UserDef(calculate_hash(s))
        }
    }
}


fn calculate_hash(t: &str) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

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

    let functions = split_code(contents);

    //println!("{:?}", functions);
    execute(functions);
}

fn split_code(input: String) -> HashMap<u64, Vec<Command>> {
    let mut result: HashMap<u64, Vec<Command>> = HashMap::new();
    let mut in_comment = false;
    let mut in_str = false;
    let mut word_start = 0;
    let mut curr_func_hash: Option<u64> = None;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '#' if !in_str => in_comment = true,
            '\n' if in_comment => {
                in_comment = false;
                word_start = i + 1;
            }
            '"' if !in_comment => in_str = !in_str,
            ' ' | '\n' if !in_str && !in_comment => {
                if i - word_start > 0 { // The Word is done!
                    let word = &input[word_start..i];
                    if word.starts_with('$') {
                        let hash = calculate_hash(&word[1..]);
                        curr_func_hash = Some(hash);
                        result.insert(hash, Vec::new());
                    } else {
                        match curr_func_hash {
                            None => (),
                            Some(hash) => {
                                result.entry(hash).or_insert_with(Vec::new).push(Command::from_str(word));
                            }
                        }
                    }
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
        match curr_func_hash {
            None => (),
            Some(hash) => {
                result.entry(hash).or_insert_with(Vec::new).push(Command::from_str(&input[word_start..]));
            }
        }
    }
    result
}

fn execute(code: HashMap<u64, Vec<Command>>) {
    let mut stack: Vec<BigInt> = Vec::new();
    let main_hash: u64 = calculate_hash("main");
    if !code.contains_key(&main_hash) { // I can has main function?
        return;
    }
    let mut instructions = code[&main_hash].clone();
    instructions.reverse();
    let mut last_func = main_hash;

    while !instructions.is_empty() {
        //instructions.reverse();
        //println!("{:?}, {:?}", stack, instructions);
        //instructions.reverse();
        let c_instr = instructions.pop().unwrap();
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
                    last_func = name;
                    for value in values.iter().rev() {
                        instructions.push(value.clone());
                    }
                } else {
                    eprintln!("ERROR: Unresolved Symbol: \"{}\"", name);
                    eprintln!("  In \"${}\"", last_func);
                    std::process::exit(1);
                }
            },
            Command::Branch(expression) => {
                let val = stack.pop().unwrap();
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
    }
}


