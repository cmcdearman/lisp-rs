use lazy_static::lazy_static;
use regex::Regex;
use crate::T;
use crate::token::TokenKind;
use crate::token::TokenKind::Number;

pub(crate) const fn unambiguous_single_char(c: char) -> Option<TokenKind> {
    Some(match c {
        '+' => T![+],
        '-' => T![-],
        '*' => T![*],
        '/' => T![/],
        '%' => T![%],
        '(' => T!['('],
        ')' => T![')'],
        _ => return None,
    })
}

pub(crate) struct Rule {
    pub kind:    TokenKind,
    pub matches: fn(&str) -> Option<u32>,
}

fn match_single_char(input: &str, c: char) -> Option<u32> {
    input.chars().next()
        .and_then(|ch| if ch == c { Some(1) } else { None })
}

fn match_keyword(input: &str, keyword: &str) -> Option<u32> {
    input.starts_with(keyword)
        .then(|| keyword.len() as u32)
}

fn match_regex(input: &str, r: &Regex) -> Option<u32> {
    r.find(input).map(|regex_match| regex_match.end() as u32)
}

lazy_static! {
    static ref STRING_REGEX: Regex =
        Regex::new(r#"^"((\\"|\\\\)|[^\\"])*""#).unwrap();
    static ref COMMENT_REGEX: Regex =
        Regex::new(r#"^//[^\n]*\n"#).unwrap();
    static ref NUMBER_REGEX: Regex =
        Regex::new(r#"^((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#).unwrap();
    static ref IDENTIFIER_REGEX: Regex =
        Regex::new(r##"^([A-Za-z]|_)([A-Za-z]|_|\d)*"##).unwrap();
}

pub(crate) fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            kind:    T![comment],
            matches: move |input| match_regex(input, &COMMENT_REGEX),
        },
        Rule {
            kind:    T![ident],
            matches: |input| match_regex(input, &IDENTIFIER_REGEX),
        },
        Rule {
            kind:    T![number],
            matches: |input| match_regex(input, &NUMBER_REGEX),
        },
        Rule {
            kind:    T![string],
            matches: move |input| match_regex(input, &STRING_REGEX),
        },
        Rule {
            kind: T![let],
            matches: |input| match_keyword(input, "let")
        },
        Rule {
            kind: T![lambda],
            matches: |input| match_keyword(input, "lambda")
        }
    ]
}