use std::{
    fmt,
    ops::{Index, Range},
};

use logos::{Lexer, Logos};
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};

use crate::{interner::InternedString, list::List};

pub mod reader;
mod tests;

#[derive(Logos, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[default]
    Eof,
    #[error]
    Err,
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r#";[^\n]*"#)]
    Comment,
    #[regex(r#"[^\[\]()\s,{};]+"#)]
    Ident,
    #[regex(r#"([1-9]\d*|0)"#, priority = 3)]
    Int,
    #[regex(r#"((\d+(\.\d+))|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 2)]
    Real,
    #[regex(r#"'\w'"#)]
    Char,
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,
    #[regex(r#"(true)|(false)"#)]
    Bool,

    #[regex(r#"(\+|-)?\d+/\d+"#)]
    Rational,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(":")]
    Colon,
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
}

#[macro_export]
macro_rules! T {
    [EOF] => {
        $crate::parser::TokenKind::Eof
    };
    [err] => {
        $crate::parser::TokenKind::Err
    };
    [ws] => {
        $crate::parser::TokenKind::Whitespace
    };
    [;] => {
        $crate::parser::TokenKind::Comment
    };
    [ident] => {
        $crate::parser::TokenKind::Ident
    };
    [int] => {
        $crate::parser::TokenKind::Int
    };
    [real] => {
        $crate::parser::TokenKind::Real
    };
    [ratio] => {
        $crate::parser::TokenKind::Rational
    };
    [char] => {
        $crate::parser::TokenKind::Char
    };
    [str] => {
        $crate::parser::TokenKind::String
    };
    [bool] => {
        $crate::parser::TokenKind::Bool
    };
    ['('] => {
        $crate::parser::TokenKind::LParen
    };
    [')'] => {
        $crate::parser::TokenKind::RParen
    };
    ['['] => {
        $crate::parser::TokenKind::LBrack
    };
    [']'] => {
        $crate::parser::TokenKind::RBrack
    };
    ['{'] => {
        $crate::parser::TokenKind::LBrace
    };
    ['}'] => {
        $crate::parser::TokenKind::RBrace
    };
    [:] => {
        $crate::parser::TokenKind::Colon
    };
    [.] => {
        $crate::parser::TokenKind::Period
    };
    [,] => {
        $crate::parser::TokenKind::Comma
    };
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![EOF] => "<EOF>",
                T![err] => "Error",
                T![ws] => "Whitespace",
                T![;] => "Comment",
                T![ident] => "Ident",
                T![int] => "Int",
                T![real] => "Real",
                T![ratio] => "Rational",
                T![char] => "Char",
                T![str] => "String",
                T![bool] => "Bool",
                T!['('] => "(",
                T![')'] => ")",
                T!['['] => "[",
                T![']'] => "]",
                T!['{'] => "{",
                T!['}'] => "}",
                T![:] => ":",
                T![.] => ".",
                T![,] => ",",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.start, self.end)
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start as usize..span.end as usize
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::from(index)]
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub fn lit<'a>(&self, src: &'a str) -> &'a str {
        &src[self.span]
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - {}", self.kind, self.span)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserError(pub InternedString);

impl ParserError {
    pub fn new(msg: &str) -> Self {
        Self(msg.into())
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", InternedString::from(self.0.key))
    }
}

pub type ParseResult<T> = std::result::Result<T, ParserError>;

// Parser entry point
pub fn expr(sexpr: &Sexpr) -> ParseResult<Expr> {
    match sexpr {
        Sexpr::Atom(a) => Ok(a.clone()),
        Sexpr::List(l) => {
            let mut iter = l.clone().into_iter();
            if let Some(first) = iter.next() {
                let lam = match first {
                    Sexpr::Atom(a) => match a {
                        Atom::Symbol(s) => match &*s.to_string() {
                            "lambda" => lambda(&mut iter),
                            "let" => parse_let(&mut iter),
                            "if" => parse_if(&mut iter),
                            _ => parse_apply(&mut iter),
                        },
                        _ => Err(ParserError::new("cannot apply non-lambda")),
                    },
                    Sexpr::List(_) => parse_apply(&mut iter),
                }?;
            } else {
                Ok(Expr::Unit)
            }
        }
    }

    fn lambda(list_iter: &mut ConsIter) -> ParseResult<Expr> {
        let params = list_iter
            .next()
            .ok_or(ParserError::new("lambda missing parameter list"))?;
        let body = list_iter
            .next()
            .ok_or(ParserError::new("lambda missing body"))?;
        Ok(Expr::Lambda {
            param: params,
            body: Box::new(expr(body)?),
        })
    }

    fn curry_fn(mut params: impl Iterator<Item = InternedString>, body: Expr) -> ParseResult<Expr> {
        Ok(params.fold(body, |acc, p| {
            Expr::Lit(Lit::Lambda {
                param: p,
                body: Box::new(acc),
            })
        }))
    }

    fn curry_apply(&mut self, args: impl Iterator<Item = Expr>, func: Expr) -> Expr {
        args.fold(func, |acc, arg| Expr::Apply(Box::new(acc), Box::new(arg)))
    }
}
