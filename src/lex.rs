use logos::Logos;
use crate::token::{Token, Span, TokenKind};

pub fn lex(src: &str) -> Vec<Token> {
    let mut last_span = Span::new(0, 0);
    let  mut tokens = TokenKind::lexer(src)
        .spanned()
        .map(|(t, s)| {
            let span = &mut last_span;
            *span = Span::from(s);
            Token {
                kind: t,
                span: *span,
                lit: src[*span].to_string(),
            }
        })
        .collect::<Vec<Token>>();
    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(last_span.end+1, last_span.end+1),
        lit: String::from("EOF"),
    });
    tokens
}
