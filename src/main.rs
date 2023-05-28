mod lexer;
use crate::lexer::tokenizer;
use crate::lexer::Token;
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
            match read_input_file(file.clone()) {
                Ok(tokens) => {
                    for token in tokens.iter() {
                        println!("debugging, we got to here! \n");
                        println!("{}", token.to_string().clone());
                    }
                }
                Err(err) => println!("Error reading file: {}", err),
            }
            println!("Input file: {}", file);
            println!("Output filename: {}", output);
        }
        _ => println!("Invalid input. Usage: compiler -i input_file -o output_filename"),
    }
    println!("success?");
}

fn read_input_file(file: String) -> Result<Vec<Token>, String> {
    match read_to_string(file) {
        Ok(lines) => match tokenizer(lines) {
            Ok(tokens) => return Ok(tokens),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}
