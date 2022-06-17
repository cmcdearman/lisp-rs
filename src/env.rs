// use std::collections::HashMap;
// use std::fmt::Error;
// use crate::ast::{Atom, Literal, Sexpr};
//
// pub enum Value {
//    Func(fn(&[Sexpr]) -> Sexpr),
//    Exp(Sexpr)
// }
//
// pub struct Env {
//    data: HashMap<String, Value>
// }
//
// pub fn default_env() -> Env {
//    let mut data: HashMap<String, Value> = HashMap::new();
//    data.insert(String::from("+"), Value::Func(|args: &[Sexpr]| -> Sexpr {
//       Sexpr::Atom(Atom::Literal(Literal::Number(sum_num_list(args)?)))
//    }));
//    Env{data}
// }
//
// fn sum_num_list(list: &[Sexpr]) -> Result<f64, Error> {
//    list.iter().map(|n| num_float(n)).collect()
// }
//
// fn num_float(exp: &Sexpr) -> Result<f64, Error> {
//    match exp {
//       Literal::Number(n) => Ok(*n),
//       _ => Err(ParserError::EnvErr(String::from("")))
//    }
// }