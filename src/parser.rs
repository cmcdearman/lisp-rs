use std::iter::Peekable;

use crate::token::{Token, TokenStream, TokenKind};
use crate::ast::Sexpr;

// [
// /* 0 */ Node::Nil,
// /* 1 */ Node::Num(3.0),
// /* 2 */ Node::Cons(0, 1),
// /* 3 */ Node::Num(2.0),
// /* 4 */ Node::Cons(2, 3),
// /* 5 */ Node::Num(1.0),
// /* 6 */ Node::Cons(4, 5),
// ]
pub fn parse(tokens: Vec<Token>) -> Vec<Sexpr> {
    let mut stream = TokenStream::new(tokens).peekable();
    let mut ast = Vec::new();
    match stream.peek().unwrap().kind {
       TokenKind::LParen => { stream.next(); parse_list(stream) } 
       _ => parse_atom(stream)
    }
}

// let mut new_tail = elements.len();
// elements.push(SExpr::Cons(0, 0));
// match &mut elements[tail] {
//     Atom(_) => unreachable!(), // Sadly, this is neccessary
//     Cons(_, tail) => *tail = new_tail,
// }
// tail = new_tail;
fn parse_list(stream: Peekable<TokenStream>) -> Sexpr {
    todo!()
}

fn parse_atom(stream: Peekable<TokenStream>) -> Sexpr {
    todo!()
}
