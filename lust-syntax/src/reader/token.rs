use logos::Logos;
use lust_util::span::Spanned;
use std::fmt::{Debug, Display};

#[derive(Logos, Debug, Clone, Default, PartialEq)]
pub enum TokenKind {
    #[default]
    Eof,
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r#";[^\n]*"#)]
    Comment,
    #[regex(r#"[^\[\]()\s,{};]+"#)]
    Ident,
    #[regex(r#"([1-9]\d*|0)"#, priority = 3)]
    Int,
    #[regex(r#"(\+|-)?\d+/\d+"#)]
    Rational,
    #[regex(r#"((\d+(\.\d+))|(\.\d+))([Ee](\+|-)?\d+)?"#, priority = 2)]
    Real,
    #[regex(r#"'\w'"#)]
    Char,
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,

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
    #[token("#")]
    Hash,
    #[token("'")]
    Quote,
    #[token("`")]
    Backquote,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::Eof => "<EOF>",
                TokenKind::Whitespace => "Whitespace",
                TokenKind::Comment => "Comment",
                TokenKind::Ident => "Ident",
                TokenKind::Int => "Int",
                TokenKind::Rational => "Rational",
                TokenKind::Real => "Real",
                TokenKind::Char => "Char",
                TokenKind::String => "String",
                TokenKind::LParen => "(",
                TokenKind::RParen => ")",
                TokenKind::LBrack => "[",
                TokenKind::RBrack => "]",
                TokenKind::LBrace => "{",
                TokenKind::RBrace => "}",
                TokenKind::Colon => ":",
                TokenKind::Period => ".",
                TokenKind::Comma => ",",
                TokenKind::Hash => "#",
                TokenKind::Quote => "'",
                TokenKind::Backquote => "`",
            }
        )
    }
}

pub type Token = Spanned<TokenKind>;
