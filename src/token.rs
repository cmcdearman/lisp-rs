#[derive(Debug, Clone)]
pub enum Token {
    ILLEGAL(str),
    EOF(i32),
    COMMENT(str),

    IDENT(str),
    INT(i32),
    STRING(str),

    ADD(char), // +
    SUB(char), // -
    MUL(char), // *
    QUO(char), // /
    MOD(char), // %

    LPAREN(char), // (
    RPAREN(char), // )

    LET(str),
    LAMBDA(str)
    // STRUCT,
    // IF,
    // ELSE
}
