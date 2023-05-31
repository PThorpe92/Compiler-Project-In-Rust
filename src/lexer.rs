use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
/*
* My initial plan was to do this iteratively, and I am definitely still
* going to make use of Rust's Iterator<Peekable> methods. But I think I am
* going to end up making this a little more traditional and make a Lexer object,
* that has functions that parse one token at a time, and return the Lexer object
* with the parsed token, position of the token in the source code, etc. This will
* be called recursively in our main function, instead of one large Tokenizer func
* that will output the Result<Vector, String> with the entire source file parsed...
* To be honest, I am really not sure what the advantages are to this. I feel like
* most compiler implementations that I've checked out, use this method of recursively
* calling a method that parses one token at a time, on a struct/object that has details
* about the current Token, Span. Perhaps it's simpler to display error messages this way?
* I'm sure as the complexity grows, so too will you want the modularity of something
* like this. So for now, we implement methods on our Lexer object to handle this,
* outputting Tokens separately.
*/
#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub reader: Peekable<Chars<'static>>,
    pub curr_token: Token,
    pub position: u32,
}
impl Lexer {
    pub fn peek_token(&self) -> char {
        if self.input.len() <= self.position.try_into().unwrap() {
            '\0'
        } else {
            self.position += 1;
            return *self.reader.peek().unwrap();
        }
    }
    pub fn new(&self, file: String) -> Lexer {
        let readr: Peekable<Chars> = file.chars().peekable().clone();
        let token: Token = self.parse_token();
        return Lexer {
            input: file,
            reader: readr,
            curr_token: token,
            position: 0,
        };
    }
    pub fn parse_token(&self) -> Option<Token> {
        let token: Token = Token::new();
        let ch = self.peek_token();
            if ch == '"' {
                if openquote {
                    return Some(Token {
                        kind: TokenType::Quote,
                        value: "none".to_string(),
                    });
            } else if ch == '\'' {
                return Some(Token {
                    kind: TokenType::SingleQuote,
                    value: "none".to_string(),
                })
            } else if ch.is_whitespace() {
                // Skip whitespace characters
                return None;
            } else if ch == '\t' {
                return None;
            } else if ch == '\n' {
                self.position += 1;
                return None;
            }
            match ch {
                '(' => {
                    token = (Token {
                        kind: TokenType::OpenParen,
                        value: "none".to_string(),
                    })
                }
                ')' => {
                    token = (Token {
                        kind: TokenType::ClosedParen,
                        value: "none".to_string(),
                    })
                }
                '{' => {
                    token = (Token {
                        kind: TokenType::OpenBracket,
                        value: "none".to_string(),
                    })
                }
                '}' => {
                    token = (Token {
                        kind: TokenType::ClosedBracket,
                        value: "none".to_string(),
                    })
                }
                '|' => {
                    token = (Token {
                        kind: TokenType::Pipe,
                        value: "none".to_string(),
                    })
                }
                '\\' => {
                    token = (Token {
                        kind: TokenType::BackSlash,
                        value: "none".to_string(),
                    })
                }
                '*' => {
                    token = (Token {
                        kind: TokenType::Asterisk,
                        value: "none".to_string(),
                    })
                }
                '!' => {
                    token = (Token {
                        kind: TokenType::Bang,
                        value: "none".to_string(),
                    })
                }
                '&' => {
                    token = (Token {
                        kind: TokenType::Ampersand,
                        value: "none".to_string(),
                    })
                }
                '%' => {
                    token = (Token {
                        kind: TokenType::Modulo,
                        value: "none".to_string(),
                    })
                }
                '^' => {
                    token = (Token {
                        kind: TokenType::Carrot,
                        value: "none".to_string(),
                    })
                }
                '+' => {
                    token = (Token {
                        kind: TokenType::Plus,
                        value: "none".to_string(),
                    })
                }
                ',' => {
                    token = (Token {
                        kind: TokenType::Comma,
                        value: "none".to_string(),
                    })
                }
                '.' => {
                    return Some(Token {
                        kind: TokenType::Period,
                        value: "none".to_string(),
                    })
                }
                '_' => {
                    return Some(Token {
                        kind: TokenType::Underscore,
                        value: "none".to_string(),
                    })
                }
                '-' => {
                    return Some(Token {
                        kind: TokenType::Minus,
                        value: "none".to_string(),
                    })
                }
                '#' => {
                    return Some(Token {
                        kind: TokenType::Hashtag,
                        value: "none".to_string(),
                    })
                }
                '=' => {
                    return Some(Token {
                        kind: TokenType::Equals,
                        value: "none".to_string(),
                    })
                }
                ':' => {
                    return Some(Token {
                        kind: TokenType::Colon,
                        value: "none".to_string(),
                    })
                }
                ';' => {
                    return Some(Token {
                        kind: TokenType::Semicolon,
                        value: "none".to_string(),
                    })
                }
                '<' => {
                    return Some(Token {
                        kind: TokenType::LeftCarrot,
                        value: "none".to_string(),
                    })
                }
                '>' => {
                    return Some(Token {
                        kind: TokenType::RightCarrot,
                        value: "none".to_string(),
                    })
                }
                '/' => {
                    if *self.reader.peek().unwrap() == '/' {
                        //inline comment
                        while let Some(&next) = self.next() {
                            if next == '\n' {
                                break;
                            } else {
                                continue;
                            }
                        }
                    } else if *self.reader.peek().unwrap() == '*' {
                        // block comment, ignore all words/chars until we see these again
                        while let Some(&next) = self.next() {
                            if next == '*' && *self.peek_token() == '/' {
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
        }
        return Some(token);
    }

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
    Quote,
    SingleQuote,
    Ampersand,
    EOF,
}
impl Token {
    pub fn new() -> Token {
        return Token {
            kind: TokenType::new(),
            value: "".to_string(),
        };
    }
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
    pub fn new() -> TokenType {
        return TokenType::new();
    }
    pub fn to_string(&self) -> String {
        match self {
            TokenType::Minus => return String::from("Minus/Dash"),
            TokenType::Plus => return String::from("Plus"),
            TokenType::Equals => return String::from("Equals"),
            TokenType::StringLiteral => return String::from("String Literal"),
            TokenType::NumberLiteral => return String::from("Number Literal"),
            TokenType::Operand => return String::from("Operand/Symbol"),
            TokenType::Quote => return String::from("DoubleQuote"),
            TokenType::SingleQuote => return String::from("Single Quote"),
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
