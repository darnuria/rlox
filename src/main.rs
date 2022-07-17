use core::fmt;
use std::{env::args, path::Path};

type Value = f64;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    /// (
    LeftParens,
    /// )
    RightParens,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// ,
    Comma,
    /// .
    Dot,
    /// -
    Minus,
    /// +
    Plus,
    /// ;
    Semicolon,
    /// /
    Slash,
    /// *
    Star,

    /// !
    Bang,
    /// !=
    BangEqual,
    /// =
    Equal,
    /// ==
    EqualEqual,
    /// >
    Greater,
    /// <=
    GreaterEqual,
    /// <
    Lesser,
    /// >=
    LesserEqual,

    // Litterals
    /// ???
    Identifier,
    /// "[.]*"
    String,
    /// 0-9
    Number,

    // KEYWORDS
    /// and
    And,
    /// or
    Or,
    /// struct / class
    Struct,
    /// if
    If,
    /// else
    Else,
    /// true
    True,
    /// false
    False,
    /// Function
    Fun,
    /// Loop
    Loop,
    /// While
    While,
    /// For
    For,
    /// Nil
    Nil,
    /// Return
    Return,
    /// let
    Let,
    /// print
    Print,
    /*
    Class,
    Super
     */
    TokSelf,
}

#[inline]
fn is_ascii_alphabetic_or_underscore(c: &u8) -> bool {
    c.is_ascii_alphabetic() || *c == b'_'
}

