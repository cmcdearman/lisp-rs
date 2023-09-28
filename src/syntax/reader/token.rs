use logos::Logos;
use std::fmt::{Debug, Display};

#[derive(Logos, Debug, Clone, Default, PartialEq)]
pub enum Token {
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Eof => "<EOF>",
                Token::Whitespace => "Whitespace",
                Token::Comment => "Comment",
                Token::Ident => "Ident",
                Token::Int => "Int",
                Token::Rational => "Rational",
                Token::Real => "Real",
                Token::Char => "Char",
                Token::String => "String",
                Token::LParen => "(",
                Token::RParen => ")",
                Token::LBrack => "[",
                Token::RBrack => "]",
                Token::LBrace => "{",
                Token::RBrace => "}",
                Token::Colon => ":",
                Token::Period => ".",
                Token::Comma => ",",
                Token::Hash => "#",
                Token::Quote => "'",
                Token::Backquote => "`",
            }
        )
    }
}
