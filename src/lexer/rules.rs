use crate::T;
use crate::token::TokenKind;

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

pub(crate) fn get_rules() -> Vec<Rule> {
    vec![
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