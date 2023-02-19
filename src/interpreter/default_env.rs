use crate::parser::sexpr::{env::Env, Atom, Lit, Number, Sexpr};

use super::runtime_error::RuntimeError;

pub fn default_env() -> Env {
    let mut env = Env::new();
    env.define(
        String::from("+"),
        Sexpr::NativeFn(|_, args| Ok(sum_number_list(args)?)),
    );
    // env.define(
    //     String::from("-"),
    //     Sexpr::NativeFn(|_, args| Ok((sub_number_list(args)?))),
    // );
    // env.define(
    //     String::from("*"),
    //     Sexpr::NativeFn(|_, args| Ok((mul_number_list(args)?))),
    // );
    // env.define(
    //     String::from("/"),
    //     Sexpr::NativeFn(|_, args| Ok((quo_number_list(args)?))),
    // );
    // env.define(
    //     String::from("let"),
    //     Sexpr::NativeFn(|_, args| Ok((sum_number_list(args)?))),
    // );
    // env.define(
    //     String::from("mod"),
    //     Sexpr::NativeFn(|_, args| Ok((mod_number_list(args)?))),
    // );
    // env.define(
    //     String::from("fn"),
    //     Sexpr::NativeFn(|env, args| {
    //         if !(2..4).contains(&args.len()) {
    //             return Err("not enough arguments for function declaration".to_string());
    //         }
    //         let lambda_args = &args[0];
    //         let body = &args[1];
    //         let mut fn_args;
    //         if args.len() == 3 {
    //             fn_args = &args[2];
    //         }
    //         // Ok(Sexpr::Lambda(Lambda { env, args: lambda_args, body }))
    //         todo!()
    //     }),
    // );
    // env.define(
    //     String::from("type-of"),
    //     Sexpr::NativeFn(|env, args| todo!()),
    // );
    env
}

fn sum_number_list(args: Vec<Sexpr>) -> Result<Sexpr, RuntimeError> {
    Ok(Sexpr::Atom(Atom::Lit(Lit::Number(args.iter().fold(
        Number::Fixnum(0),
        |acc, s| -> Number {
            match s {
                Sexpr::Atom(Atom::Lit(Lit::Number(n))) => Ok((acc + *n)?),
                _ => Err(RuntimeError::IvalidFunctionArgumentsError),
            }
        },
    )))))
}

// fn gcd(a: i64, b: i64) -> i64 {
//     match b {
//        0 => a,
//        _ => gcd(b, a % b)
//     }
// }

// fn sub_number_list(args: Vec<Sexpr>) -> Result<Sexpr, String> {
//     let first = match args.get(0) {
//         Sexpr::Atom(n) => n,
//         _ => Err(String::from("error converting sub arguments to Sexprs"))?,
//     };

//     Ok(first.clone() - sum_number_list(args[1..].to_vec())?)
// }

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
