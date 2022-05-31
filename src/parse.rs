use crate::token::{Token, TokenStream};
use std::collections::VecDeque;
use std::iter::Peekable;
use std::path::Iter;
use crate::ast::{Ast, Atom, Literal, Sexpr};
use crate::T;

pub fn parse(tok_stream: &mut TokenStream, src: &str) -> Ast {
    let mut list: VecDeque<Sexpr> = VecDeque::new();
    loop {
        list.push_back(parse_sexpr(tok_stream, src));
        if tok_stream.peek() == T![EOF] {
            return list;
        }
    }
}

fn parse_sexpr(tok_stream: &mut TokenStream, src: &str) -> Sexpr {
    match tok_stream.peek() {
        T!['('] => parse_list(tok_stream, src),
        _ => parse_atom(tok_stream, src)
    }
}

fn parse_list(tok_stream: &mut TokenStream, src: &str) -> Sexpr {
    let mut list: VecDeque<Sexpr> = VecDeque::new();
    tok_stream.next();
    loop {
        list.push_back(parse_sexpr(tok_stream, src));
        if tok_stream.peek() == T![')'] {
            tok_stream.next();
            return Sexpr::List(list);
        }
    }
}

fn parse_atom(tok_stream: &mut TokenStream, src: &str) -> Sexpr {
        match tok_stream.peek() {
        lit @ T![number] | lit @ T![string] => {
            let literal_text = {
                // the calls on `self` need to be split, because `next` takes `&mut self`
                // if `peek` is not `T![EOF]`, then there must be a next token
                tok_stream.next().unwrap().text(src)
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
                tok_stream.next().unwrap().text(src).to_string()
            }))
        }
        T![+] | T![-] | T![*] | T![/] | T![%] => {
            Sexpr::Atom(Atom::Symbol({
                tok_stream.next().unwrap().text(src).to_string()
            }))
        }
        kind => {
            panic!("Unknown start of atom: `{}`", kind);
        }
    }
}
