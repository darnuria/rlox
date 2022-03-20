use core::fmt;
use std::ops::RangeBounds;

type Value = u32;

#[derive(Debug)]
enum Opcode {
    Return,
    Constant(u8),
    Litteral(u16), // Store directly value
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Opcode::Return => "RETURN",
            Opcode::Constant(c) => {
                write!(f, "CONSTANT {}", c)?;
                return fmt::Result::Ok(());
            }
            Opcode::Litteral(v) => {
                write!(f, "LITERRAL {}", v)?;
                return fmt::Result::Ok(());
            }
        };
        write!(f, "{}", s)
    }
}

struct Chunk {
    // Well a vec is the direct translation of the "growable code zone" in the book.
    code: Vec<Opcode>,
    values: Vec<Value>,
    lines: Vec<(u8, u16)>, // repeat + lines
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            code: Vec::with_capacity(8),
            values: Vec::with_capacity(4),
            lines: Vec::with_capacity(8),
        }
    }

    fn write_value(&mut self, v: Value) {
        self.values.push(v);
    }
    // Write an opcode, one at a time.
    fn write_opcode(&mut self, op: Opcode, line: u16) {
        self.code.push(op);
        match self.lines.last_mut() {
            Some((repeat, last)) if *last == line => {
                *repeat += 1;
            }
            _ => self.lines.push((1, line)),
        }
    }

    // Disassemble a chunck and dump it.
    fn dissemble(&self, name: &str) -> String {
        format!("=== {} ===\n{}========", name, self)
    }
}

fn get_line(lines: &[(u8, u16)], idx: usize) -> Option<u16> {
    // Assertion : lines.iter().map(|(r, _), r).sum() == idx
    // partial sum of rep >= idx.
    let mut count = 0usize;
    for (rep, line) in lines.iter() {
        let up = count + (*rep as usize);
        if count == idx || (count..up).contains(&idx) {
            return Some(*line);
        }
        count += *rep as usize;
    }
    None
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(self.lines.len() <= self.code.len());
        for (offset, op) in self.code.iter().enumerate() {
            let line = get_line(&self.lines, offset).unwrap();
            write!(f, "{:04}:ln {} ", offset, line)?;
            match op {
                &Opcode::Constant(i) => write!(f, "{} {}", op, self.values[i as usize])?,
                _ => write!(f, "{}", op)?,
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

fn main() {
    println!("Let's do a virtual machine!");
    let mut code = Chunk::new();
    code.write_opcode(Opcode::Return, 1);
    code.write_value(42);
    code.write_opcode(Opcode::Constant(0), 2);
    code.write_opcode(Opcode::Litteral(1152), 2);
    code.write_opcode(Opcode::Return, 3);

    code.write_opcode(Opcode::Return, 4);
    code.write_opcode(Opcode::Return, 4);
    code.write_opcode(Opcode::Return, 4);
    println!("{}", code.dissemble("test"));
}
