use crate::{ast::{Sexpr, Atom, Lit}, env::Env};

pub fn eval(ast: &Vec<Sexpr>, pos: usize, env: Env) -> Result<Sexpr, String> {
    match &ast[pos] {
        lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        Sexpr::Atom(Atom::Sym(name)) => Ok(env.find(name.clone())?.clone()),
        Sexpr::Cons(d, n) => {
            todo!()
        }
        Sexpr::Fn(f) => todo!(),
        Sexpr::Nil => todo!(),
    }
}
