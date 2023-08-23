// use crate::{parser::ast::Expr, span::Spanned, vm::chunk::Chunk};

// use self::error::CompileResult;

// pub mod error;
// mod tests;

// #[derive(Debug)]
// pub struct Compiler {
//     chunk: Chunk,
// }

// impl Compiler {
//     pub fn new() -> Self {
//         Self {
//             chunk: Chunk::new(),
//         }
//     }

//     pub fn compile(&mut self, expr: &Spanned<Expr>) -> CompileResult<Chunk> {
//         match expr.clone().0 {
//             Expr::Symbol(_) => todo!(),
//             Expr::Lit(l) => match l {
//                 crate::parser::ast::Lit::Int(i) => {
//                     let idx = self.chunk.add_constant(crate::vm::value::Value::Int(i));
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Const as u8, expr.1);
//                     self.chunk.write(idx as u8, expr.1);
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Return as u8, expr.1);
//                     Ok(self.chunk.clone())
//                 }
//                 crate::parser::ast::Lit::Rational(r) => {
//                     let idx = self
//                         .chunk
//                         .add_constant(crate::vm::value::Value::Rational(r));
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Const as u8, expr.1);
//                     self.chunk.write(idx as u8, expr.1);
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Return as u8, expr.1);
//                     Ok(self.chunk.clone())
//                 }
//                 crate::parser::ast::Lit::Real(r) => {
//                     let idx = self.chunk.add_constant(crate::vm::value::Value::Real(r));
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Const as u8, expr.1);
//                     self.chunk.write(idx as u8, expr.1);
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Return as u8, expr.1);
//                     Ok(self.chunk.clone())
//                 }
//                 crate::parser::ast::Lit::Char(c) => {
//                     let idx = self.chunk.add_constant(crate::vm::value::Value::Char(c));
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Const as u8, expr.1);
//                     self.chunk.write(idx as u8, expr.1);
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Return as u8, expr.1);
//                     Ok(self.chunk.clone())
//                 }
//                 crate::parser::ast::Lit::String(s) => {
//                     let idx = self.chunk.add_constant(crate::vm::value::Value::String(s));
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Const as u8, expr.1);
//                     self.chunk.write(idx as u8, expr.1);
//                     self.chunk
//                         .write(crate::vm::opcode::OpCode::Return as u8, expr.1);
//                     Ok(self.chunk.clone())
//                 }
//             },
//             Expr::List(_) => todo!(),
//             Expr::BinaryOp { op, lhs, rhs } => todo!(),
//             Expr::UnaryOp { op, expr } => {
//                 self.compile(expr.as_ref())?;
//                 self.chunk
//                     .write(crate::vm::opcode::OpCode::Neg as u8, expr.1);
//                 self.chunk
//                     .write(crate::vm::opcode::OpCode::Return as u8, expr.1);
//                 Ok(self.chunk.clone())
//             }
//             Expr::Lambda { params, body } => todo!(),
//             Expr::Apply { func, args } => todo!(),
//             Expr::Let { name, value, body } => todo!(),
//             Expr::If { cond, then, else_ } => todo!(),
//             Expr::Nil => todo!(),
//         }
//     }
// }
