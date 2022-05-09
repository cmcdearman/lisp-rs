mod rules;

use crate::lexer::rules::unambiguous_single_char;
use crate::token::{Token, Span};
use crate::T;

pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut ret = Vec::new();
        let mut suffix = input;
        while !suffix.is_empty() {
            let token = self.next_token(suffix);
            ret.push(token);
            suffix = &suffix[token.len()..];
        }
        ret.push(Token {
            kind: T![EOF],
            span: Span {
                start: input.len() as u32,
                end:   input.len() as u32,
            },
        });
        ret
    }

    pub fn next_token(&self, input: &str) -> Token {
        self.valid_token(input).unwrap_or_else(|| self.invalid_token(input))
    }

    /// Returns `None` if the lexer cannot find a token at the start of `input`.
    fn valid_token(&self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if let Some(kind) = unambiguous_single_char(next) {
            (1, kind)
        } else {
            return None;
        };

        Some(Token {
            kind,
            // We will fix this later
            span: Span { start: 0, end: len },
        })
    }

    /// Always "succeeds", because it creates an error `Token`.
    fn invalid_token(&self, input: &str) -> Token {
        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());
        debug_assert!(len <= input.len());
        Token {
            kind: T![illegal],
            span: Span {
                start: 0,
                end:   len as u32,
            },
        }
    }
}