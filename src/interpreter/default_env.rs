use crate::sexpr::{env::Env, symbol::Symbol, Sexpr, Atom, Lit, number::Number};

// pub fn default_env() -> Env {
//     let mut env = Env::new();
//     env.define(
//         Symbol::from("+"),
//         Object::NativeFn(|_, args| Ok(Object::Atom(Atom::Lit(Lit::Num(sum_num_list(args)?))))),
//     );
//     env.define(
//         Symbol::from("-"),
//         Object::NativeFn(|_, args| Ok(Object::Atom(Atom::Lit(Lit::Num(sub_num_list(args)?))))),
//     );
//     env.define(
//         Symbol::from("*"),
//         Object::NativeFn(|_, args| Ok(Object::Atom(Atom::Lit(Lit::Num(mul_num_list(args)?))))),
//     );
//     env.define(
//         Symbol::from("/"),
//         Object::NativeFn(|_, args| Ok(Object::Atom(Atom::Lit(Lit::Num(quo_num_list(args)?))))),
//     );
//     env.define(
//         Symbol::from("let"),
//         Object::NativeFn(|_, args| Ok(Object::Atom(Atom::Lit(Lit::Num(sum_num_list(args)?))))),
//     );
//     env.define(
//         Symbol::from("mod"),
//         Object::NativeFn(|_, args| Ok(Object::Atom(Atom::Lit(Lit::Num(mod_num_list(args)?))))),
//     );
//     env.define(
//         Symbol::from("fn"),
//         Object::NativeFn(|env, args| {
//             if !(2..4).contains(&args.len()) {
//                 return Err("not enough arguments for function declaration".to_string());
//             }
//             let lambda_args = &args[0];
//             let body = &args[1];
//             let mut fn_args;
//             if args.len() == 3 {
//                 fn_args = &args[2];
//             }
//             // Ok(Object::Lambda(Lambda { env, args: lambda_args, body }))
//             todo!()
//         }),
//     );
//     env.define(
//         Symbol::from("type-of"),
//         Object::NativeFn(|env, args| todo!()),
//     );
//     env
// }

// fn sum_num_list(args: Vec<Object>) -> Result<Number, String> {
//     args.iter()
//         .map(|s| -> Result<Number, String> {
//             match s {
//                 Object::Atom(Atom::Lit(Lit::Num(n))) => Ok(n.clone()),
//                 _ => Err(String::from("error converting arguments to numbers")),
//             }
//         })
//         .sum()
// }

// fn sub_num_list(args: Vec<Object>) -> Result<Number, String> {
//     let first = match args.get(0) {
//         Some(Object::Atom(Atom::Lit(Lit::Num(n)))) => n,
//         _ => Err(String::from("error converting sub arguments to numbers"))?,
//     };

//     Ok(first.clone() - sum_num_list(args[1..].to_vec())?)
// }

// fn mul_num_list(args: Vec<Object>) -> Result<Number, String> {
//     args.iter()
//         .map(|s| -> Result<Number, String> {
//             match s {
//                 Object::Atom(Atom::Lit(Lit::Num(n))) => Ok(n.clone()),
//                 _ => Err(String::from("error converting mul arguments to numbers"))?,
//             }
//         })
//         .product()
// }

// fn quo_num_list(args: Vec<Object>) -> Result<Number, String> {
//     let first = match &args[0] {
//         Object::Atom(Atom::Lit(Lit::Num(n))) => n,
//         _ => Err(String::from("error converting quo arguments to numbers"))?,
//     };

//     Ok(first.clone() / mul_num_list(args[1..].to_vec())?)
// }

// fn mod_num_list(args: Vec<Object>) -> Result<Number, String> {
//     if args.len() != 2 {
//         return Err("need two args for mod".to_string());
//     }
//     let num = match &args[0] {
//         Object::Atom(Atom::Lit(Lit::Num(n))) => n,
//         _ => Err(String::from("error converting quo arguments to numbers"))?,
//     };
//     let div = match &args[1] {
//         Object::Atom(Atom::Lit(Lit::Num(n))) => n,
//         _ => Err(String::from("error converting quo arguments to numbers"))?,
//     };

//     Ok(num.clone() % div.clone())
// }