struct Scanner<'a> {
    code: &'a [u8],
    start: usize,
    cursor: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn new(code: &'a str) -> Scanner {
        Scanner {
            code: code.as_bytes(),
            // Only used in ident oupsy
            start: 0,
            cursor: 0,
            // end?
            line: 1,
        }
    }

    fn whitespaces_and_comments(&mut self) {
        while let Some(c) = self.code.get(self.cursor) {
            match c {
                b' ' | b'\r' | b'\t' => {
                    self.cursor += 1;
                    break;
                }
                b'\n' => {
                    self.line += 1;
                }
                b'/' => {
                    if let Some(b'/') = self.code.get(self.cursor + 1) {
                        self.cursor += 1;
                        while let Some(nomnom) = self.code.get(self.cursor + 1) {
                            match nomnom {
                                b'\n' => break,
                                _ => self.cursor += 1,
                            }
                            self.cursor += 1;
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn skip_whitespaces(&mut self) {
        self.cursor += self.code.iter().take_while(|c| **c == b' ').count();
    }

    fn string(&mut self) -> Result<Token, ScanError> {
        // let next = self
        // .code
        // .get(self.cursor + 1)
        // .ok_or(ScanError::UnmatchedString)?;

        let start = self.cursor;
        while let Some(c) = self.code.get(self.cursor + 1) {
            if *c == b'\n' {
                self.line += 1;
            } else if *c == b'"' {
                break;
            }
            self.cursor += 1;
        }
        if start == self.cursor {
            return Err(ScanError::UnmatchedString);
        }
        self.cursor += 1;
        Ok(Token::String)
    }

    fn numbers(&mut self) -> Result<Token, ScanError> {
        let mut iter = self.code.iter().peekable();
        while let Some(d) = iter.peek() {
            if d.is_ascii_digit() {
                iter.next();
                self.cursor += 1;
            } else {
                break;
            }
        }

        if iter.next() == Some(&b'.') {
            self.cursor += 1;
            while let Some(d) = iter.next() {
                if d.is_ascii_digit() {
                    self.cursor += 1;
                } else {
                    break;
                }
            }
        }
        Ok(Token::Number)
    }

    #[inline]
    fn advance(&mut self) -> u8 {
        let c = self.code[self.cursor];
        self.cursor += 1;
        c
    }

    fn identifier(&mut self) -> Result<Token, ScanError> {
        self.start = self.cursor;
        loop {
            match self.peek() {
                Some(c) if is_ascii_alphabetic_or_underscore(c) || c.is_ascii_digit() => {
                    self.advance();
                }
                _ => break,
            }
        }
        self.identifier_type()
    }

    fn identifier_type(&mut self) -> Result<Token, ScanError> {
        let s = &self.code[self.start..self.cursor];
        let tok = match s {
            b"false" => Token::False,
            b"true" => Token::True,
            b"self" => Token::TokSelf,
            b"struct" => Token::Struct,
            b"return" => Token::Return,
            b"loop" => Token::Loop,
            b"fun" => Token::Fun,
            // TODO make it a function
            b"print" => Token::Print,
            b"or" => Token::Or,
            b"and" => Token::And,
            b"for" => Token::For,
            b"while" => Token::While,
            b"if" => Token::If,
            b"else" => Token::Else,
            // TODO use Option<T>?
            b"nil" => Token::Nil,
            b"let" => Token::Let,
            _ => return Err(ScanError::UnknownToken),
        };
        Result::Ok(tok)
    }

    #[inline]
    fn peek(&self) -> Option<&u8> {
        self.code.get(self.cursor)
    }

    fn is_at_end(&self) -> bool {
        self.cursor == self.code.len() - 1
    }

    fn peek_next(&self) -> Option<&u8> {
        if self.is_at_end() {
            None
        } else {
            self.code.get(self.cursor + 1)
        }
    }

    fn scan_token(&mut self) -> Result<(Token, usize, usize), ScanError> {
        // TODO: Update lines.
        // TODO: Manage comments

        self.whitespaces_and_comments();
        let c = self.code.get(self.cursor).ok_or(ScanError::End)?;

        //let mut code = code.peekable();
        let tok = match c {
            b'0'..=b'9' => self.numbers()?,
            b'(' => Token::LeftParens,
            b')' => Token::RightParens,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b';' => Token::Semicolon,
            b',' => Token::Comma,
            b'.' => Token::Dot,
            b'-' => Token::Minus,
            b'+' => Token::Plus,
            b'/' => Token::Slash,
            b'*' => Token::Star,
            //    b'!' => if let Some(_) = code.next_if_eq(&'=') { Token::BangEqual } else { Token::Bang },
            //    b'=' => if let Some(_) = code.next_if_eq(&'=') { Token::EqualEqual } else { Token::Equal },
            //    b'<' => if let Some(_) = code.next_if_eq(&'=') { Token::LesserEqual } else { Token::Lesser },
            //    b'>' => if let Some(_) = code.next_if_eq(&'=') { Token::GreaterEqual } else { Token::Greater },
            b'"' => self.string()?,
            b'a'..=b'z' | b'_' => self.identifier()?,
            _ => return Err(ScanError::UnknownToken),
        };

        Result::Ok((tok, self.line, self.cursor - self.start))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ScanError {
    UnknownToken,
    UnmatchedString,
    End,
}

#[derive(Debug, PartialEq, Eq)]
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
    STDINnError,
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
        let b = stack.last_mut().ok_or(InterpretError::StackUnderflow)?;
        match op {
            Opcode::Add => *b += a,
            Opcode::Sub => *b -= a,
            Opcode::Div => *b /= a,
            Opcode::Mul => *b *= a,
            _ => unreachable!(),
        }
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

    fn compile(&mut self, code: &str) -> Result<Chunk, InterpretError> {
        let mut line = 0; // there is no line 0 :)
        let mut code = code.chars();

        loop {
            unimplemented!()
            //let tok = scan_token(&mut code).expect("Scan Whopsy");
        }
        unimplemented!()
    }

    fn run_file<P: AsRef<Path>>(&mut self, source_code: P) -> Result<(), InterpretError> {
        let code = std::fs::read_to_string(source_code).expect("Cannot found file?!");
        let chunk = self.compile(&code);
        unimplemented!()
    }

    fn eval(&mut self, code: &str) -> Result<(), InterpretError> {
        unimplemented!()
    }

    fn repl(mut self) -> Result<(), InterpretError> {
        let mut buffer = String::with_capacity(1024);
        loop {
            print!(">>>");
            std::io::stdin()
                .read_line(&mut buffer)
                .map_err(|_| InterpretError::STDINnError)?;
            println!("");
            self.eval(&buffer)?;
        }
    }
}

fn main() {
    let mut code = Chunk::new();
    code.write_value(42.);
    code.write_opcode(Opcode::Constant(0), 1);
    code.write_opcode(Opcode::Negate, 2);
    code.write_opcode(Opcode::Litteral(1152), 2);
    code.write_opcode(Opcode::Add, 3);
    code.write_opcode(Opcode::Return, 4);
    let mut vm = VirtualMachine::new(code);
    let mut args = args();
    if (args.len()) == 1 {
        vm.repl().expect("Whops REPL ERROR");
    } else if args.len() == 2 {
        let file = args.next().expect("Missing filepath");
        vm.run_file(&file).expect("Whops Compile/Interp ERROR");
    } else {
        println!("Usage: rlox [path]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_no_string() {
        let code = r#"NoString"#;
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Err(ScanError::UnknownToken));
    }

    #[test]
    fn test_token_string() {
        let code = r#""test""#;
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::String, 1, 5)));
    }

    #[test]
    fn test_unmatched_string() {
        let code = r#"""#;
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Err(ScanError::UnmatchedString));
    }

    #[test]
    fn test_empty_string() {
        let code = r#""""#;

        println!("{}", code);
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::String, 1, 0)));
    }
    #[test]
    fn test_unmatched_char() {
        let code = r#"e""#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Err(ScanError::UnknownToken));
    }

    #[test]
    fn test_number_sigle() {
        let code = r#"0"#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Number, 1, 1)));
    }

    #[test]
    fn test_number_lenght() {
        let code = r#"123456789"#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Number, 1, 9)));
    }

    #[test]
    fn test_number_end_fractionnal() {
        let code = r#"123456789."#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Number, 1, 10)));
    }

    #[test]
    fn test_number_fractional_part() {
        let code = r#"12345.6789"#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Number, 1, 10)));
    }

    #[test]
    fn test_scan_tok_if() {
        let code = r#"if else fun"#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::If, 1, 2)));
        assert_eq!(scan.scan_token(), Ok((Token::Else, 1, 4)));
        assert_eq!(scan.scan_token(), Ok((Token::Fun, 1, 3)));
    }
}
