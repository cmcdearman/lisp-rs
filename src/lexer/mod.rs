mod rules;

use crate::lexer::rules::unambiguous_single_char;
use crate::token::{Token, Span};
use crate::T;

pub struct Lexer<'input> {
    input: &'input str,
    position: u32,
    eof: bool
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0, eof: false }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
       self.collect()
    }

    pub fn next_token(&mut self, input: &str) -> Token {
        self.valid_token(input).unwrap_or_else(|| self.invalid_token(input))
    }

    /// Returns `None` if the lexer cannot find a token at the start of `input`.
    fn valid_token(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if next.is_whitespace() {
            (
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_whitespace())
                    .last()
                    .unwrap() // we know there is at least one whitespace character
                    .0 as u32
                    + 1,
                T![ws],
            )
        } else if let Some(kind) = unambiguous_single_char(next) {
            (1, kind)
        } else {
            return None;
        };

        let start = self.position;
        self.position += len;
        Some(Token {
            kind,
            span: Span {
                start,
                end: start + len,
            },
        })
    }

    /// Always "succeeds", because it creates an error `Token`.
    fn invalid_token(&mut self, input: &str) -> Token {
        let start = self.position;
        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());
        debug_assert!(len <= input.len());
        let len = len as u32;
        self.position = start + len;
        Token {
            kind: T![illegal],
            span: Span {
                start,
                end: start + len,
            },
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position as usize >= self.input.len() {
            if self.eof {
                return None;
            }
            self.eof = true;
            Some(Token {
                kind: T![EOF],
                span: Span {
                    start: self.position,
                    end:   self.position,
                },
            })
        } else {
            Some(self.next_token(&self.input[self.position as usize..]))
        }
    }
}

// /// walks `$tokens` and compares them to the given kinds.
// macro_rules! assert_tokens {
//     ($tokens:ident, [$($kind:expr,)*]) => {
//         {
//             let mut it = $tokens.iter();
//             $(
//                 let token = it.next().expect("not enough tokens");
//                 assert_eq!(token.kind, $kind);
//             )*
//         }
//     };
// }
//
// #[test]
// fn single_char_tokens() {
//     let lexer = Lexer::new();
//     let input = "+-()";
//     let tokens = lexer.tokenize(input);
//     assert_tokens!(tokens, [T![+], T![-], T!['('], T![')'], T![EOF]]);
// }
//
// #[test]
// fn unknown_input() {
//     let lexer = Lexer::new();
//     let input = "{$$$$$$$+";
//     let tokens = lexer.tokenize(input);
//     assert_tokens!(tokens, [T![illegal], T![+], T![EOF]]);
// }