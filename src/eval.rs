use crate::{
    ast::{Atom, Lit, Sexpr},
    env::Env,
};

pub fn eval(ast: &Vec<Sexpr>, pos: usize, env: Env) -> Result<Sexpr, String> {
    match &ast[pos] {
        lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        Sexpr::Atom(Atom::Sym(name)) => Ok(env.find(name.clone())?.clone()),
        Sexpr::Cons(d, n) => {
            todo!()
        }
        Sexpr::Fn(_) => Err("unexpected form".to_string()),
        Sexpr::Nil => todo!(),
    }
}
