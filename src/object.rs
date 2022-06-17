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

