/*
* TO COMPILE, WE MUST FIRST...
* TOKENIZE IT BABY
*/
use std::iter::Peekable;
// For our tokens, we dont' need to go into too much detail as to what we are storing...
// We can define further in our Node Type exactly what we are parsing. For now
// just being able to separate out string literals, number literals, open and closed
// parens and brackets should be enough.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(String),
    Operand(char),
    Carrot,
    Period,
    Colon,
    Semicolon,
    Underscore,
    Hashtag,
    Asterisk,
    Modulo,
    Backslash,
    Bang,
    Comma,
    Pipe,
    LeftCarrot,
    RightCarrot,
    ClosedParen,
    OpenParen,
    OpenBracket,
    Equals,
    Plus,
    Minus,
    ClosedBracket,
    Quote,
    Ampersand,
}
impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Minus => return String::from("Minus/Dash"),
            Token::Plus => return String::from("Plus"),
            Token::Equals => return String::from("Equals"),
            Token::String(s) => return s.clone(),
            Token::Number(s) => return s.clone(),
            Token::Operand(s) => return s.to_string(),
            Token::Quote => return String::from("Quote"),
            Token::Asterisk => return String::from("Star/Asterisk"),
            Token::Modulo => return String::from("Modulo"),
            Token::Bang => return String::from("Bang"),
            Token::Backslash => return String::from("Backslash"),
            Token::Semicolon => return String::from("Semi-Colon"),
            Token::Colon => return String::from("Colon"),
            Token::Period => return String::from("Period"),
            Token::Comma => return String::from("Comma"),
            Token::Pipe => return String::from("Pipe"),
            Token::Underscore => return String::from("Underscore"),
            Token::Carrot => return String::from("Carrot"),
            Token::Hashtag => return String::from("Hashtag"),
            Token::Ampersand => return String::from("Ampersand"),
            Token::LeftCarrot => return String::from("Left Carrot"),
            Token::RightCarrot => return String::from("Right Carrot"),
            Token::ClosedParen => return String::from("Closed Parenthesis"),
            Token::OpenParen => return String::from("Open Parenthesis"),
            Token::OpenBracket => return String::from("Open Bracket"),
            Token::ClosedBracket => return String::from("Closed Bracket"),
        }
    }
}
/* So for instance, our language may have

 func addOne(x int) int {
 return x + 1
  }

* so in this case our tokenizer would need to pick up "func" and store it in a Token::String enum
* with the value String::from("func") and then the next token would also be Token::String type with
* the value String::from("addOne") followed by a naked Token::OpenParen follwed by a String.. and
* you get the idea. So let's begin our tokenizer function...
*/

// This will take in our input string and return a result type, meaning either Ok(Vec<Token>) or
// Err(String) in this case..
pub fn tokenizer(lines: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    // What we need to do here is iterate one character at a time, and if it is a simliar type of
    // char (number literal, string literal, etc) we need to store it first in a string, then
    // create out token enum and push it into our vector that we will return.
    let mut line_number = 0;
    let binding = lines.clone();
    let mut words: Peekable<std::str::Chars> = binding.chars().peekable();
    while let Some(&next) = words.peek() {
        if next == '"' {
            tokens.push(Token::Quote)
        }
        match next {
            '\n' => line_number += 1,
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::ClosedParen),
            '{' => tokens.push(Token::OpenBracket),
            '}' => tokens.push(Token::ClosedBracket),
            '*' => tokens.push(Token::Asterisk),
            '!' => tokens.push(Token::Bang),
            '&' => tokens.push(Token::Ampersand),
            '%' => tokens.push(Token::Modulo),
            '^' => tokens.push(Token::Carrot),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '_' => tokens.push(Token::Underscore),
            '#' => tokens.push(Token::Hashtag),
            '=' => tokens.push(Token::Equals),
            ':' => tokens.push(Token::Colon),
            ';' => tokens.push(Token::Semicolon),
            '<' => tokens.push(Token::LeftCarrot),
            '>' => tokens.push(Token::RightCarrot),
            '|' => tokens.push(Token::Pipe),
            '/' => tokens.push(Token::Backslash),
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&next) = words.peek() {
                    if next.is_ascii_digit() {
                        number.push(words.next().unwrap());
                    }
                    tokens.push(Token::Number(number.clone()));
                }
            }
            'a'..='z' | 'A'..='Z' => {
                while let Some(&next) = words.peek() {
                    let mut word: String = String::new();
                    if next.is_ascii_alphabetic() {
                        word.push(words.next().unwrap().to_ascii_lowercase());
                    } else {
                        break;
                    }
                }
            }
            _ => continue,
        }
    }
    println!("success!");
    return Ok(tokens);
}