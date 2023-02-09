pub mod cons;
pub mod env;
pub mod lambda;
pub mod list;
pub mod number;
pub mod symbol;

use std::fmt::{Debug, Display};

use self::number::{float::Float, integer::Integer};

#[derive(Clone)]
pub enum Object {
    Int(Integer),
    Float(Float),
    Imag(String, String),
    String(String),
    Char(char),

    Neg(Box<Object>),
    Add(Box<Object>, Box<Object>),
    Sub(Box<Object>, Box<Object>),
    Mul(Box<Object>, Box<Object>),
    Div(Box<Object>, Box<Object>),

    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Object>,
    },
    Call(String, Vec<Object>),
    Let {
        name: String,
        rhs: Box<Object>,
        lhs: Box<Object>,
    },
    Block(Vec<Object>),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Int(_) => todo!(),
            Object::Float(_) => todo!(),
            Object::Imag(_, _) => todo!(),
            Object::String(_) => todo!(),
            Object::Char(_) => todo!(),
            Object::Neg(_) => todo!(),
            Object::Add(_, _) => todo!(),
            Object::Sub(_, _) => todo!(),
            Object::Mul(_, _) => todo!(),
            Object::Div(_, _) => todo!(),
            Object::Fn { name, args, body } => todo!(),
            Object::Call(_, _) => todo!(),
            Object::Let { name, rhs, lhs } => todo!(),
            Object::Block(_) => todo!(),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(_) => todo!(),
            Object::Float(_) => todo!(),
            Object::Imag(_, _) => todo!(),
            Object::String(_) => todo!(),
            Object::Char(_) => todo!(),
            Object::Neg(_) => todo!(),
            Object::Add(_, _) => todo!(),
            Object::Sub(_, _) => todo!(),
            Object::Mul(_, _) => todo!(),
            Object::Div(_, _) => todo!(),
            Object::Fn { name, args, body } => todo!(),
            Object::Call(_, _) => todo!(),
            Object::Let { name, rhs, lhs } => todo!(),
            Object::Block(_) => todo!(),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Object {}
