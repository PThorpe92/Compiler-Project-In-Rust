use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TS {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, PartialEq, Copy, Eq, Clone)]
pub struct Span {
    pub beg: u32,
    pub end: u32,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum TokenType {
    StringLiteral,
    NumberLiteral,
    Operand,
    Carrot,
    Period,
    Colon,
    Semicolon,
    Underscore,
    Hashtag,
    Asterisk,
    Modulo,
    BackSlash,
    ForwardSlash,
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
    QuoteOpen,
    QuoteClosed,
    Ampersand,
    EOF,
}
impl Token {
    pub fn to_string(&self) -> String {
        match self.kind {
            TokenType::StringLiteral => {
                return String::from(format!("StringLiteral: {}", self.value.clone()))
            }
            TokenType::NumberLiteral => {
                return String::from(format!("NumberLiteral: {}", self.value.clone()))
            }
            _ => return self.kind.to_string(),
        }
    }
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::Minus => return String::from("Minus/Dash"),
            TokenType::Plus => return String::from("Plus"),
            TokenType::Equals => return String::from("Equals"),
            TokenType::StringLiteral => return String::from("String Literal"),
            TokenType::NumberLiteral => return String::from("Number Literal"),
            TokenType::Operand => return String::from("Operand/Symbol"),
            TokenType::QuoteOpen => return String::from("OpenQuote"),
            TokenType::QuoteClosed => return String::from("ClosedQuote"),
            TokenType::Asterisk => return String::from("Star/Asterisk"),
            TokenType::Modulo => return String::from("Modulo"),
            TokenType::Bang => return String::from("Bang"),
            TokenType::BackSlash => return String::from("Backslash"),
            TokenType::ForwardSlash => return String::from("Forwardslash"),
            TokenType::Semicolon => return String::from("Semi-Colon"),
            TokenType::Colon => return String::from("Colon"),
            TokenType::Period => return String::from("Period"),
            TokenType::Comma => return String::from("Comma"),
            TokenType::Pipe => return String::from("Pipe"),
            TokenType::Underscore => return String::from("Underscore"),
            TokenType::Carrot => return String::from("Carrot"),
            TokenType::Hashtag => return String::from("Hashtag"),
            TokenType::Ampersand => return String::from("Ampersand"),
            TokenType::LeftCarrot => return String::from("Left Carrot/Less than"),
            TokenType::RightCarrot => return String::from("Right Carrot/Greater than"),
            TokenType::ClosedParen => return String::from("Closed Parenthesis"),
            TokenType::OpenParen => return String::from("Open Parenthesis"),
            TokenType::OpenBracket => return String::from("Open Bracket"),
            TokenType::ClosedBracket => return String::from("Closed Bracket"),
            TokenType::EOF => return String::from("EOF"),
        }
    }
}
// Spanner is a trait that returns a Span object,
// which is the beginning and end point of our current token
// we are parsing in the source code
pub trait Spanner {
    fn span(&self) -> Span;
}
impl Spanner for Span {
    fn span(&self) -> Span {
        return *self;
    }
}
impl<T: Spanner> Spanner for Vec<T> {
    fn span(&self) -> Span {
        if self.is_empty() {
            return Span { beg: 0, end: 0 };
        } else {
            return Span {
                beg: self.first().unwrap().span().beg,
                end: self.last().unwrap().span().end,
            };
        }
    }
}

