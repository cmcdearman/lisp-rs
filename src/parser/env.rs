use std::collections::HashMap;
use crate::parser::ast::Sexpr;

pub struct Env {
    data: HashMap<String, Sexpr>
}

// pub fn default_env() -> Env {
//     let mut data: HashMap<String, Sexpr> = HashMap::new();
//     data.insert(
//         "+".to_string(),
//
//     )
//     Env{data}
// }
