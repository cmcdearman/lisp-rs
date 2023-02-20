use std::{cell::RefCell, rc::Rc};

use crate::parser::sexpr::{env::Env, Atom, Lit, Number, Sexpr};

use super::runtime_error::{Result, RuntimeError};

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
    // env.define(
    //     String::from("*"),
    //     Sexpr::NativeFn(|_, args| Ok((mul_number_list(args)?))),
    // );
    // env.define(
    //     String::from("/"),
    //     Sexpr::NativeFn(|_, args| Ok((quo_number_list(args)?))),
    // );
    // env.define(
    //     String::from("def"),
    //     Sexpr::NativeFn(|env, args| {
    //         if let Sexpr::Atom(Atom::Sym(s)) = &args[0] {
    //             env.borrow_mut().define(s.to_string(), args[1].clone());
    //             return Ok(args[1].clone());
    //         }
    //         Err(RuntimeError::IvalidFunctionArgumentsError)
    //     }),
    // );
    // env.define(
    //     String::from("mod"),
    //     Sexpr::NativeFn(|_, args| Ok((mod_number_list(args)?))),
    // );

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

// fn mul_number_list(args: Vec<Sexpr>) -> Result<Sexpr, String> {
//     args.iter()
//         .map(|s| -> Result<Sexpr, String> {
//             match s {
//                 (n) => Ok(n.clone()),
//                 _ => Err(String::from("error converting mul arguments to Sexprs"))?,
//             }
//         })
//         .product()
// }

// fn quo_number_list(args: Vec<Sexpr>) -> Result<Sexpr, String> {
//     let first = match &args[0] {
//         (n) => n,
//         _ => Err(String::from("error converting quo arguments to Sexprs"))?,
//     };

//     Ok(first.clone() / mul_Number_list(args[1..].to_vec())?)
// }

// fn mod_number_list(args: Vec<Sexpr>) -> Result<Sexpr, String> {
//     if args.len() != 2 {
//         return Err("need two args for mod".to_string());
//     }
//     let Number = match &args[0] {
//         (n) => n,
//         _ => Err(String::from("error converting quo arguments to Sexprs"))?,
//     };
//     let div = match &args[1] {
//         (n) => n,
//         _ => Err(String::from("error converting quo arguments to Sexprs"))?,
//     };

//     Ok(Number.clone() % div.clone())
// }

// fn gcd(a: i64, b: i64) -> i64 {
//     match b {
//        0 => a,
//        _ => gcd(b, a % b)
//     }
// }
