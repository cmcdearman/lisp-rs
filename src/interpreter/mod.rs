use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, env::Env, Atom};

pub mod default_env; 
pub mod repl;

pub fn eval(obj: &Object, env: Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        lit @ Object::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        Object::Atom(Atom::Sym(name)) => Ok(env
            .borrow_mut()
            .find(&name)
            .ok_or_else(|| "name not found".to_string())?
            .clone()),
        Object::List(l) => {
            let mut list_iter = l.into_iter();
            match eval(
                &list_iter
                    .next()
                    .ok_or("expected non-empty list".to_string())?,
                env.clone(),
            )? {
                Object::NativeFn(f) => {
                    let args: Result<Vec<Object>, String> =
                        list_iter.map(|x| eval(&x, env.clone())).collect();
                    f(env, args?)
                }
                _ => Err("first element of list must be a function".to_string()),
            }
        }
        _ => Err("unexpected form".to_string()),
    }
}
