#[derive(Debug, Clone)]
pub enum Token {
    ILLEGAL(String),
    EOF(i32),
    COMMENT(String),

    IDENT(String),
    INT(i32),
    STRING(String),

    ADD(char), // +
    SUB(char), // -
    MUL(char), // *
    QUO(char), // /
    MOD(char), // %

    LPAREN(char), // (
    RPAREN(char), // )

    LET,
    LAMBDA,
    STRUCT,
    IF,
    ELSE
}
