use std::fmt::{Debug, Display};

use logos::{Lexer, Logos};
use num_bigint::BigInt;
use num_rational::{BigRational, Rational64};

use crate::{
    interner::InternedString,
    token::{Span, Token, TokenKind},
    T,
};

#[derive(Clone, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    List(List),
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
pub struct List {
    head: Option<Box<Cons>>,
}

impl List {
    pub const NIL: Self = Self { head: None };

    pub fn new(head: Option<Box<Cons>>) -> Self {
        Self { head }
    }
}

// impl Display for List {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self.head {
//             Some(h) => write!(f, "({})", h),
//             None => write!(f, "Nil"),
//         }
//     }
// }

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.head {
            Some(h) => write!(f, "{:?}", h),
            None => write!(f, "Nil"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Cons {
    car: Sexpr,
    cdr: Option<Box<Cons>>,
}

// impl Display for Cons {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self.cdr {
//             Some(cdr) => write!(f, "{} {}", self.car, cdr.as_ref()),
//             None => write!(f, "{}", self.car),
//         }
//     }
// }

impl Debug for Cons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cdr {
            Some(cdr) => write!(f, "({:?} . {:?})", self.car, cdr.as_ref()),
            None => write!(f, "({:?} . Nil)", self.car),
        }
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
    Float(f64),
    // BigFloat(),
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

pub type Result<T> = std::result::Result<T, ReaderError>;

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
    pub fn sexpr(&mut self) -> Result<Sexpr> {
        match self.peek().kind {
            T!['('] => self.list(),
            _ => self.atom(),
        }
    }

    fn list(&mut self) -> Result<Sexpr> {
        self.consume(T!['(']);
        let mut sexprs = vec![];
        while !self.at(T![')']) {
            sexprs.push(self.sexpr()?);
        }
        self.consume(T![')']);
        Ok(Sexpr::List(List::new(
            sexprs
                .into_iter()
                .rev()
                .fold(None, |cdr, car| Some(Box::new(Cons { car, cdr }))),
        )))
    }

    fn atom(&mut self) -> Result<Sexpr> {
        match self.peek().kind {
            T![int] | T![float] | T![ratio] | T![char] | T![str] | T![bool] => {
                Ok(Sexpr::Atom(Atom::Lit(self.lit()?)))
            }
            T![ident] => Ok(Sexpr::Atom(Atom::Symbol(self.symbol()?))),
            _ => Err(ReaderError(format!(
                "Unexpected token in atom `{}`",
                self.peek().kind
            ))),
        }
    }

    fn lit(&mut self) -> Result<Lit> {
        match self.peek().kind {
            T![int] => Ok(Lit::Number(self.int()?)),
            T![float] => Ok(Lit::Number(self.float()?)),
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

    fn int(&mut self) -> Result<Number> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| ReaderError::new("Failed to parse integer"))?;
        Ok(Number::Int(num))
    }

    fn float(&mut self) -> Result<Number> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| ReaderError::new("Failed to parse float"))?;
        Ok(Number::Float(num))
    }

    fn rational(&mut self) -> Result<Number> {
        let token = self.next();
        let text = self.text(token);
        let num = text
            .parse()
            .map_err(|_| ReaderError::new("Failed to parse rational"))?;
        Ok(Number::Rational(num))
    }

    fn string(&mut self) -> Result<InternedString> {
        let token = self.next();
        let text = self.text(token);
        Ok(InternedString::from(&text[1..(text.len() - 1)]))
    }

    fn char(&mut self) -> Result<char> {
        let token = self.next();
        let text = self.text(token);
        Ok(text
            .chars()
            .nth(1)
            .ok_or(ReaderError::new("Failed to parse char"))?)
    }

    fn bool(&mut self) -> Result<bool> {
        let token = self.next();
        let text = self.text(token);
        Ok(text == "#t")
    }

    fn symbol(&mut self) -> Result<InternedString> {
        let token = self.next();
        let text = self.text(token);
        Ok(InternedString::from(text))
    }
}

#[test]
fn test_read_int() {
    let mut reader = Reader::new("12");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_float() {
    let mut reader = Reader::new("12.2");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_rational() {
    let mut reader = Reader::new("12/2");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_char() {
    let mut reader = Reader::new("\'a\'");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_string() {
    let mut reader = Reader::new("\"Hello, world!\"");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_bool() {
    let mut reader = Reader::new("true");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_symbol() {
    let mut reader = Reader::new("foo");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_add_list() {
    let mut reader = Reader::new("(+ 1 2)");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}

#[test]
fn test_read_nested_list() {
    let mut reader = Reader::new("(+ (% 5 3) 1 2)");
    let sexpr = reader.sexpr().expect("Failed to read sexpr");
    insta::assert_debug_snapshot!(sexpr);
}
