use std::{cell::RefCell, rc::Rc};

use crate::parser::{
    sexpr::{env::Env, Atom, Cons, ConsIter, Sexpr, NIL},
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

pub fn eval(env: Rc<RefCell<Env>>, sexpr: &Sexpr) -> Result<Sexpr> {
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
            let first = list_iter.next().ok_or(RuntimeError::EarlyListEndError)?;

            if first.is_special_form() {
                return eval_special_form(
                    env,
                    first.get_special_form().expect("expected special form"),
                    &mut list_iter,
                );
            }

            match eval(env.clone(), &first)? {
                Sexpr::NativeFn(f) => {
                    let args: Result<Vec<Sexpr>> =
                        list_iter.map(|x| eval(env.clone(), &x)).collect();
                    f(env, args?)
                }
                _ => Err(RuntimeError::FirstElemError),
            }
        }
        lambda @ Sexpr::Lambda {
            env: _,
            args: _,
            body: _,
        } => Ok(lambda.clone()),
        _ => Err(RuntimeError::IvalidFunctionArgumentsError),
    }
}

fn eval_special_form(
    env: Rc<RefCell<Env>>,
    special_form: String,
    list_iter: &mut ConsIter,
) -> Result<Sexpr> {
    match special_form.as_str() {
        "def" => {
            if let Some(Sexpr::Atom(Atom::Sym(s))) = list_iter.next() {
                let val = eval(
                    env.clone(),
                    &list_iter
                        .next()
                        .ok_or(RuntimeError::IvalidFunctionArgumentsError)?,
                )?;

                env.borrow_mut().define(s.to_string(), val.clone());
                return Ok(val.clone());
            }
            Err(RuntimeError::IvalidFunctionArgumentsError)
        }
        "let" => todo!(),
        "fn" => todo!(),
        "quote" => list_iter.next().ok_or(RuntimeError::EarlyListEndError),
        _ => panic!("expected special form got `{}`", special_form),
    }
}
