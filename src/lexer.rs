use std::io::Read;

struct Lexer {
    src: Vec<u8>,
    pos: u32,
    read_pos: u32,
    ch: char
}

impl Lexer {
    fn new(src: Vec<u8>) -> Self {
        Self { src, pos: 0, read_pos: 0, ch: *&'0' }
    }

    fn read(&self) {
        if self.read_pos >= self.src.len() {
            &self.ch = &'0';
        } else {
            &self.ch = self.src[self.read_pos]
        }
        &self.pos = &self.read_pos;
        &self.read_pos += 1;
    }
}