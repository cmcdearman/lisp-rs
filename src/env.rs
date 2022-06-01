use std::collections::HashMap;
use crate::ast::Sexpr;

pub struct Env {
   data: HashMap<String, Sexpr>
}
