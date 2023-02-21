use std::{cell::RefCell, rc::Rc};

use crate::parser::sexpr::{env::Env, Atom, Lit, Number, Sexpr};

use super::{
    eval,
    runtime_error::{Result, RuntimeError},
};

pub fn default_env() -> Rc<RefCell<Env>> {
    let mut env = Env::new();
    env.define(
        String::from("+"),
        Sexpr::NativeFn(|_, args| Ok(sum_number_list(args)?)),
    );
    env.define(
        String::from("-"),
        Sexpr::NativeFn(|_, args| Ok(sub_number_list(args)?)),
    );
    env.define(
        String::from("*"),
        Sexpr::NativeFn(|_, args| Ok(mul_number_list(args)?)),
    );
    env.define(
        String::from("/"),
        Sexpr::NativeFn(|_, args| Ok(quo_number_list(args)?)),
    );
    env.define(
        String::from("mod"),
        Sexpr::NativeFn(|_, args| Ok(mod_number_list(args)?)),
    );
    env.define(
        String::from(">"),
        Sexpr::NativeFn(|env, args| Ok(gtr(env, args)?)),
    );
    env.define(
        String::from("<"),
        Sexpr::NativeFn(|env, args| Ok(lss(env, args)?)),
    );
    env.define(
        String::from("!"),
        Sexpr::NativeFn(|env, args| Ok(not(env, args)?)),
    );
    env.define(
        String::from("=="),
        Sexpr::NativeFn(|env, args| Ok(eql(env, args)?)),
    );
    env.define(
        String::from("!="),
        Sexpr::NativeFn(|env, args| Ok(neq(env, args)?)),
    );
    env.define(String::from("type-of"), Sexpr::NativeFn(type_of));

    Rc::new(RefCell::new(env))
}

fn type_of(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    match &args[0] {
        Sexpr::Atom(a) => match a {
            Atom::Sym(s) => {
                if let Some(var) = env.borrow().find(s) {
                    return type_of(env.clone(), vec![var]);
                }
                Ok(Sexpr::Atom(Atom::Sym("Symbol".to_string())))
            }
            Atom::Lit(l) => match l {
                Lit::Number(n) => match n {
                    Number::Fixnum(_) => Ok(Sexpr::Atom(Atom::Sym("Fixnum".to_string()))),
                    Number::Float(_) => Ok(Sexpr::Atom(Atom::Sym("Float".to_string()))),
                    Number::Rational(_) => Ok(Sexpr::Atom(Atom::Sym("Rational".to_string()))),
                    Number::Bignum(_) => Ok(Sexpr::Atom(Atom::Sym("Bignum".to_string()))),
                },
                Lit::Bool(_) => Ok(Sexpr::Atom(Atom::Sym("Boolean".to_string()))),
                Lit::Str(_) => Ok(Sexpr::Atom(Atom::Sym("String".to_string()))),
            },
        },
        Sexpr::List(_) => Ok(Sexpr::Atom(Atom::Sym("List".to_string()))),
        Sexpr::Lambda { env, args, body } => todo!(),
        Sexpr::NativeFn(f) => Ok(Sexpr::Atom(Atom::Sym(format!("NativeFn: {:?}", f)))),
    }
}

fn sum_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(args.iter().try_fold(
        Number::Fixnum(0),
        |acc, s| -> Result<Number> {
            match s {
                Sexpr::Atom(Atom::Lit(Lit::Number(n))) => acc + n.clone(),
                _ => Err(RuntimeError::IvalidFunctionArgumentsError),
            }
        },
    )?))))
}

fn sub_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = match args.get(0) {
        Some(Sexpr::Atom(Atom::Lit(Lit::Number(n)))) => n,
        _ => Err(RuntimeError::FirstElemError)?,
    };

    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = sum_number_list(args[1..].to_vec())? {
        return (first.clone() - n).map(|num| Sexpr::Atom(Atom::Lit(Lit::Number(num))));
    }
    Err(RuntimeError::IvalidFunctionArgumentsError)
}

fn mul_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(args.iter().try_fold(
        Number::Fixnum(1),
        |acc, s| -> Result<Number> {
            match s {
                Sexpr::Atom(Atom::Lit(Lit::Number(n))) => acc * n.clone(),
                _ => Err(RuntimeError::IvalidFunctionArgumentsError),
            }
        },
    )?))))
}

fn quo_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = match &args.get(0) {
        Some(Sexpr::Atom(Atom::Lit(Lit::Number(n)))) => n,
        _ => Err(RuntimeError::FirstElemError)?,
    };

    let denom = match mul_number_list(args[1..].to_vec())? {
        Sexpr::Atom(Atom::Lit(Lit::Number(n))) => n,
        _ => Err(RuntimeError::FirstElemError)?,
    };

    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(
        (first.clone() / denom)?,
    ))))
}

fn mod_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    if args.len() != 2 {
        return Err(RuntimeError::IvalidFunctionArgumentsError);
    }
    let num = match args.get(0) {
        Some(Sexpr::Atom(Atom::Lit(Lit::Number(n)))) => n,
        _ => Err(RuntimeError::FirstElemError)?,
    };
    let div = match args.get(1) {
        Some(Sexpr::Atom(Atom::Lit(Lit::Number(n)))) => n,
        _ => Err(RuntimeError::FirstElemError)?,
    };

    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(
        (num.clone() % div.clone())?,
    ))))
}

fn gtr(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first;
    let second;
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(0).ok_or(RuntimeError::EarlyListEndError)?,
    )? {
        first = n;
    } else {
        return Err(RuntimeError::IvalidFunctionArgumentsError);
    }
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(1).ok_or(RuntimeError::EarlyListEndError)?,
    )? {
        second = n;
    } else {
        return Err(RuntimeError::IvalidFunctionArgumentsError);
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first > second))))
}

fn lss(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first;
    let second;
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(0).ok_or(RuntimeError::EarlyListEndError)?,
    )? {
        first = n;
    } else {
        return Err(RuntimeError::IvalidFunctionArgumentsError);
    }
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(1).ok_or(RuntimeError::EarlyListEndError)?,
    )? {
        second = n;
    } else {
        return Err(RuntimeError::IvalidFunctionArgumentsError);
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first < second))))
}

fn not(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first;
    if let Sexpr::Atom(Atom::Lit(Lit::Bool(b))) =
        eval(env, args.get(0).ok_or(RuntimeError::EarlyListEndError)?)?
    {
        first = b;
    } else {
        return Err(RuntimeError::IvalidFunctionArgumentsError);
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(!first))))
}

fn eql(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = eval(
        env.clone(),
        args.get(0).ok_or(RuntimeError::EarlyListEndError)?,
    )?;
    let second = eval(
        env.clone(),
        args.get(1).ok_or(RuntimeError::EarlyListEndError)?,
    )?;
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first == second))))
}

fn neq(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = eval(
        env.clone(),
        args.get(0).ok_or(RuntimeError::EarlyListEndError)?,
    )?;
    let second = eval(
        env.clone(),
        args.get(1).ok_or(RuntimeError::EarlyListEndError)?,
    )?;
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first != second))))
}

// fn gcd(a: i64, b: i64) -> i64 {
//     match b {
//        0 => a,
//        _ => gcd(b, a % b)
//     }
// }
