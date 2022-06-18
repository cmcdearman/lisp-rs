use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;

// Object is the AST that will actually be used for eval. Here we use reference counters
// in our Cons to maintain interior mutability.
#[derive(Debug, Clone)]
pub enum Object {
    Symbol(String),
    Number(f64),
    String(String),
    Cons { car: Rc<Object>, cdr: Rc<Object> },
    Func(fn(Object) -> Object),
    Nil
}

// pub struct ObjectIter {
//     cur: Rc<Object>
// }
//
// impl Iterator for ObjectIter {
//     type Item = Object;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match &self.cur.as_ref() {
//             Object::Cons { car: _car, cdr } => {
//                 let cur = Some(*self.cur.as_ref());
//                 &self.cur = cdr;
//                 cur
//             }
//             _ => { None }
//         }
//     }
// }