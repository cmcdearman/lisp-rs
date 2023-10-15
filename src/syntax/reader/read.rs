use super::{
    sexpr::{Atom, Pair, Root, Sexpr},
    token::Token,
};
use crate::util::{intern::InternedString, node::SrcNode, span::Span};
use chumsky::{
    extra,
    input::{Stream, ValueInput},
    prelude::{Input, Rich},
    primitive::just,
    recursive::recursive,
    select, IterParser, Parser,
};
use logos::Logos;
use std::cmp::max;

pub type ReadError<'a> = Rich<'a, Token, Span>;

fn sexpr_reader<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, Sexpr, extra::Err<Rich<'a, Token, Span>>> {
    recursive(|sexpr| {
        let atom = select! {
            Token::Symbol(name) => Atom::Symbol(InternedString::from(name)),
            Token::Number(n) => Atom::Number(n),
            Token::String(s) => Atom::String(s),
        }
        .map_with_span(SrcNode::new)
        .map(Sexpr::Atom);

        let quote = just(Token::Quote)
            .map_with_span(SrcNode::new)
            .then(sexpr.clone())
            .map(|(q, sexpr)| {
                let quote = Sexpr::Atom(SrcNode::new(
                    Atom::Symbol(InternedString::from("quote")),
                    q.span(),
                ));
                Sexpr::from_iter(vec![quote, sexpr].into_iter().rev())
            });

        // let quasiquote = just(Token::Backquote)
        //     .map_with_span(SrcNode::new)
        //     .then(sexpr.clone())
        //     .map(|(q, sexpr)| {
        //         let quote = Sexpr::Atom(SrcNode::new(
        //             Atom::Symbol(InternedString::from("quasiquote")),
        //             q.span(),
        //         ));
        //         Sexpr::from_iter(vec![quote, sexpr].into_iter().rev())
        //     });

        let unquote = just(Token::Comma)
            .map_with_span(SrcNode::new)
            .then(sexpr.clone())
            .map(|(q, sexpr)| {
                let quote = Sexpr::Atom(SrcNode::new(
                    Atom::Symbol(InternedString::from("unquote")),
                    q.span(),
                ));
                Sexpr::from_iter(vec![quote, sexpr].into_iter().rev())
            });

        let unquote_splice = just(Token::CommaAt)
            .map_with_span(SrcNode::new)
            .then(sexpr.clone())
            .map(|(q, sexpr)| {
                let quote = Sexpr::Atom(SrcNode::new(
                    Atom::Symbol(InternedString::from("unquote-splice")),
                    q.span(),
                ));
                Sexpr::from_iter(vec![quote, sexpr].into_iter().rev())
            });

        let dot = sexpr
            .clone()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(just(Token::Period))
            .then(sexpr.clone())
            .map(|(list, tail)| {
                list.into_iter().rev().fold(tail, |acc, next| {
                    Sexpr::Pair(SrcNode::new(
                        Pair::new(next.clone(), acc.clone()),
                        Span::new(next.span().start, max(acc.span().end, next.span().end)),
                    ))
                })
            })
            .delimited_by(just(Token::LParen), just(Token::RParen));

        let list = sexpr
            .repeated()
            .collect::<Vec<_>>()
            .map(|sexprs| Sexpr::from_iter(sexprs.into_iter().rev()))
            .delimited_by(just(Token::LParen), just(Token::RParen));

        atom.or(list)
            .or(quote)
            .or(unquote)
            .or(unquote_splice)
            .or(dot)
            .boxed()
    })
}

fn reader<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, SrcNode<Root>, extra::Err<Rich<'a, Token, Span>>> {
    sexpr_reader()
        .repeated()
        .collect()
        .map(Root)
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
        // (1 . (2 . (3 . ())))
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

    // #[test]
    // fn read_quasiquote() {
    //     let src = "`(1 2 3)";
    //     let (root, errs) = read(src);
    //     if !errs.is_empty() {
    //         panic!("{:?}", errs);
    //     }
    //     insta::assert_debug_snapshot!(root.unwrap());
    // }

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
    fn read_quote_unquote() {
        let src = "'(1 ,(+ 1 1) 3)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_dot() {
        let src = "(1 . 2)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_list_dot() {
        let src = "(1 2 . 3)";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }

    #[test]
    fn read_advanced() {
        let src = "(macro (for-each x in . body)\n
        `(loop (let x in)\n
           (if (not (empty? x))\n
               (begin . ,body)\n
               (for-each . ,body))))";
        let (root, errs) = read(src);
        if !errs.is_empty() {
            panic!("{:?}", errs);
        }
        insta::assert_debug_snapshot!(root.unwrap());
    }
}
