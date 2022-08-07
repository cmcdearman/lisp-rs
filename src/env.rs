use std::collections::HashMap;
use crate::ast::{Atom, Literal, Sexpr};
use crate::parser::ParseError;


#[derive(Debug, Clone)]
pub struct Env {
   data: HashMap<String, Sexpr>
}

impl Env {
   pub fn new() -> Self { Self { data: HashMap::new() }}

   pub fn push(&mut self, name: String, value: Sexpr) {
      self.data.insert(name, value);
   }

   pub fn pop(&mut self, name: String) {
      self.data.remove(&*name);
   }
}

pub fn default_env() -> Env {
   let mut data: HashMap<String, Sexpr> = HashMap::new();
   data.insert(String::from("+"), Sexpr::Func(|args: Sexpr| -> Result<Sexpr, ParseError> {
      Ok(Sexpr::Atom(Atom::Literal(Literal::Number(sum_num_list(args)?))))
   }));
   Env{data}
}

fn sum_num_list(args: Sexpr) -> Result<f64, ParseError> {
   todo!()
}

// fn num_float(exp: Sexpr) -> Result<f64, Error> {
//    match exp {
//       Literal::Number(n) => Ok(*n),
//       _ => Err(ParseError::)
//    }
// }
