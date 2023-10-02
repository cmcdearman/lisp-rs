use super::{
    sexpr::{Atom, Cons, Root, Sexpr},
    token::Token,
};
use crate::util::{intern::InternedString, list::List, node::SrcNode, span::Span};
use chumsky::{
    extra,
    input::{Stream, ValueInput},
    prelude::{Input, Rich},
    primitive::just,
    recursive::recursive,
    select, IterParser, Parser,
};
use logos::Logos;

pub type ReadError<'a> = Rich<'a, Token, Span>;

fn sexpr_reader<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, SrcNode<Sexpr>, extra::Err<Rich<'a, Token, Span>>> {
    recursive(|sexpr| {
        let atom = select! {
            Token::Symbol(name) => Atom::Symbol(InternedString::from(name)),
            Token::Number(n) => Atom::Number(n),
            Token::String(s) => Atom::String(s),
        }
        .map_with_span(SrcNode::new)
        .map(Sexpr::Atom);

        // let quote = just(Token::Quote)
        //     .map_with_span(SrcNode::new)
        //     .then(sexpr.clone())
        //     .map(|(q, sexpr)| {
        //         let quote = Sexpr::Atom(SrcNode::new(
        //             Atom::Symbol(InternedString::from("quote")),
        //             q.span(),
        //         ));
        //         vec![SrcNode::new(quote, q.span()), sexpr]
        //             .into_iter()
        //             .rev()
        //             .collect::<List<_>>()
        //     })
        //     .map(Sexpr::Cons);

        // let quasiquote = just(Token::Backquote)
        //     .map_with_span(SrcNode::new)
        //     .then(sexpr.clone())
        //     .map(|(q, sexpr)| {
        //         let quote = Sexpr::Atom(SrcNode::new(
        //             Atom::Symbol(InternedString::from("quasiquote")),
        //             q.span(),
        //         ));
        //         vec![SrcNode::new(quote, q.span()), sexpr]
        //             .into_iter()
        //             .rev()
        //             .collect::<List<_>>()
        //     })
        //     .map(Sexpr::Cons);

        // let unquote = just(Token::Comma)
        //     .map_with_span(SrcNode::new)
        //     .then(sexpr.clone())
        //     .map(|(q, sexpr)| {
        //         let quote = Sexpr::Atom(SrcNode::new(
        //             Atom::Symbol(InternedString::from("unquote")),
        //             q.span(),
        //         ));
        //         vec![SrcNode::new(quote, q.span()), sexpr]
        //             .into_iter()
        //             .rev()
        //             .collect::<List<_>>()
        //     })
        //     .map(Sexpr::Cons);

        // let dot = sexpr
        //     .then_ignore(just(Token::Period))
        //     .then(sexpr.clone())
        //     .map(|(car, cdr)| List::Node(Box::new(SrcNode::new(Sexpr::Atom(car), car.span())), ))
        //     .map(Sexpr::Cons)
        //     .delimited_by(just(Token::LParen), just(Token::RParen));

        let list = sexpr
            .foldl(
                sexpr.repeated(),
                |car: SrcNode<Sexpr>, cdr: SrcNode<Sexpr>| {
                    match cdr.inner() {
                        Sexpr::Cons(cons) => {
                            todo!()
                        }
                        _ => car.inner().clone(),
                    }
                },
            )
            .map(Sexpr::Cons)
            .delimited_by(just(Token::LParen), just(Token::RParen));

        atom.or(list)
            // .or(quote)
            // .or(quasiquote)
            // .or(unquote)
            // .or(dot)
            .map_with_span(SrcNode::new)
    })
}

fn reader<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, SrcNode<Root>, extra::Err<Rich<'a, Token, Span>>> {
    sexpr_reader()
        .repeated()
        .collect()
        .map(|sexprs| Root { sexprs })
        .map_with_span(SrcNode::new)
}

pub fn read<'src>(src: &'src str) -> (Option<SrcNode<Root>>, Vec<ReadError<'src>>) {
    let tokens = Token::lexer(&src).spanned().map(|(tok, span)| match tok {
        Ok(tok) => (tok, Span::from(span)),
        Err(err) => panic!("lex error: {:?}", err),
    });
    let tok_stream = Stream::from_iter(tokens).spanned(Span::from(src.len()..src.len()));
    reader().parse(tok_stream).into_output_errors()
}

mod tests {
    use crate::syntax::reader::read::read;

    #[test]
    fn read_int() {
        let src = "42";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_list() {
        let src = "(1 2 3)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_quote() {
        let src = "'(1 2 3)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_quasiquote() {
        let src = "`(1 2 3)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_unquote() {
        let src = ",(1 2 3)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_quasi_unquote() {
        let src = "`(1 ,(+ 1 1) 3)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }
}
