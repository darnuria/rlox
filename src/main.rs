use core::fmt;

type Value = f64;

#[derive(Debug)]
enum Opcode {
    Return,
    Negate,
    Add,
    Sub,
    Mul,
    Div,
    Constant(u8),
    Litteral(u16), // Store directly value
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Opcode::Negate => "NEGATE",
            Opcode::Return => "RETURN",
            Opcode::Constant(c) => {
                write!(f, "CONSTANT {}", c)?;
                return fmt::Result::Ok(());
            }
            Opcode::Litteral(v) => {
                write!(f, "LITERRAL {}", v)?;
                return fmt::Result::Ok(());
            }
            Opcode::Add => "ADD",
            Opcode::Sub => "SUB",
            Opcode::Mul => "MUL",
            Opcode::Div => "DIV",
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

struct VirtualMachine {
    chunk: Chunk,
    stack: Vec<Value>,
    //ip: usize,
}

#[derive(Debug)]
enum InterpretError {
    Compile,
    Runtime,
    StackUnderflow,
}

impl VirtualMachine {
    fn new(chunk: Chunk) -> VirtualMachine {
        VirtualMachine {
            chunk,
            stack: Vec::with_capacity(256),
        }
    }

    #[inline]
    fn exec_binop(stack: &mut Vec<Value>, op: &Opcode) -> Result<(), InterpretError> {
        let a = stack.pop().ok_or(InterpretError::StackUnderflow)?;
        let b = stack.pop().ok_or(InterpretError::StackUnderflow)?;
        let result = match op {
            Opcode::Add => a + b,
            Opcode::Sub => b - a,
            Opcode::Div => b / a,
            Opcode::Mul => a * b,
            _ => unreachable!(),
        };
        stack.push(result);
        Ok(())
    }
    fn run(mut self) -> Result<(), InterpretError> {
        println!("{}", self.chunk.dissemble("debug"));
        let mut ip = 0;
        loop {
            let opcode = &self.chunk.code.get(ip).ok_or(InterpretError::Runtime)?;
            match opcode {
                Opcode::Add | Opcode::Mul | Opcode::Div | Opcode::Sub => {
                    Self::exec_binop(&mut self.stack, opcode)?
                }
                Opcode::Negate => {
                    let a = self
                        .stack
                        .last_mut()
                        .ok_or(InterpretError::StackUnderflow)?;
                    *a = -*a;
                }
                Opcode::Return => {
                    let ret = self.stack.pop().ok_or(InterpretError::StackUnderflow)?;
                    println!("{}", ret);
                    return Ok(());
                }
                Opcode::Constant(n) => {
                    let constant = self.chunk.values[*n as usize];
                    self.stack.push(constant);
                    println!("{}", constant)
                }
                Opcode::Litteral(litteral) => {
                    self.stack.push(*litteral as Value);
                    println!("{}", litteral)
                }

                _ => unimplemented!(),
            }
            ip += 1;
            println!("{:?}", self.stack);
        }
    }
}

fn main() {
    println!("Let's do a virtual machine!");
    let mut code = Chunk::new();
    code.write_value(42.);
    code.write_opcode(Opcode::Constant(0), 1);
    code.write_opcode(Opcode::Negate, 2);
    code.write_opcode(Opcode::Litteral(1152), 2);
    code.write_opcode(Opcode::Return, 3);

    let vm = VirtualMachine::new(code);
    vm.run().expect("Whops?");
}
