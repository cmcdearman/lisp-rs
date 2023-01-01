use crate::{
    ast::{Atom, Sexpr},
    env::Env,
};

pub fn eval(sexpr: &Sexpr, env: &mut Env) -> Result<Sexpr, String> {
    match sexpr {
        lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        Sexpr::Atom(Atom::Sym(name)) => Ok(env.find(name.clone())?.clone()),
        Sexpr::List(l) => {
            let first_form = l.first().ok_or("expected a non-empty list".to_string())?;
            let arg_forms = &l[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                Sexpr::Fn(f) => {
                    let args: Result<Vec<Sexpr>, String> = arg_forms
                    .iter()
                    .map(|x| eval(x, env))
                    .collect();
                    f(&args?)
                },
                _ => Err("first form must be a function".to_string())
            }
        }
        _ => Err("unexpected form".to_string()),
    }
}
