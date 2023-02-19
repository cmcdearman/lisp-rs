use crate::parser::{
    sexpr::{env::Env, Atom, Sexpr},
    Parser,
};

pub mod default_env;
pub mod repl;
pub mod runtime_error;

pub struct Interpreter<'src> {
    src: &'src str,
    parser: Parser<'src>,
}

impl<'src> Interpreter<'src> {
    pub fn new(src: &'src str, lazy: bool) -> Self {
        Self {
            src,
            parser: Parser::new(src, lazy),
        }
    }

    // pub fn eval(&self, env: Box<Env>) -> Result<Sexpr, String> {
    //     match self.parser.parse() {
    //         lit @ Sexpr::Atom(Atom::Lit(_)) => Ok(lit.clone()),
    //         Sexpr::Atom(Atom::Sym(name)) => Ok(env
    //             .find(&name)
    //             .ok_or_else(|| format!("name `{}` not found", name))?
    //             .clone()),
    //         Sexpr::List(head) => {
    //             todo!()
    //         }
    //         Sexpr::Lambda { args, body } => todo!(),
    //         Sexpr::NativeFn(_) => todo!(),
    //     }
    // }
}
