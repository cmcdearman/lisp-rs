use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::token::{Token, TokenKind, TokenStream};
use std::iter::Peekable;
use crate::ast::{Atom, Literal, Sexpr};
use crate::{lex, T};

pub struct ParseError {
    err: String
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

pub struct Parser<'src> {
    tokens: Peekable<TokenStream>,
    src: &'src str
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self { Self { tokens: lex(src), src } }

    fn peek(&mut self) -> TokenKind {
        self.tokens.peek().map(|token| token.kind).unwrap_or(T![EOF])
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn parse(&mut self) -> Sexpr {
        match self.peek() {
            T!['('] =>  { self.next(); self.parse_list() },
            _ => self.parse_atom()
        }
    }

    fn parse_list(&mut self) -> Sexpr {
        let car = self.parse();
        let cdr: Sexpr;
        
        if self.peek() == T![')'] {
            self.next();
            cdr = Sexpr::Nil;              
        } else {
            cdr = self.parse_list();
        }
       
        Sexpr::Cons { car: Box::new(car), cdr: Box::new(cdr) }
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
