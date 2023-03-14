use crate::T;

use super::{sexpr::{Sexpr, Number}, token::Token};
use chumsky::{prelude::Simple, recursive::recursive, Parser, select, primitive::just};

pub fn parser() -> impl Parser<Token, Sexpr, Error = Simple<Token>> {
    recursive(|sexpr| {
        let number = select! {
            T![int] => Number::Fixnum(),
            T![float] => Sexpr::Number(Number::Float(0.0)),
        };

        let lit = select! {
            T![int] => Sexpr::Number(Number::Int(0)),
            T![float] => Sexpr::Number(Number::Float(0.0)),
            T![string] => Sexpr::String("".to_string()),
            T![char] => Sexpr::Char('a'),
            T![bool] => Sexpr::Bool(true),
        };
        
        let atom = select! {
            T![int] => 
        }

        atom
    })
}
