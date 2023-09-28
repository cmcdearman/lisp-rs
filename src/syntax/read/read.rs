use super::{
    sexpr::{Atom, Root},
    token::Token,
};
use crate::util::{intern::InternedString, node::SrcNode, span::Span};
use chumsky::{
    extra,
    input::{Stream, ValueInput},
    prelude::{Input, Rich},
    select, IterParser, Parser,
};
use logos::Logos;

pub type ReadError<'a> = extra::Err<Rich<'a, Token, Span>>;

fn ident_reader<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, InternedString, extra::Err<Rich<'a, Token, Span>>> {
    select! {
        Token::Symbol(name) => InternedString::from(name),
    }
}

fn atom_reader<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, SrcNode<Atom>, extra::Err<Rich<'a, Token, Span>>> {
    select! {
        Token::Symbol(name) => Atom::Symbol(InternedString::from(name)),
        Token::Number(n) => Atom::Number(n),
        Token::String(s) => Atom::String(s),
    }
}

fn reader<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, SrcNode<Root>, extra::Err<Rich<'a, Token, Span>>> {
    atom_reader()
        .repeated()
        .collect()
        .map(|sexprs| Root { sexprs })
}

pub fn read<'src>(src: &'src str) -> (Option<Root>, Vec<ReadError<'src>>) {
    let tokens = Token::lexer(&src).spanned().map(|(tok, span)| match tok {
        Ok(tok) => (tok, Span::from(span)),
        Err(err) => panic!("lex error: {:?}", err),
    });
    let tok_stream = Stream::from_iter(tokens).spanned(Span::from(src.len()..src.len()));
    reader().parse(tok_stream).into_output_errors()
}

mod tests {
    // use super::Reader;

    // #[test]
    // fn read_int() {
    //     let src = "42";
    //     let mut reader = Reader::new(src);
    //     let (root, errs) = reader.read();
    //     if !errs.is_empty() {
    //         panic!("{:?}", errs);
    //     }
    //     insta::assert_debug_snapshot!(root);
    // }

    // #[test]
    // fn read_list() {
    //     let src = "(1 2 3)";
    //     let mut reader = Reader::new(src);
    //     let (root, errs) = reader.read();
    //     if !errs.is_empty() {
    //         panic!("{:?}", errs);
    //     }
    //     insta::assert_debug_snapshot!(root);
    // }
}
