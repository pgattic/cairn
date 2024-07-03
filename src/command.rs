use std::collections::HashMap;
use num_bigint::BigInt;
use num_bigint::Sign::Plus;

#[derive(Clone)]
#[derive(Debug)]
pub enum Command {
    Integer(BigInt),
    BuiltIn(BuiltInCommand),
    UserDef(String),
    Branch(HashMap<usize, Command>)
}

#[derive(Clone)]
#[derive(Debug)]
pub enum BuiltInCommand {
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
    fn from_str(s: &str) -> Self {
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
            _ => Self::UserDef(s.to_string())
        }
    }

    pub fn split_code(input: String) -> HashMap<String, Vec<Command>> {
        let mut result: HashMap<String, Vec<Command>> = HashMap::new();
        let mut in_comment = false;
        let mut in_str = false;
        let mut word_start = 0;
        let mut curr_func: Option<&str> = None;
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
                            let name = &word[1..];
                            curr_func = Some(name);
                            result.insert(name.to_string(), Vec::new());
                        } else {
                            match curr_func {
                                None => (),
                                Some(name) => {
                                    result.entry(name.to_string()).or_insert_with(Vec::new).push(Command::from_str(word));
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
            match curr_func {
                None => (),
                Some(name) => {
                    result.entry(name.to_string()).or_insert_with(Vec::new).push(Command::from_str(&input[word_start..]));
                }
            }
        }
        result
    }
}


