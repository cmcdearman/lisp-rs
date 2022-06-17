use std::iter::Peekable;
use logos::Logos;
use crate::token::{LogosToken, Token, Span, TokenStream};

pub fn lex(src: &str) -> Peekable<TokenStream> {
   println!("{}", src);
   TokenStream::new(LogosToken::lexer(src).spanned().map(|(t, s)|
        Token { kind: t.kind(), span: Span::from(s) }).collect::<Vec<Token>>()).peekable()
}
