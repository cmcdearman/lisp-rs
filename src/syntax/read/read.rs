use super::token::Token;
use crate::util::{intern::InternedString, span::Span};
use chumsky::{extra, input::ValueInput, prelude::Rich, select, Parser};

fn ident_parser<'a, I: ValueInput<'a, Token = Token, Span = Span>>(
) -> impl Parser<'a, I, InternedString, extra::Err<Rich<'a, Token, Span>>> {
    select! {
        Token::Symbol(name) => InternedString::from(name),
    }
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
