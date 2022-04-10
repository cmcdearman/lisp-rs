#[derive(Debug, Clone)]
pub enum Token {
    Illegal(String),
    Eof,
    Comment(String),

    Ident(String),
    Int(i32),
    String(String),

    Add, // +
    Sub, // -
    Mul, // *
    Quo, // /
    Mod, // %

    LParen, // (
    RParen, // )

    Let,
    // LAMBDA,
    // STRUCT,
    // IF,
    // ELSE
}
