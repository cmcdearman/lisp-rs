use std::iter::Peekable;
use crate::token::{Span, Token, TokenKind, TokenStream};
use logos::Logos;

pub fn lex(src: &str) -> Peekable<TokenStream> {
    let mut last_span = Span::new(0, 0);
    let mut tokens: Vec<Token> = TokenKind::lexer(src)
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
        .collect();
    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(last_span.end + 1, last_span.end + 1),
        lit: String::from("EOF"),
    });
    TokenStream::new(tokens).peekable()
}
