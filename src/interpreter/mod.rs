use std::{cell::RefCell, rc::Rc};

use crate::parser::{
    sexpr::{env::Env, Atom, Sexpr, NIL},
    Parser,
};

use self::runtime_error::{Result, RuntimeError};

pub mod default_env;
pub mod repl;
pub mod runtime_error;

// pub struct Interpreter<'src> {
//     src: &'src str,
//     parser: Parser<'src>,
// }

// impl<'src> Interpreter<'src> {
//     pub fn new(src: &'src str, lazy: bool) -> Self {
//         Self {
//             src,
//             parser: Parser::new(src, lazy),
//         }
//     }

//     pub fn eval(&mut self, env: Box<Env>) -> Result<Sexpr> {
//         match &self
//             .parser
//             .parse()
//             .map_err(|e| RuntimeError::ParserError(e))?
//         {
//             lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
//             Sexpr::Atom(Atom::Sym(name)) => Ok(env
//                 .find(&name)
//                 .ok_or(RuntimeError::UnknownIdentError)?
//                 .clone()),
//             Sexpr::List(head) => {
//                 let mut list_iter = head.clone().into_iter();
//                 match self.eval(env)? {
//                     Sexpr::NativeFn(_) => {
//                         let args: Result<Vec<Sexpr>> =
//                             list_iter.map(|x| eval(&x, env.clone())).collect();
//                         f(env, args?)
//                     }
//                     _ => todo!(),
//                 }
//                 todo!()
//             }
//             _ => Err(RuntimeError::IvalidFunctionArgumentsError),
//         }
//     }
// }

pub fn eval(sexpr: &Sexpr, env: Rc<RefCell<Env>>) -> Result<Sexpr> {
    match sexpr {
        lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        sym @ Sexpr::Atom(Atom::Sym(name)) => {
            if let Some(v) = env.borrow().find(&name) {
                return Ok(v.clone());
            }
            Ok(sym.clone())
        }
        Sexpr::List(l) => {
            let mut list_iter = l.clone().into_iter();
            match eval(
                &list_iter.next().ok_or(RuntimeError::EarlyListEndError)?,
                env.clone(),
            )? {
                Sexpr::NativeFn(f) => {
                    let args: Result<Vec<Sexpr>> =
                        list_iter.map(|x| eval(&x, env.clone())).collect();
                    f(env, args?)
                }
                _ => Err(RuntimeError::FirstElemError),
            }
        }
        _ => Err(RuntimeError::IvalidFunctionArgumentsError),
    }
}
