#[derive(Debug, Clone)]
pub enum Token {
    Illegal(String),
    Eof,
    Comment(String),

    Ident(String),
    Int(i32),
    String(String),

    Add(char), // +
    Sub(char), // -
    Mul(char), // *
    Quo(char), // /
    Mod(char), // %

    LParen(char), // (
    RParen(char), // )

    Let,
    // LAMBDA,
    // STRUCT,
    // IF,
    // ELSE
}
