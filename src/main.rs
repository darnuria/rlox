use core::fmt;
use std::fmt::write;

type Value = u32;

#[derive(Debug)]
enum Opcode {
    Return,
    Constant(u8),
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Opcode::Return => "RETURN",
            Opcode::Constant(c) => {
                write!(f, "constant {}", c)?;
                return fmt::Result::Ok(());
            }
        };
        write!(f, "{}", s)
    }
}

struct ValuePool {
    values: Vec<Value>,
}

impl ValuePool {
    fn new() -> ValuePool {
        ValuePool {
            values: Vec::with_capacity(4),
        }
    }
    // Write an opcode, one at a time.
    fn write_constant(&mut self, val: Value) {
        self.values.push(val)
    }
}

struct Chunk {
    // Well a vec is the direct translation of the "growable code zone" in the book.
    code: Vec<Opcode>,
    lines: Vec<u32>,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            code: Vec::with_capacity(8),
            lines: Vec::with_capacity(8),
        }
    }

    // Write an opcode, one at a time.
    fn write_opcode(&mut self, op: Opcode, line: u32) {
        self.code.push(op);
        self.lines.push(line);
    }

    // Disassemble a chunck and dump it.
    fn dissemble(&self, name: &str) -> String {
        format!("=== {} ===\n{}\n========", name, self)
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(self.lines.len() <= self.code.len());
        for (offset, op) in self.code.iter().enumerate() {
            let line = self.lines[offset];
            write!(f, "{:04} {:<16} | {}", offset, op, line)?;
        }
        fmt::Result::Ok(())
    }
}

fn main() {
    println!("Let's do a virtual machine!");
    let ret = Opcode::Return;
    let mut code = Chunk::new();
    code.write_opcode(ret, 1);
    println!("{}", code.dissemble("test"));
}
