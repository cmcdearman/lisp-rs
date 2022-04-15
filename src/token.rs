#[derive(Debug, Clone)]
pub enum Token {
    Illegal(String),
    Eof,
    // Comment(String),

    Ident(String),
    Number(i32),
    // String(String),

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
