type TokenType = u32;

struct Token {
    token_type: TokenType,
    lit: String
}

enum TokenTypes {
    ILLEGAL,
    EOF,
    COMMENT,

    IDENT,
    INT,
    STRING,

    ADD, // +
    SUB, // -
    MUL, // *
    QUO, // /
    MOD, // %

    LPAREN, // (
    RPAREN, // )

    LET,
    LAMBDA
    // STRUCT,
    // IF,
    // ELSE
}

impl Token {
    fn new() -> Self {
        Self { token_type: 0, lit: String::from("") }
    }
}