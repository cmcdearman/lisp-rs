#[cfg(test)]
mod tests {
    use lust::{
        ast::{Atom, Lit, Sexpr},
        env,
        eval::eval,
        lex::lex,
        parse::parse,
        token::TokenStream,
    };
    
    #[test]
    fn integration_test() {
        let input = vec![("(+ (/ 6 (* 1.5 2)) (- 1 (mod 5 2))) ; 2.0", 2.0)];
        for (src, ans) in input {
            match eval(
                &parse(&mut TokenStream::new(lex(&src)).peekable()),
                &mut env::default_env(),
            ) {
                Ok(_) => {}
                Err(_) => {}
            }
            // assert_eq!(
            //     eval(&parse(&mut TokenStream::new(lex(&src)).peekable())).expect("error eval"),
            //     ans
            // );
        }
    }
}
