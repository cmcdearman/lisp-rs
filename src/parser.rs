use crate::token::{Token, TokenKind, TokenStream};
use std::collections::VecDeque;
use std::iter::Peekable;
use std::path::Iter;
use crate::ast::{Ast, Atom, Literal, Sexpr};
use crate::T;

pub struct Parser<'src, I> where I: Iterator<Item = Token> {
    tokens: Peekable<I>,
    src: &'src str
}

impl<'src, I> Parser<'src, I> where I: Iterator<Item = Token> {
    pub fn new(tokens: Peekable<I>, src: &'src str) -> Self { Self { tokens, src } }

    fn peek(&mut self) -> TokenKind {
        self.tokens.peek().map(|token| token.kind).unwrap_or(T![EOF])
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn parse(&mut self) -> Ast {
        let mut list: VecDeque<Sexpr> = VecDeque::new();
        loop {
            list.push_back(self.parse_sexpr());
            if self.peek() == T![EOF] {
                return list;
            }
        }
    }

    fn parse_sexpr(&mut self) -> Sexpr {
        match self.peek() {
            T!['('] => self.parse_list(),
            _ => self.parse_atom()
        }
    }

    fn parse_list(&mut self) -> Sexpr {
        let mut list: VecDeque<Sexpr> = VecDeque::new();
        self.next();
        loop {
            list.push_back(self.parse_sexpr());
            if self.peek() == T![')'] {
                self.next();
                return Sexpr::List(list);
            }
        }
    }

    fn parse_atom(&mut self) -> Sexpr {
        match self.peek() {
            lit @ T![number] | lit @ T![string] => {
                let literal_text = {
                    // the calls on `self` need to be split, because `next` takes `&mut self`
                    // if `peek` is not `T![EOF]`, then there must be a next token
                    self.next().unwrap().text(self.src)
                };
                let lit = match lit {
                    T![number] => Literal::Number(
                        literal_text
                            .parse()
                            .expect(&format!(
                                "invalid floating point literal: `{}`",
                                literal_text)
                            ),
                    ),
                    T![string] => Literal::String(
                        // trim the quotation marks
                        literal_text[1..(literal_text.len() - 1)].to_string()
                    ),
                    _ => unreachable!(),
                };
                Sexpr::Atom(Atom::Literal(lit))
            }
            T![ident] | T![let] | T![lambda] => {
                Sexpr::Atom(Atom::Symbol({
                    self.next().unwrap().text(self.src).to_string()
                }))
            }
            T![+] | T![-] | T![*] | T![/] | T![%] => {
                Sexpr::Atom(Atom::Symbol({
                    self.next().unwrap().text(self.src).to_string()
                }))
            }
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}



