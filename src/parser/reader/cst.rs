use crate::interner::InternedString;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: SyntaxKind,
    pub children: Vec<Child>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxKind {
    Root,
    Identifier,
    Number,
    String,
    Boolean,
    Null,
    Array,
    Object,
    Pair,
    Comma,
    Colon,
    Whitespace,
    Comment,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub text: InternedString,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Child {
    Node(Node),
    Token(Token),
}
