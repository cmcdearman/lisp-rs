use crate::{parser::ast::Expr, span::Spanned, vm::chunk::Chunk};

use self::error::CompileResult;

pub mod error;
mod tests;

pub fn compile(expr: &Spanned<Expr>) -> CompileResult<Chunk> {
    match expr.clone().0 {
        Expr::Symbol(_) => todo!(),
        Expr::Lit(l) => match l {
            crate::parser::ast::Lit::Int(i) => {
                let mut chunk = Chunk::new();
                chunk.add_constant(crate::vm::value::Value::Int(i));
                chunk.write(crate::vm::opcode::OpCode::Const as u8, expr.1);
                Ok(chunk)
            }
            crate::parser::ast::Lit::Rational(_) => todo!(),
            crate::parser::ast::Lit::Real(_) => todo!(),
            crate::parser::ast::Lit::Char(_) => todo!(),
            crate::parser::ast::Lit::String(_) => todo!(),
        },
        Expr::List(_) => todo!(),
        Expr::Lambda { params, body } => todo!(),
        Expr::Apply { func, args } => todo!(),
        Expr::Let { name, value, body } => todo!(),
        Expr::If { cond, then, else_ } => todo!(),
        Expr::Nil => todo!(),
    }
}
