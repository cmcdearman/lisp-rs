// use std::collections::HashMap;
// use std::fmt::Error;
// use std::os::unix::raw::off_t;
// use crate::ast::{Atom, Literal, Sexpr};
// use crate::object::Object;
//
// pub struct Env {
//    data: HashMap<String, Object>
// }
//
// impl Env {
//    pub fn new() -> Self { Self { data: HashMap::new() }}
//
//    pub fn push(&mut self, name: String, value: Object) {
//       self.data.insert(name, value);
//    }
//
//    pub fn pop(&mut self, name: String) {
//       self.data.remove(&*name);
//    }
// }
//
// pub fn default_env() -> Env {
//    let mut data: HashMap<String, Object> = HashMap::new();
//    data.insert(String::from("+"), Object::Func(|args: Object| -> Object {
//       Object::Number(sum_num_list(args)?)
//    }));
//    Env{data}
// }
//
// fn sum_num_list(args: Object) -> Result<f64, Error> {
//
//    todo!()
// }
//
// fn num_float(exp: Sexpr) -> Result<f64, Error> {
//    match exp {
//       Literal::Number(n) => Ok(*n),
//       _ => Err(ParserError::EnvErr(String::from("expected number")))
//    }
// }