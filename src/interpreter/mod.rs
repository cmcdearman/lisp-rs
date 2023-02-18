use std::{cell::RefCell, rc::Rc};

use crate::parser::sexpr::{env::Env, Atom, Sexpr};

pub mod default_env;
pub mod repl;

pub fn eval(obj: &Sexpr, env: Rc<RefCell<Env>>) -> Result<Sexpr, String> {
    match obj {
        lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        Sexpr::Atom(Atom::Sym(name)) => Ok(env
            .borrow_mut()
            .find(*name)
            .ok_or_else(|| "name not found".to_string())?
            .clone()),
        Sexpr::Cons(car, cdr) => {
            match eval(
                car,
                    .next()
                    .ok_or("expected non-empty list".to_string())?,
                env.clone(),
            )? {
                atom @ Sexpr::Atom(_) => Ok(atom.clone()),
                Sexpr::Cons(ca, cd) => todo!(),
            }
        }
    }
}