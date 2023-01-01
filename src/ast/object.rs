use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::{env::Env, lambda::Lambda, list::List, number::Number, symbol::Symbol};

#[derive(Clone)]
pub enum Object {
    Atom(Atom),
    List(List),
    Lambda(Lambda),
    NativeFn(NativeFn),
}

#[derive(Clone)]
pub enum Atom {
    Sym(Symbol),
    Lit(Lit),
}

#[derive(Clone)]
pub enum Lit {
    Num(Number),
    Bool(bool),
    Str(String),
}

pub type NativeFn = fn(env: Rc<RefCell<Env>>, args: Vec<Object>) -> Result<Object, String>;

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
            Object::Atom(_) => todo!(),
            Object::List(_) => todo!(),
            Object::Lambda(_) => todo!(),
            Object::NativeFn(_) => todo!(),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Object {}
