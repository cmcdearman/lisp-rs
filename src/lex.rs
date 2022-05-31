use std::iter::Peekable;
use std::path::Iter;
use logos::{Span as LogosSpan, SpannedIter, Logos};
use crate::token::{LogosToken, TokenKind, Token, Span, TokenStream};

pub fn lex(src: &str) -> TokenStream {
   TokenStream::new(LogosToken::lexer(src).spanned().map(|(t, s)|
        Token { kind: t.kind(), span: Span::from(s)}).collect::<Vec<Token>>())
}