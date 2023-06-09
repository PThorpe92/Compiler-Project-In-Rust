mod lexer;
mod parser;
use crate::lexer::tokenizer;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    File(String),
    Option(Flag),
}

impl Input {
    pub fn to_string(&self) -> String {
        match self {
            Input::File(s) => s.clone(),
            Input::Option(_) => String::from("Option variant"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Flag {
    Output,
    Input,
    Help,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_file: Option<String> = None;
    let mut output_filename: Option<String> = None;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-i" if i < args.len() - 1 => {
                input_file = Some(args[i + 1].clone());
            }
            "-o" if i < args.len() - 1 => {
                output_filename = Some(args[i + 1].clone());
            }
            _ => (),
        }
    }

    match (input_file, output_filename) {
        (Some(file), Some(output)) => {
            match read_to_string(file.clone()) {
                Ok(lines) => match tokenizer(lines) {
                    Ok(_tokens) => (),
                    Err(err) => println!("{}", err),
                },
                Err(err) => println!("{}", err),
            }
            match tokenizer(file.clone()) {
                Ok(_tokens) => {
                    for item in _tokens.iter() {
                        std::fs::write("./output.txt", item.to_string()).unwrap();
                    }
                }
                Err(err) => println!("{}", err),
            }
            println!("Input file: {}", file);
            println!("Output filename: {}", output);
        }
        _ => println!("Invalid input. Usage: compiler -i input_file -o output_filename"),
    }
    println!("success?");
}
