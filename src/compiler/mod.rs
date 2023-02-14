use crate::parser::Parser;

pub struct Compiler<'src> {
    parser: Parser<'src>,
    bytecode: Vec<u8>,
}

impl<'src> Compiler<'src> {
    pub fn new(src: &'src str, lazy: bool) -> Self {
        Self {
            parser: Parser::new(src, lazy),
            bytecode: Vec::new(),
        }
    }

    fn macro_expand(&self) {}

    pub fn compile(&self) {}
}
