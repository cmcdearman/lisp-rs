use std::fmt::Debug;

use super::{list::List, lambda::Lambda, number::Number, symbol::Symbol};

#[derive(Clone)]
pub enum Object {
    Sym(Symbol),
    Num(Number),
    Bool(bool),
    Str(String),
    List(List),
    Lambda(Lambda)
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // let str = match self {
        //     Object::Atom(a) => {
        //         match a {
        //             Atom::Sym(s) => s.clone(),
        //             Atom::(l) => {
        //                 match l {
        //                     Lit::Num(n) => n.to_string(),
        //                     Lit::Str(s) => s.to_string(),
        //                     Lit::Bool(b) => b.to_string(),
        //                 }
        //             }
        //         }
        //     }
        //     Object::List(list) => {
        //         let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
        //         format!("({})", xs.join(","))
        //     }
        // };

        // write!(f, "{}", str)
        todo!()
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Sym(_) => todo!(),
            Object::Num(_) => todo!(),
            Object::Bool(_) => todo!(),
            Object::Str(_) => todo!(),
            Object::List(_) => todo!(),
            Object::Lambda(_) => todo!(),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Object {}