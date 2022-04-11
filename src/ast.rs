// pub trait Node {
//     fn token_literal() -> String;
// }
//
// pub struct StatementNode {
//
// }
//
// pub struct ExpressionNode {
//
// }

type Operator = u64;
type Literal = String;

pub enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(Box<BinaryExpr>),
    Grouping(Box<Expr>),
    Literal(Literal)
}

struct BinaryExpr {
    op: Operator,
    lhs: Expr,
    rhs: Expr,
}
