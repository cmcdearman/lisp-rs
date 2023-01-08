use crate::ast::{
    env::Env,
    object::{Atom, Object},
};

pub fn eval(obj: &Object, env: &mut Env) -> Result<Object, String> {
    match obj {
        lit @ Object::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        Object::Atom(Atom::Sym(name)) => Ok(env
            .find(&name)
            .ok_or_else(|| "name not found".to_string())?
            .clone()),
        Object::List(l) => {
            todo!()
        }
        _ => Err("unexpected form".to_string()),
    }
}
