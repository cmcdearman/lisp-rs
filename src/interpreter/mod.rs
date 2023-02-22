use std::{cell::RefCell, rc::Rc};

use crate::parser::{
    sexpr::{env::Env, Atom, Cons, ConsIter, Lit, Sexpr, NIL},
    Parser,
};

use self::runtime_error::{Result, RuntimeError};

pub mod default_env;
pub mod repl;
pub mod runtime_error;

pub fn eval(env: Rc<RefCell<Env>>, sexpr: &Sexpr) -> Result<Sexpr> {
    match sexpr {
        lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
        sym @ Sexpr::Atom(Atom::Sym(name)) => {
            if let Some(v) = env.borrow().find(&name) {
                println!("sym: {}", v);
                return Ok(v.clone());
            }

            Err(RuntimeError::new(&format!(
                "use of unbound Symbol `{}`",
                sym.clone()
            )))
        }
        Sexpr::List(l) => {
            let mut list_iter = l.clone().into_iter();
            let first = list_iter
                .next()
                .ok_or(RuntimeError::new("unexpected end to list"))?;

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
                    let arg_env = Rc::new(RefCell::new(Env::create_child(fn_env)));
                    for (i, a) in args.iter().enumerate() {
                        if let Sexpr::Atom(Atom::Sym(s)) = a {
                            arg_env
                                .borrow_mut()
                                .define(s.to_string(), params[i].clone());
                        } else {
                            return Err(RuntimeError::new(
                                "lambda arguments must be of type String",
                            ));
                        }
                    }
                    eval(arg_env.clone(), &body)
                }
                _ => Err(RuntimeError::new(&format!(
                    "first list element must be function call, got `{}`",
                    eval(env.clone(), &first)?
                ))),
            }
        }
        lambda @ Sexpr::Lambda {
            env: _,
            args: _,
            body: _,
        } => Ok(lambda.clone()),
        _ => Err(RuntimeError::new("unknown sexpr")),
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
                        .ok_or(RuntimeError::new("def takes 2 arguments. got 0"))?,
                )?;

                env.borrow_mut().define(s.to_string(), val.clone());

                println!("Env after def: {:p}", env.as_ref());

                return Ok(val.clone());
            }
            Err(RuntimeError::new("first def argument must be a Symbol"))
        }
        "let" => todo!(),
        "fn" => {
            let mut fn_args = vec![];
            if let Sexpr::List(l) = &list_iter
                .next()
                .ok_or(RuntimeError::new("fn takes 2 arguments, got 0"))?
            {
                fn_args = l.clone().into_iter().map(|x| x.clone()).collect();
            }

            let body = &list_iter
                .next()
                .ok_or(RuntimeError::new("fn takes 2 arguments, got 1"))?;

            Ok(Sexpr::Lambda {
                env: Rc::new(RefCell::new(Env::create_child(env))),
                args: fn_args,
                body: Box::new(body.clone()),
            })
        }
        "quote" => list_iter
            .next()
            .ok_or(RuntimeError::new("`quote` takes 1 argument, got 0")),
        "if" => {
            if let Sexpr::Atom(Atom::Lit(Lit::Bool(b))) = eval(
                env.clone(),
                &list_iter
                    .next()
                    .ok_or(RuntimeError::new("`if` takes 3 arguments, got 0"))?,
            )? {
                println!("{:?}", &list_iter.clone().collect::<Vec<Sexpr>>());
                if b {
                    return eval(
                        env.clone(),
                        &list_iter
                            .next()
                            .ok_or(RuntimeError::new("`if` takes 3 arguments, got 1"))?,
                    );
                } else {
                    list_iter.next().expect("list ended early");
                    println!("else: {:?}", &list_iter.clone().collect::<Vec<Sexpr>>());
                    return eval(
                        env.clone(),
                        &list_iter
                            .next()
                            .ok_or(RuntimeError::new("`if` takes 3 arguments, got 2"))?,
                    );
                }
            }
            Err(RuntimeError::new("`if` first argument must be Boolean"))
        }
        _ => panic!("expected special form got `{}`", special_form),
    }
}

mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        interpreter::default_env::default_env,
        parser::{
            sexpr::{env::Env, Atom, Lit, Number, Sexpr},
            Parser,
        },
    };

    use super::eval;

    #[test]
    fn eval_rec_gcd_call() {
        let env = default_env();
        let ast = &Parser::new(
            "(def gcd (fn (a b) (if (eq b 0) (a) (gcd b (mod a b)))))",
            false,
        )
        .parse()
        .expect("expected recursive test to parse");

        println!("ast: {}", ast);

        eval(env.clone(), ast).expect("expected recursive definition to eval");
        assert_eq!(
            eval(
                env.clone(),
                &Parser::new("(gcd 18 24)", false)
                    .parse()
                    .expect("expected recursive test to parse")
            )
            .expect("expected recursive test to pass"),
            Sexpr::Atom(Atom::Lit(Lit::Number(Number::Fixnum(6))))
        );
    }
}
