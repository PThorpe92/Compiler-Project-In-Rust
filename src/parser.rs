/* VERY beginning stages, very little work done yet as lexer is reworked */
use crate::lexer::Span;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::lexer::TS;
use std::iter::Peekable;

pub struct Parser<R: Iterator<Item = TS>> {
    reader: Peekable<R>, // Our source of tokens
    token: Token,        // the current token being parsed
    span: Span,          // span represents the relative location in the source
                         // code that our current token resides. This is for error
                         // messages, warnings, diagnostics
}
macro_rules! enum_from_impl {
    ($enum_type:ident, $(($enum_variant:ident, $inner_type:ty)),*) => {
        $(
            impl From<$inner_type> for $enum_type {
                fn from(x: $inner_type) -> $enum_type {
                    $enum_type::$enum_variant(x)
                }
            }
        )*
    }
}

enum_from_impl!(Statement,
                (Decl, DeclStmt),
                (Labeled, LabeledStmt),
                (Simple, SimpleStmt),
                (Return, ReturnStmt),
                (Break, BreakStmt),
                (Continue, ContinueStmt),
                (Fallthrough, FallthroughStmt),
                (Block, Block),
                (If, IfStmt),
                (Select, SelectStmt),
                (For, ForStmt),
                (Defer, DeferStmt),
                (Empty, EmptyStmt));


/// A simple statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleStmt {
    EmptyStmt,
    Expr(Spanned<Expr>),
    Send(SendStmt),
    IncDec(IncDecStmt),
    Assignment(Assignment),
    ShortVarDecl(ShortVarDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStmt {
    pub header: ForHeader,
    pub body: Block,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForHeader {
    Condition(Expr),
    ForClause(ForClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForClause {
    pub init: Option<SimpleStmt>,
    pub condition: Option<Expr>,
    pub post: Option<SimpleStmt>,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IterVars {
    Exprs(Vec<Spanned<Expr>>),
    Idents(Vec<Spanned<String>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub lhs: Vec<Spanned<Expr>>,
    pub rhs: Vec<Spanned<Expr>>,
    pub op: Option<BinaryOperator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyStmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclStmt {
    Const(ConstDecl),
    TypeDecl(TypeDecl),
    VarDecl(VarDecl),
}

pub enum Statement {
    Decl(DeclStmt),
    Labeled(LabeledStmt),
    Simple(SimpleStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Block(Block),
    If(IfStmt),
    Select(SelectStmt),
    For(ForStmt),
    Defer(DeferStmt),
    Empty(EmptyStmt),
}
// We will implement the following funcitons on our Parser object:
// new: to create a new Parser object, and parse: to output the result
// of parsing each following token
impl<R: Iterator<Item = TS>> Parser<R> {
    pub fn new(mut it: R) -> Parser<R> {
        let first = it.next().unwrap();
        return Parser {
            token: first.token,
            span: first.span,
            reader: it.peekable(),
        };
    }
    pub fn parse(mut self) -> Result<AST, String> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    BitClear,

    LeftShift,
    RightShift,

    Equals,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LogAnd,
    LogOr,
}

impl BinaryOperator {
    pub fn from_token_kind(tok: TokenKind) -> Option<BinaryOperator> {
        use self::BinaryOperator::*;
        Some(match tok {
            TokenKind::Plus => Add,
            TokenKind::Minus => Sub,
            TokenKind::Star => Mul,
            TokenKind::FwdSlash => Div,
            TokenKind::Percent => Rem,
            TokenKind::And => BitAnd,
            TokenKind::Or => BitOr,
            TokenKind::Caret => BitXor,
            TokenKind::BitClear => BitClear,
            TokenKind::Lshift => LeftShift,
            TokenKind::Rshift => RightShift,
            TokenKind::Equals => Equals,
            TokenKind::NotEqual => NotEqual,
            TokenKind::LessThan => LessThan,
            TokenKind::LeftCaret => LessThanOrEqual,
            TokenKind::GreaterThan => GreaterThan,
            TokenKind::GreaterThanOrEqual => GreaterThanOrEqual,
            TokenKind::AndAnd => LogAnd,
            TokenKind::OrOr => LogOr,

            _ => return None,
        })
    }

    pub fn from_token_kind_assign_op(tok: TokenKind) -> Option<BinaryOperator> {
        use self::BinaryOperator::*;
        Some(match tok {
            TokenKind::PlusAssign => Add,
            TokenKind::MinusAssign => Sub,
            TokenKind::StarAssign => Mul,
            TokenKind::SlashAssign => Div,
            TokenKind::PercentAssign => Rem,

            TokenKind::AndAssign => BitAnd,
            TokenKind::OrAssign => BitOr,
            TokenKind::CaretAssign => BitXor,
            TokenKind::BitClearAssign => BitClear,

            TokenKind::LshiftAssign => LeftShift,
            TokenKind::RshiftAssign => RightShift,

            _ => return None,
        })
    }

    pub fn precedence(self) -> i32 {
        use self::BinaryOperator::*;

        match self {
            Mul | Div | Rem | LeftShift | RightShift | BitAnd | BitClear => 5,
            Add | Sub | BitOr | BitXor => 4,
            Equals | NotEqual | LessThan | LessThanOrEqual | GreaterThan | GreaterThanOrEqual => 3,
            LogAnd => 2,
            LogOr => 1,
        }
    }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub lhs: Box<Spanned<Expr>>,
    pub op: BinaryOperator,
    pub rhs: Box<Spanned<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpr {
    Primary(Box<PrimaryExpr>),
    UnaryOperation(UnaryOperation),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Spanned<UnaryExpr>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    Xor,
    Deref,
    And,
}

impl UnaryOperator {
    pub fn from_token_kind(k: TokenKind) -> Option<UnaryOperator> {
        use self::UnaryOperator::*;

        Some(match k {
            TokenKind::Plus => Plus,
            TokenKind::Minus => Minus,
            TokenKind::Not => Not,
            TokenKind::Caret => Xor,
            TokenKind::Star => Deref,
            TokenKind::And => And,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryExpr {
    Operand(Operand),
    Conversion(Conversion),
    SelectorExpr(SelectorExpr),
    Indexing(IndexExpr),
    Slicing(SliceExpr),
    TypeAssertion(TypeAssertion),
    FuncCall(FuncCall),
}

/// Operands denote the elementary values in an expression. An operand may be a literal, a
/// (possibly qualified) non-blank identifier denoting a constant, variable, or function, a method
/// expression yielding a function, or a parenthesized expression.
// XXX/FIXME/TODO: not finished.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Lit(Literal),
    MethodExpr(MethodExpr),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectorExpr {
    pub operand: Box<PrimaryExpr>,
    pub selector: Ident,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexExpr {
    pub operand: Box<Spanned<PrimaryExpr>>,
    pub index: Spanned<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceExpr {
    pub operand: Box<Spanned<PrimaryExpr>>,
    pub slicing: Slicing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slicing {
    pub low: Spanned<Expr>,
    pub high: Spanned<Expr>,
    pub max: Option<Spanned<Expr>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAssertion {
    pub expr: Box<PrimaryExpr>,
    pub typ: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncCall {
    pub callee: Box<Spanned<PrimaryExpr>>,
    pub args: Arguments,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodExpr {
    /// Receiver type.
    pub receiver: Type,
    /// Name of the method.
    pub name: String,
}
