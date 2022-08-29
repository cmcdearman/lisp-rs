use crate::ast::{Atom, Lit, Sexpr};
use crate::token::{Token, TokenKind, TokenStream};

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
        TokenKind::LParen => {
            // Consume Rparen at start of list
            stream.next();
            

        }
        lit @ TokenKind::Num | lit @ TokenKind::String => {
            let lit_text = stream.next().unwrap().lit;
            let lit = match lit {
                TokenKind::Num => Lit::Num(
                    lit_text
                        .parse()
                        .expect(&format!("invalid floating point literal: `{}`", lit_text)),
                ),
                TokenKind::String => Lit::Str(lit_text[1..(lit_text.len() - 1)].to_string()),
                _ => unreachable!(),
            };
            ast.push(Sexpr::Atom(Atom::Lit(lit)));
        }
        TokenKind::Ident
        | TokenKind::Add
        | TokenKind::Sub
        | TokenKind::Mul
        | TokenKind::Quo
        | TokenKind::Mod
        | TokenKind::Let
        | TokenKind::Lambda => ast.push(Sexpr::Atom(Atom::Sym(
            stream.next().unwrap().lit.to_string(),
        ))),
        kind => {
            panic!("Unknown start of atom: `{}`", kind);
        }
    }
    ast
}

// let mut new_tail = elements.len();
// elements.push(SExpr::Cons(0, 0));
// match &mut elements[tail] {
//     Atom(_) => unreachable!(), // Sadly, this is neccessary
//     Cons(_, tail) => *tail = new_tail,
// }
// tail = new_tail;
