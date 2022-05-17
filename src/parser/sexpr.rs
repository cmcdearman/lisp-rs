use std::collections::VecDeque;
use crate::parser::{ast, Parser};
use crate::parser::ast::Sexpr;
use crate::T;
use crate::token::{Token, TokenKind};

impl<'input, I> Parser<'input, I> where I: Iterator<Item = Token> {
    pub fn parse(&mut self) -> VecDeque<ast::Sexpr> {
        let mut list: VecDeque<Sexpr> = VecDeque::new();
        loop {
            list.push_back(self.parse_sexpr());
            if self.peek() == T![EOF] {
                return list;
            }
        }
    }

    pub(crate) fn parse_sexpr(&mut self) -> ast::Sexpr {
        match self.peek() {
           T!['('] => self.parse_list(),
           _ => self.parse_atom()
        }
    }

    pub(crate) fn parse_list(&mut self) -> Sexpr {
        let mut list: VecDeque<Sexpr> = VecDeque::new();
        self.next();
        loop {
            list.push_back(self.parse_sexpr());
            if self.peek() == T![')'] {
                self.next();
                return ast::Sexpr::List(list);
            }
        }
    }

    pub(crate) fn parse_atom(&mut self) -> ast::Sexpr {
        match self.peek() {
            lit @ T![number] | lit @ T![string] => {
                let literal_text = {
                    // the calls on `self` need to be split, because `next` takes `&mut self`
                    // if `peek` is not `T![EOF]`, then there must be a next token
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };
                let lit = match lit {
                    T![number] => ast::Literal::Number(
                        literal_text
                            .parse()
                            .expect(&format!(
                                "invalid floating point literal: `{}`",
                                literal_text)
                            ),
                    ),
                    T![string] => ast::Literal::String(
                        // trim the quotation marks
                        literal_text[1..(literal_text.len() - 1)].to_string()
                    ),
                    _ => unreachable!(),
                };
                ast::Sexpr::Atom(ast::Atom::Literal(lit))
            }
            T![ident] => {
                ast::Sexpr::Atom(ast::Atom::Symbol({
                    let ident_token = self.next().unwrap();
                    self.text(ident_token).to_string() // <- now we need a copy
                }))
            }
            T![+] | T![-] | T![*] | T![/] | T![%] => {
                ast::Sexpr::Atom(ast::Atom::Symbol({
                    let tok = self.next().unwrap();
                    self.text(tok).to_string()
                }))
            }
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}