// This will take in our input string and return a result type, meaning either Ok(Vec<Token>) or
// Err(String) in this case..
pub fn tokenizer(lines: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    /*What we need to do here is iterate one character at a time, and if it is a simliar type of
    char (number literal, string literal, etc) we need to store it first in a string, then
    create out token enum and push it into our vector that we will return.
    Rust has the type Peekable<T> which allows us to look ahead in the iterable
    without indexing and risking an index out of range error */
    let mut line_number = 0;
    let mut openquote: bool = false;
    let mut error_lines: HashMap<String, i32> = HashMap::new();
    let binding = lines.clone();
    let mut words: Peekable<std::str::Chars> = binding.chars().peekable();
    while let Some(ch) = words.next() {
        if ch == '"' {
            if openquote {
                tokens.push(Token {
                    kind: TokenType::QuoteClosed,
                    value: "none".to_string(),
                });
                openquote = false;
            } else {
                tokens.push(Token {
                    kind: TokenType::QuoteOpen,
                    value: "none".to_string(),
                });
                openquote = true;
            }
        } else if ch.is_whitespace() {
            // Skip whitespace characters
            continue;
        } else if ch == '\t' {
            continue;
        } else if ch == '\n' {
            line_number += 1;
            continue;
        }
        match ch {
            '(' => tokens.push(Token {
                kind: TokenType::OpenParen,
                value: "none".to_string(),
            }),
            ')' => tokens.push(Token {
                kind: TokenType::ClosedParen,
                value: "none".to_string(),
            }),
            '{' => tokens.push(Token {
                kind: TokenType::OpenBracket,
                value: "none".to_string(),
            }),
            '}' => tokens.push(Token {
                kind: TokenType::ClosedBracket,
                value: "none".to_string(),
            }),
            '|' => tokens.push(Token {
                kind: TokenType::Pipe,
                value: "none".to_string(),
            }),
            '\\' => tokens.push(Token {
                kind: TokenType::BackSlash,
                value: "none".to_string(),
            }),
            '*' => tokens.push(Token {
                kind: TokenType::Asterisk,
                value: "none".to_string(),
            }),
            '!' => tokens.push(Token {
                kind: TokenType::Bang,
                value: "none".to_string(),
            }),
            '&' => tokens.push(Token {
                kind: TokenType::Ampersand,
                value: "none".to_string(),
            }),
            '%' => tokens.push(Token {
                kind: TokenType::Modulo,
                value: "none".to_string(),
            }),
            '^' => tokens.push(Token {
                kind: TokenType::Carrot,
                value: "none".to_string(),
            }),
            '+' => tokens.push(Token {
                kind: TokenType::Plus,
                value: "none".to_string(),
            }),
            ',' => tokens.push(Token {
                kind: TokenType::Comma,
                value: "none".to_string(),
            }),
            '.' => tokens.push(Token {
                kind: TokenType::Period,
                value: "none".to_string(),
            }),
            '_' => tokens.push(Token {
                kind: TokenType::Underscore,
                value: "none".to_string(),
            }),
            '-' => tokens.push(Token {
                kind: TokenType::Minus,
                value: "none".to_string(),
            }),
            '#' => tokens.push(Token {
                kind: TokenType::Hashtag,
                value: "none".to_string(),
            }),
            '=' => tokens.push(Token {
                kind: TokenType::Equals,
                value: "none".to_string(),
            }),
            ':' => tokens.push(Token {
                kind: TokenType::Colon,
                value: "none".to_string(),
            }),
            ';' => tokens.push(Token {
                kind: TokenType::Semicolon,
                value: "none".to_string(),
            }),
            '<' => tokens.push(Token {
                kind: TokenType::LeftCarrot,
                value: "none".to_string(),
            }),
            '>' => tokens.push(Token {
                kind: TokenType::RightCarrot,
                value: "none".to_string(),
            }),
            '/' => {
                if *words.peek().unwrap() == '/' {
                    //inline comment
                    while let Some(&next) = words.peek() {
                        if next == '\n' {
                            words.next();
                            break;
                        } else {
                            words.next();
                        }
                    }
                } else if *words.peek().unwrap() == '*' {
                    // block comment, ignore all words/chars until we see these again
                    while let Some(&next) = words.peek() {
                        if next == '*' && *words.peek().unwrap() == '/' {
                            words.next();
                            break;
                        }
                    }
                } else {
                    tokens.push(Token {
                        kind: TokenType::ForwardSlash,
                        value: "none".to_string(),
                    });
                }
            }
            '?' | '@' | '$' | '~' => tokens.push(Token {
                kind: TokenType::Operand,
                value: String::from(ch),
            }),
            '0'..='9' => {
                let mut number = String::new();
                number.push(ch);
                while let Some(&ch) = words.peek() {
                    if ch.is_ascii_digit() {
                        number.push(ch);
                        words.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    kind: TokenType::NumberLiteral,
                    value: number.clone(),
                });
            }
            'a'..='z' | 'A'..='Z' => {
                let mut word: String = String::new();
                word.push(ch);
                while let Some(&ch) = words.peek() {
                    if ch.is_ascii_alphabetic() {
                        word.push(ch);
                        words.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    kind: TokenType::StringLiteral,
                    value: word.clone(),
                });
            }
            _ => {
                error_lines.insert(String::from(ch), line_number);
            }
        }
        /*
            func main() {
                println("hello world")
                let i = 0
                for i < 100 {
                println(i)
                }
        }
        This is our output of the above file:

        "StringLiteral: func"
        "StringLiteral: main"
        "Open Parenthesis"
        "Closed Parenthesis"
        "Open Bracket"
        "StringLiteral: println"
        "Open Parenthesis"
        "Quote"
        "StringLiteral: ello"
        "StringLiteral: world"
        "Quote"
        "Closed Parenthesis"
        "StringLiteral: let"
        "StringLiteral: i"
        "Equals"
        "NumberLiteral: 0"
        "StringLiteral: for"
        "StringLiteral: i"
        "Left Carrot/Less than"
        "NumberLiteral: 100"
        "Open Bracket"
        "StringLiteral: println"
        "Open Parenthesis"
        "StringLiteral: i"
        "Closed Parenthesis"
        "Closed Bracket"
        "Closed Bracket"
        success!
        "StringLiteral: main"
        "Period"
        "StringLiteral: p"
        Input file: main.p
        Output filename: filename
            */
    }
    println!("success!");
    for item in tokens.iter() {
        println!("{:?}", item.to_string());
    }
    for (key, value) in error_lines.iter() {
        println!("Unexpected character: {} on line {}", key, value);
    }
    return Ok(tokens);
}
