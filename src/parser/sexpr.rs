use crate::parser::{ast, Parser};
use crate::T;
use crate::token::{Token, TokenKind};

impl<'input, I> Parser<'input, I> where I: Iterator<Item = Token> {
    pub fn parse_sexpr(&mut self) -> Vec<ast::Sexpr> {
       todo!()
    }

    pub fn parse_atom(&mut self) -> ast::Sexpr {
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
            kind => {
                panic!("Unknown start of atom: `{}`", kind);
            }
        }
    }
}
