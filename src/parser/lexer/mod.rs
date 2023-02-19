use logos::Logos;

use self::token::{TokenKind, Token};

pub mod token;

pub struct Lexer<'src> {
    logos: logos::SpannedIter<'src, TokenKind>,
    skip_comments: bool,
    eof: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str, skip_comments: bool) -> Self {
        Self {
            logos: TokenKind::lexer(src).spanned(),
            skip_comments,
            eof: false,
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.logos.next() {
            Some((t, s)) => {
                if t == TokenKind::Comment && self.skip_comments {
                    return self.next();
                } else {
                    return Some(Token {
                        kind: t,
                        span: s.into(),
                    });
                }
            }
            None if self.eof => None,
            None => {
                self.eof = true;
                Some(Token {
                    kind: TokenKind::Eof,
                    span: (0..0).into(),
                })
            }
        }
    }
}

// mod tests {
//     #[test]
//     fn test_lex() {
//         let input = "(+ 3 4) ; test comment";
//     }
// }