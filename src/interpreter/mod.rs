use std::{cell::RefCell, rc::Rc};

use crate::parser::{
    sexpr::{env::Env, Atom, Cons, ConsIter, Lit, Sexpr, NIL},
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
            println!("{:?}", &list_iter.clone().collect::<Vec<Sexpr>>());
            let first = list_iter.next().ok_or(RuntimeError::EarlyListEndError)?;

            if first.is_special_form() {
                return eval_special_form(
                    env.clone(),
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
                Sexpr::Lambda {
                    env: fn_env,
                    args,
                    body,
                } => {
                    let params = list_iter.collect::<Vec<Sexpr>>();
                    let mut arg_env = fn_env.borrow().create_child();
                    for (i, a) in args.iter().enumerate() {
                        if let Sexpr::Atom(Atom::Sym(s)) = a {
                            arg_env.define(s.to_string(), params[i].clone());
                        } else {
                            return Err(RuntimeError::EarlyListEndError);
                        }
                    }
                    eval(Rc::new(RefCell::new(arg_env)), &body)
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
        "fn" => {
            if !(2..4).contains(&list_iter.len()) {
                return Err(RuntimeError::IvalidFunctionArgumentsError);
            }

            let mut fn_args = vec![];
            if let Sexpr::List(l) = &list_iter.next().ok_or(RuntimeError::EarlyListEndError)? {
                fn_args = l.clone().into_iter().map(|x| x.clone()).collect();
            }

            println!("{:?}", list_iter.clone().collect::<Vec<Sexpr>>());
            let body = &list_iter
                .next()
                .ok_or(RuntimeError::IvalidFunctionArgumentsError)?;

            Ok(Sexpr::Lambda {
                env: Rc::new(RefCell::new(env.borrow().create_child())),
                args: fn_args,
                body: Box::new(body.clone()),
            })
        }
        "quote" => list_iter.next().ok_or(RuntimeError::EarlyListEndError),
        "if" => {
            if let Sexpr::Atom(Atom::Lit(Lit::Bool(b))) = eval(
                env.clone(),
                &list_iter.next().ok_or(RuntimeError::EarlyListEndError)?,
            )? {
                if b {
                    return eval(
                        env.clone(),
                        &list_iter
                            .next()
                            .ok_or(RuntimeError::IvalidFunctionArgumentsError)?,
                    );
                } else {
                    list_iter.next();
                    return eval(
                        env.clone(),
                        &list_iter
                            .next()
                            .ok_or(RuntimeError::IvalidFunctionArgumentsError)?,
                    );
                }
            }
            Err(RuntimeError::FirstElemError)
        }
        _ => panic!("expected special form got `{}`", special_form),
    }
}
