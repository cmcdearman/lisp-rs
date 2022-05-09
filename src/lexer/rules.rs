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
