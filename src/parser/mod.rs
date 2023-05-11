use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Index, Range},
};

use logos::{Lexer, Logos};
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};

use crate::{interner::InternedString, list::List};

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

impl Display for Span {
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

#[derive(Clone, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    List(ConsList),
}

impl Debug for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(a) => write!(f, "{:?}", a),
            Self::List(l) => write!(f, "{:?}", l),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ConsList {
    pub head: Option<Box<Cons>>,
}

impl ConsList {
    pub const NIL: Self = Self { head: None };

    pub fn new(head: Option<Box<Cons>>) -> Self {
        Self { head }
    }
}

impl Debug for ConsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.head {
            Some(h) => write!(f, "{:?}", h),
            None => write!(f, "Nil"),
        }
    }
}

impl IntoIterator for ConsList {
    type Item = Sexpr;

    type IntoIter = ConsIter;

    fn into_iter(self) -> Self::IntoIter {
        ConsIter(self.head.clone())
    }
}

#[derive(Clone, PartialEq)]
pub struct Cons {
    pub car: Sexpr,
    pub cdr: Option<Box<Cons>>,
}

impl Debug for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cdr {
            Some(cdr) => write!(f, "({:?} . {:?})", self.car, cdr.as_ref()),
            None => write!(f, "({:?} . Nil)", self.car),
        }
    }
}

#[derive(Clone)]
pub struct ConsIter(Option<Box<Cons>>);

impl Iterator for ConsIter {
    type Item = Sexpr;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.clone().map(|cons| {
            let sexpr = cons.car.clone();

            self.0 = cons.cdr.clone();

            sexpr
        })
    }
}

impl ExactSizeIterator for ConsIter {
    fn len(&self) -> usize {
        let mut length: usize = 0;

        self.clone().for_each(|_| length += 1);

        length
    }
}

#[derive(Clone, PartialEq)]
pub enum Atom {
    Lit(Lit),
    Symbol(InternedString),
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(l) => write!(f, "{:?}", l),
            Self::Symbol(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Lit {
    Number(Number),
    String(InternedString),
    Char(char),
    Bool(bool),
}

impl Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{:?}", n),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Char(c) => write!(f, "{:?}", c),
            Self::Bool(b) => write!(f, "{:?}", b),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    BigInt(BigInt),
    Real(f64),
    Rational(Rational64),
    BigRational(BigRational),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReaderError(pub String);

impl ReaderError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

pub type ReadResult<T> = std::result::Result<T, ReaderError>;

/// Parser is a recursive descent parser for the Lust language.
pub struct Reader<'src> {
    /// The source code to parse.
    src: &'src str,

    /// The [`Lexer`] used to lex the source code.
    logos: Lexer<'src, TokenKind>,

    /// The next token to be consumed.
    peek: Option<Token>,
}

impl<'src> Reader<'src> {
    /// Creates a new [`Parser`].
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            logos: TokenKind::lexer(src),
            peek: None,
        }
    }

    /// Returns the source code of the token.
    fn text(&self, token: Token) -> &'src str {
        token.lit(&self.src)
    }

    /// Returns the peek token in the stream.
    fn next(&mut self) -> Token {
        if let Some(t) = self.peek.take() {
            t
        } else {
            self.generate()
        }
    }

    /// Returns the next token in the stream without consuming it.
    fn peek(&mut self) -> Token {
        if let Some(t) = self.peek {
            t
        } else {
            let t = self.generate();
            self.peek = Some(t);
            t
        }
    }

    /// Gets the next token from the [`Lexer`].
    fn generate(&mut self) -> Token {
        match self.logos.next().map(|t| (t, self.logos.span())) {
            None => Token {
                kind: T![EOF],
                span: Span::new(0, 0),
            },
            Some((T![;], _)) => self.generate(),
            Some((t, s)) => Token {
                kind: t,
                span: Span::from(s),
            },
        }
    }

    /// Returns true if the next token is of the given kind.
    fn at(&mut self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    /// Consumes the next token if it is of the given kind.
    fn consume(&mut self, expected: TokenKind) {
        let token = self.next();
        assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}`",
            expected, token.kind
        );
    }

    /// Parses the source code into a [`Sexpr`].
    pub fn sexpr(&mut self) -> ReadResult<Sexpr> {
        match self.peek().kind {
            T!['('] => self.list(),
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> ReadResult<Sexpr> {
        self.consume(T!['(']);
        let mut sexprs = vec![];
        while !self.at(T![')']) {
            sexprs.push(self.sexpr()?);
        }
        self.consume(T![')']);
        Ok(Sexpr::List(ConsList::new(
            sexprs
                .into_iter()
                .rev()
                .fold(None, |cdr, car| Some(Box::new(Cons { car, cdr }))),
        )))
    }

    fn atom(&mut self) -> ReadResult<Sexpr> {
        match self.peek().kind {
            T![int] | T![real] | T![ratio] | T![char] | T![str] | T![bool] => {
                Ok(Sexpr::Atom(Atom::Lit(self.lit()?)))
            }
            T![ident] => Ok(Sexpr::Atom(Atom::Symbol(self.symbol()?))),
            _ => Err(ReaderError(format!(
                "Unexpected token in atom `{}`",
                self.peek().kind
            ))),
        }
    }

    fn lit(&mut self) -> ReadResult<Lit> {
        match self.peek().kind {
            T![int] => Ok(Lit::Number(self.int()?)),
            T![real] => Ok(Lit::Number(self.real()?)),
            T![ratio] => Ok(Lit::Number(self.rational()?)),
            T![char] => Ok(Lit::Char(self.char()?)),
            T![str] => Ok(Lit::String(self.string()?)),
            T![bool] => Ok(Lit::Bool(self.bool()?)),
            _ => Err(ReaderError(format!(
                "Unexpected token in literal `{}`",
                self.peek().kind
            ))),
        }
    }

    fn int(&mut self) -> ReadResult<Number> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| ReaderError::new("Failed to parse integer"))?;
        Ok(Number::Int(num))
    }

    fn real(&mut self) -> ReadResult<Number> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| ReaderError::new("Failed to parse float"))?;
        Ok(Number::Real(num))
    }

    fn rational(&mut self) -> ReadResult<Number> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| ReaderError::new("Failed to parse rational"))?;
        Ok(Number::Rational(num))
    }

    fn string(&mut self) -> ReadResult<InternedString> {
        let token = self.next();
        let text = self.text(token);
        Ok(InternedString::from(&text[1..(text.len() - 1)]))
    }

    fn char(&mut self) -> ReadResult<char> {
        let token = self.next();
        let text = self.text(token);
        Ok(text
            .chars()
            .nth(1)
            .ok_or(ReaderError::new("Failed to parse char"))?)
    }

    fn bool(&mut self) -> ReadResult<bool> {
        let token = self.next();
        let text = self.text(token);
        Ok(text == "#t")
    }

    fn symbol(&mut self) -> ReadResult<InternedString> {
        let token = self.next();
        let text = self.text(token);
        Ok(InternedString::from(text))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    List(List<Self>),
    Lambda {
        param: Vec<Self>,
        body: Box<Self>,
    },
    Apply {
        func: Box<Self>,
        arg: Box<Self>,
    },
    Let {
        name: InternedString,
        value: Box<Self>,
        body: Box<Self>,
    },
    If {
        cond: Box<Self>,
        then: Box<Self>,
        else_: Box<Self>,
    },
    Unit,
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
