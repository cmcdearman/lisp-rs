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
        String::from("%"),
        Sexpr::NativeFn(|env, args| Ok(mod_number_list(env, args)?)),
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
        String::from(">="),
        Sexpr::NativeFn(|env, args| Ok(geq(env, args)?)),
    );
    env.define(
        String::from("<="),
        Sexpr::NativeFn(|env, args| Ok(leq(env, args)?)),
    );
    env.define(
        String::from("not"),
        Sexpr::NativeFn(|env, args| Ok(not(env, args)?)),
    );
    env.define(
        String::from("="),
        Sexpr::NativeFn(|env, args| Ok(eql(env, args)?)),
    );
    env.define(
        String::from("neq"),
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
        Sexpr::Lambda { env, args, body } => Ok(Sexpr::Atom(Atom::Sym("Lambda".to_string()))),
        Sexpr::NativeFn(f) => Ok(Sexpr::Atom(Atom::Sym(format!("NativeFn: {:?}", f)))),
        Sexpr::Env(_) => todo!(),
    }
}

fn sum_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(args.iter().try_fold(
        Number::Fixnum(0),
        |acc, s| -> Result<Number> {
            match s {
                Sexpr::Atom(Atom::Lit(Lit::Number(n))) => acc + n.clone(),
                _ => Err(RuntimeError::new(&format!("can't add non-number `{}`", s))),
            }
        },
    )?))))
}

fn sub_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = match args.get(0) {
        Some(Sexpr::Atom(Atom::Lit(Lit::Number(n)))) => n,
        _ => Err(RuntimeError::new("can't subtract from non-number"))?,
    };

    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = sum_number_list(args[1..].to_vec())? {
        return (first.clone() - n).map(|num| Sexpr::Atom(Atom::Lit(Lit::Number(num))));
    }
    Err(RuntimeError::new("can't subtract non-number"))
}

fn mul_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(args.iter().try_fold(
        Number::Fixnum(1),
        |acc, s| -> Result<Number> {
            match s {
                Sexpr::Atom(Atom::Lit(Lit::Number(n))) => acc * n.clone(),
                _ => Err(RuntimeError::new(&format!(
                    "can't multiply non-number `{}`",
                    s
                ))),
            }
        },
    )?))))
}

fn quo_number_list(args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = match &args.get(0) {
        Some(Sexpr::Atom(Atom::Lit(Lit::Number(n)))) => n,
        _ => Err(RuntimeError::new("can't divide non-number"))?,
    };

    let denom = match mul_number_list(args[1..].to_vec())? {
        Sexpr::Atom(Atom::Lit(Lit::Number(n))) => n,
        _ => Err(RuntimeError::new("can't divide by non-number"))?,
    };

    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(
        (first.clone() / denom)?,
    ))))
}

fn mod_number_list(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    // println!("mod args: {:?}", args);
    let num = match eval(
        env.clone(),
        args.get(0)
            .ok_or(RuntimeError::new("mod takes two arguments, got 0"))?,
    )? {
        Sexpr::Atom(Atom::Lit(Lit::Number(n))) => n,
        sexpr => Err(RuntimeError::new(&format!(
            "mod first argument must be a Number, got `{}`",
            sexpr
        )))?,
    };
    let div = match eval(
        env.clone(),
        args.get(1)
            .ok_or(RuntimeError::new("mod takes two arguments, got 1"))?,
    )? {
        Sexpr::Atom(Atom::Lit(Lit::Number(n))) => n,
        _ => Err(RuntimeError::new("mod second argument must be a Number"))?,
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
        args.get(0)
            .ok_or(RuntimeError::new("gtr takes two arguments. got 0"))?,
    )? {
        first = n;
    } else {
        return Err(RuntimeError::new("gtr first argument must be a number"));
    }
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(1)
            .ok_or(RuntimeError::new("gtr takes two arguments, got 1"))?,
    )? {
        second = n;
    } else {
        return Err(RuntimeError::new("gtr second argument must be number"));
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first > second))))
}

fn lss(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first;
    let second;
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(0)
            .ok_or(RuntimeError::new("lss takes 2 arguments, got 0"))?,
    )? {
        first = n;
    } else {
        return Err(RuntimeError::new("lss first argument must be a number"));
    }
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(1)
            .ok_or(RuntimeError::new("lss takes 2 arguments, got 1"))?,
    )? {
        second = n;
    } else {
        return Err(RuntimeError::new("lss second argument must be a number"));
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first < second))))
}

fn geq(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first;
    let second;
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(0)
            .ok_or(RuntimeError::new(">= takes 2 arguments, got 0"))?,
    )? {
        first = n;
    } else {
        return Err(RuntimeError::new(">= first argument must be a Number"));
    }
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(1)
            .ok_or(RuntimeError::new(">= takes 2 arguments, got 1"))?,
    )? {
        second = n;
    } else {
        return Err(RuntimeError::new(">= second argument must be a Number"));
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first >= second))))
}

fn leq(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first;
    let second;
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(0)
            .ok_or(RuntimeError::new("<= takes 2 arguments, got 0"))?,
    )? {
        first = n;
    } else {
        return Err(RuntimeError::new("<= first argument must be a Number"));
    }
    if let Sexpr::Atom(Atom::Lit(Lit::Number(n))) = eval(
        env.clone(),
        args.get(1)
            .ok_or(RuntimeError::new("<= takes 2 arguments, got 1"))?,
    )? {
        second = n;
    } else {
        return Err(RuntimeError::new("<= second argument must be a Number"));
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first <= second))))
}

fn not(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first;
    if let Sexpr::Atom(Atom::Lit(Lit::Bool(b))) = eval(
        env,
        args.get(0)
            .ok_or(RuntimeError::new("`not` takes 1 argument, got 0"))?,
    )? {
        first = b;
    } else {
        return Err(RuntimeError::new("`not` first argument must be a Boolean"));
    }
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(!first))))
}

fn eql(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = eval(
        env.clone(),
        args.get(0)
            .ok_or(RuntimeError::new("`eq` takes 2 arguments, got 0"))?,
    )?;
    let second = eval(
        env.clone(),
        args.get(1)
            .ok_or(RuntimeError::new("`eq` takes 2 arguments, got 1"))?,
    )?;
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first == second))))
}

fn neq(env: Rc<RefCell<Env>>, args: Vec<Sexpr>) -> Result<Sexpr> {
    let first = eval(
        env.clone(),
        args.get(0)
            .ok_or(RuntimeError::new("`neq` takes 2 arguments, got 0"))?,
    )?;
    let second = eval(
        env.clone(),
        args.get(1)
            .ok_or(RuntimeError::new("`eq` takes 2 arguments, got 1"))?,
    )?;
    Ok(Sexpr::Atom(Atom::Lit(Lit::Bool(first != second))))
}

// fn gcd(a: i64, b: i64) -> i64 {
//     match b {
//        0 => a,
//        _ => gcd(b, a % b)
//     }
// }
