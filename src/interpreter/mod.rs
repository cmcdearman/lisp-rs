use crate::parser::sexpr::{env::Env, Atom, Sexpr};

pub mod default_env;
pub mod repl;

pub fn eval(obj: Sexpr, env: Box<Env>) -> Result<Sexpr, String> {
    match obj {
        lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        Sexpr::Atom(Atom::Sym(name)) => Ok(env
            .find(&name)
            .ok_or_else(|| format!("name `{}` not found", name))?
            .clone()),
        Sexpr::Cons(car, cdr) => {
            todo!()
        }
        Sexpr::Nil => todo!(),
    }
}
