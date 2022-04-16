use crate::lexer::Lexer;
use crate::ast::{ Sexpr, Atom, Literal};
use crate::token::Token;

pub fn parse(l: &mut Lexer) -> Sexpr {
    read_sexpr(l)
}

fn read_sexpr(l: &mut Lexer) -> Sexpr {
    let mut sexprs: Vec<Sexpr> = vec![];

    while let (Some(t), Some(_s)) = l.peek() {
        match t {
            Token::LParen => { sexprs.push(read_sexpr(l)); }
            Token::RParen => { }
            _ => { sexprs.push(read_atom(l)); }
        }
    }

    Sexpr::List(sexprs)
}

fn read_atom(l: &mut Lexer) -> Sexpr {
    let tok_span = l.next();
    let t = tok_span.0.unwrap();
    return match t {
        Token::Let => { Sexpr::Atom(Atom::Symbol(String::from("let"))) }
        Token::Ident(s) => { Sexpr::Atom(Atom::Symbol(s)) }
        Token::Number(n) => { Sexpr::Atom(Atom::Literal(Literal::Number(n))) }
        _ => { Sexpr::Atom(Atom::Literal(Literal::Number(0)))}
    };

}
