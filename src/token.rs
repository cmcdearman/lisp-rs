use std::{fmt, ops::{Range, Index}};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    Illegal,
    Eof,
    Comment,
    Whitespace,

    // Literals
    Ident,
    Number,
    String,

    // Operators
    Add,    // +
    Sub,    // -
    Mul,    // *
    Quo,    // /
    Mod,    // %

    // Parentheses
    LParen, // (
    RParen, // )

    // Keywords
    Let,    // let
    Lambda, // lambda
}

#[macro_export]
macro_rules! T {
    [illegal] => {
        $crate::token::TokenKind::Illegal
    };
    [EOF] => {
        $crate::token::TokenKind::Eof
    };
    [comment] => {
        $crate::token::TokenKind::Comment
    };
    [ws] => {
        $crate::token::TokenKind::Whitespace
    };
    [ident] => {
        $crate::token::TokenKind::Ident
    };
    [number] => {
        $crate::token::TokenKind::Number
    };
    [string] => {
        $crate::token::TokenKind::String
    };
    [+] => {
        $crate::token::TokenKind::Add
    };
    [-] => {
        $crate::token::TokenKind::Sub
    };
    [*] => {
        $crate::token::TokenKind::Mul
    };
    [/] => {
        $crate::token::TokenKind::Quo
    };
    [%] => {
        $crate::token::TokenKind::Mod
    };
    ['('] => {
        $crate::token::TokenKind::LParen
    };
    [')'] => {
        $crate::token::TokenKind::RParen
    };
    [let] => {
        $crate::token::TokenKind::Let
    };
    [lambda] => {
        $crate::token::TokenKind::Lambda
    };
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![illegal] => "<ILLEGAL>",
                T![EOF] => "<EOF>",
                T![comment] => "Comment",
                T![ws] => "<WHITESPACE>",
                T![ident] => "Ident",
                T![number] => "Number",
                T![string] => "String",
                T![+] => "+",
                T![-] => "-",
                T![*] => "*",
                T![/] => "/",
                T![%] => "%",
                T!['('] => "(",
                T![')'] => ")",
                T![let] => "let",
                T![lambda] => "lambda",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn token_type_display() {
        assert_eq!(T![illegal].to_string(), "<ILLEGAL>");
        assert_eq!(T![EOF].to_string(), "<EOF>");
        assert_eq!(T![comment].to_string(), "Comment");
        assert_eq!(T![ws].to_string(), "<WHITESPACE>");
        assert_eq!(T![ident].to_string(), "Ident");
        assert_eq!(T![number].to_string(), "Number");
        assert_eq!(T![string].to_string(), "String");
        assert_eq!(T![+].to_string(), "+");
        assert_eq!(T![-].to_string(), "-");
        assert_eq!(T![*].to_string(), "*");
        assert_eq!(T![/].to_string(), "/");
        assert_eq!(T![%].to_string(), "%");
        assert_eq!(T!['('].to_string(), "(");
        assert_eq!(T![')'].to_string(), ")");
        assert_eq!(T![let].to_string(), "let");
        assert_eq!(T![lambda].to_string(), "lambda");
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Default, Debug)]
pub struct Span {
    /// inclusive
    pub start: u32,
    /// exclusive
    pub end:   u32,
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
            end:   range.end as u32,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - <{}, {}>", self.kind, self.span.start, self.span.end)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}