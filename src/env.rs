use crate::ast::{Atom, Lit, Sexpr};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env {
    data: HashMap<String, Sexpr>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn push(&mut self, name: String, value: Sexpr) {
        self.data.insert(name, value);
    }

    pub fn find(&self, name: String) -> Result<Sexpr, String> {
        match self.data.get(&name) {
            Some(sexpr) => Ok(sexpr.clone()),
            None => Err(String::from("could not find name in env"))
        }
    }

    pub fn pop(&mut self, name: String) {
        self.data.remove(&*name);
    }
}

pub fn default_env() -> Env {
    let mut data: HashMap<String, Sexpr> = HashMap::new();
    data.insert(
        "+".to_string(),
        Sexpr::Fn(|args: &[Sexpr]| -> Result<Sexpr, String> {
            Ok(Sexpr::Atom(Atom::Lit(Lit::Num(sum_num_list(args)?))))
        }),
    );
    data.insert(
        "-".to_string(),
        Sexpr::Fn(|args: &[Sexpr]| -> Result<Sexpr, String> {
            Ok(Sexpr::Atom(Atom::Lit(Lit::Num(sub_num_list(args)?))))
        }),
    );
    data.insert(
        "*".to_string(),
        Sexpr::Fn(|args: &[Sexpr]| -> Result<Sexpr, String> {
            Ok(Sexpr::Atom(Atom::Lit(Lit::Num(mul_num_list(args)?))))
        }),
    );
    data.insert(
        "/".to_string(),
        Sexpr::Fn(|args: &[Sexpr]| -> Result<Sexpr, String> {
            Ok(Sexpr::Atom(Atom::Lit(Lit::Num(quo_num_list(args)?))))
        }),
    );
    data.insert(
        "let".to_string(),
        Sexpr::Fn(|args: &[Sexpr]| -> Result<Sexpr, String> {
            Ok(Sexpr::Atom(Atom::Lit(Lit::Num(sum_num_list(args)?))))
        }),
    );
    data.insert(
        "mod".to_string(),
        Sexpr::Fn(|args: &[Sexpr]| -> Result<Sexpr, String> {
            Ok(Sexpr::Atom(Atom::Lit(Lit::Num(mod_num_list(args)?))))
        }),
    );
    data.insert("lambda".to_string(), Sexpr::Fn(|args: &[Sexpr]| -> Result<Sexpr, String> {
        if !(2..4).contains(&args.len()) {
            return Err("not enough arguments for lambda expression".to_string());
        }
        let params = &args[0];
        let body = &args[1];
        let mut lambda_args; 
        if args.len() == 3 {
            lambda_args = &args[2];
        }
        todo!()
    }));
    Env { data }
}

fn sum_num_list(args: &[Sexpr]) -> Result<f64, String> {
    args.iter()
        .map(|s| -> Result<f64, String> {
            match s {
                Sexpr::Atom(Atom::Lit(Lit::Num(n))) => Ok(*n),
                _ => Err(String::from("error converting arguments to numbers")),
            }
        })
        .sum()
}

fn sub_num_list(args: &[Sexpr]) -> Result<f64, String> {
    let first = match args[0] {
        Sexpr::Atom(Atom::Lit(Lit::Num(n))) => n,
        _ => Err(String::from("error converting sub arguments to numbers"))?
    };

    Ok(first - sum_num_list(&args[1..])?)
}

fn mul_num_list(args: &[Sexpr]) -> Result<f64, String> {
    args.iter()
        .map(|s| -> Result<f64, String> {
            match s {
                Sexpr::Atom(Atom::Lit(Lit::Num(n))) => Ok(*n),
                _ => Err(String::from("error converting mul arguments to numbers"))?
            }
        })
        .product()
}

fn quo_num_list(args: &[Sexpr]) -> Result<f64, String> {
    let first = match args[0] {
        Sexpr::Atom(Atom::Lit(Lit::Num(n))) => n,
        _ => Err(String::from("error converting quo arguments to numbers"))?
    };

    Ok(first / mul_num_list(&args[1..])?)
}

fn mod_num_list(args: &[Sexpr]) -> Result<f64, String> {
    if args.len() != 2 { return Err("need two args for mod".to_string()); }
    let num = match args[0] {
        Sexpr::Atom(Atom::Lit(Lit::Num(n))) => n,
        _ => Err(String::from("error converting quo arguments to numbers"))?
    };
    let div = match args[1] {
        Sexpr::Atom(Atom::Lit(Lit::Num(n))) => n,
        _ => Err(String::from("error converting quo arguments to numbers"))?
    };

    Ok(num % div)
}