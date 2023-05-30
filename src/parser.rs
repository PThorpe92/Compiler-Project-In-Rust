use crate::lexer::Token;
use crate::lexer::TokenType;
use std::iter::Peekable;

pub struct Parser<R: Iterator<Item = Token>> {
    reader: Peekable<R>, // Our source of tokens (output of our lexer)
    token: Token,        // the current token being parsed
    span: Span,          // span represents the relative location in the source
                         // code that our current token resides. This is for error
                         // messages, warnings, diagnostics
}
pub struct AST {}
// We will implement the following funcitons on our Parser object:
// new: to create a new Parser object, and parse: to output the result
// of parsing each following token
impl<R: Iterator<Item = Token>> Parser<R> {
    pub fn new(mut it: R) -> Parser<R> {
        let first = it.next().unwrap();
        return Parser {
            token: first.token,
            span: first.span,
            reader: it.peekable(),
        };
    }
    pub fn parse(mut self) -> Result<AST, String> {
        // This is going to be done recursively, so we will have a function to advance
        // by one token at a time.

        // therefore this will return the next token
        fn advance(&mut self) -> Token {
            let next = self.reader.next();

            if let Some(Token { span, token }) = next {
                self.token = token;
                self.span = span;
            } else {
                self.token = Token {
                    kind: TokenType::EOF,
                    value: "none".to_string(),
                }
            }
        }
    }
}
