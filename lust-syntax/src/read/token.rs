use logos::Logos;
use lust_utils::{intern::InternedString, num::{Float, BigInt, BigRational}};
use std::fmt::{Debug, Display};

#[derive(Logos, Debug, Clone, Default, PartialEq)]
pub enum Token {
    Eof,
    #[default]
    Error,
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r#";[^\n]*"#)]
    Comment,
    #[regex(r#"[^.'\d\[\]()\s,{};][^.'\[\]()\s,{};]*"#, |lex| InternedString::from(lex.slice()))]
    Ident(InternedString),
    #[regex(
        r#"(0b[0-1]+)|(0o[0-7]+)|(0x[0-9a-fA-F]+)|([1-9]\d*|0)"#, 
        priority = 2, 
        callback = |lex| lex.slice().parse::<BigInt>().ok()
    )]
    Int(BigInt),
    #[regex(
        r#"([1-9]\d*|0)(\.\d+)?([eE][+-]?\d+)?"#, 
        priority = 1, 
        callback = |lex| lex.slice().parse::<Float>().ok()
    )]
    Float(Float),
    #[regex(
        r#"((0b[0-1]+)|(0o[0-7]+)|(0x[0-9a-fA-F]+)|([1-9]\d*|0))(/-?((0b[0-1]+)|(0o[0-7]+)|(0x[0-9a-fA-F]+)|([1-9]\d*|0)))?"#,
        priority = 0,
        callback = |lex| lex.slice().parse::<BigRational>().ok()
    )]
    Rational(BigRational),
    #[regex(r#""("[^"\\]*(?:\\.[^"\\]*)*")""#, |lex| InternedString::from(lex.slice()))]
    String(InternedString),

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
    #[token("...")]
    Ellipsis,
    #[token(",")]
    Comma,
    #[token(",@")]
    CommaAt,
    #[token("#")]
    Hash,
    #[token("#[")]
    HashLBrack,
    #[token("'")]
    Quote,
    #[token("`")]
    Backquote,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Eof => write!(f, "EOF"),
            Error => write!(f, "Error"),
            Whitespace => write!(f, "Whitespace"),
            Comment => write!(f, "Comment"),
            Ident(name) => write!(f, "Ident({})", name),
            Int(n) => write!(f, "Int({})", n),
            Float(n) => write!(f, "Float({})", n),
            Rational(n) => write!(f, "Rational({})", n),
            String(s) => write!(f, "String({})", s),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LBrack => write!(f, "["),
            RBrack => write!(f, "]"),
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            Colon => write!(f, ":"),
            Period => write!(f, "."),
            Ellipsis => write!(f, "..."),
            Comma => write!(f, ","),
            CommaAt => write!(f, ",@"),
            Hash => write!(f, "#"),
            HashLBrack => write!(f, "#["),
            Quote => write!(f, "'"),
            Backquote => write!(f, "`"),
        }
    }
}
