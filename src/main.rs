use core::fmt;
use std::fmt::write;

#[derive(Debug)]
enum Opcode {
    Return = 0,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Opcode::Return => "return",
        };
        write!(f, "{}", s)
    }
}
struct Chunk {
    // Well a vec is the direct translation of the "growable code zone" in the book.
    code: Vec<u8>,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            code: Vec::with_capacity(8),
        }
    }

    // Write an opcode, one at a time.
    fn write_opcode(&mut self, op: Opcode) {
        self.code.push(op as u8)
    }

    // Disassemble a chunck and dump it.
    fn dissemble(&self) -> String {
        format!("=== %s ===\n{} ========", self)
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (offset, &op) in self.code.iter().enumerate() {
            write!(f, "{:04} {}", offset, op)?;
        }
        fmt::Result::Ok(())
    }
}

fn main() {
    println!("Let's do a virtual machine!");
    let ret = Opcode::Return;
    let mut code = Chunk::new();
    code.write_opcode(ret);
}
