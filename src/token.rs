#[derive(Debug, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Comment,

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

// #[macro_export]
// macro_rules! T {
//     [+] => {
//         $crate::token::TokenType::Add
//     };
// }