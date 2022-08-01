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

#[derive(Debug)]
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
        while let Some(c) = self.peek() {
            match c {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                    break;
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' => {
                    if let Some(b'/') = self.peek_next() {
                        while let Some(nomnom) = self.peek() {
                            match nomnom {
                                b'\n' => break,
                                _ => self.advance(),
                            };
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Result<Token, ScanError> {
        while let Some(c) = self.peek_next() {
            if *c == b'"' {
                break;
            }
            if *c == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScanError::UnmatchedString);
        }
        self.advance();
        // TODO: Hack for empty strings, it's ugly.
        if self.cursor - self.start == 1 {
            self.cursor += 1;
        }
        Ok(Token::String)
    }

    fn numbers(&mut self) -> Result<Token, ScanError> {
        let mut iter = self.code.iter().peekable();
        while let Some(d) = iter.peek() {
            if d.is_ascii_digit() {
                iter.next();
                self.advance();
            } else {
                break;
            }
        }

        if iter.next() == Some(&b'.') {
            self.advance();
            for d in iter {
                if d.is_ascii_digit() {
                    self.advance();
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

    /// Helper function derived from crafting interpreter eponnymous function.
    /// if it match `expected` advance the cursor.
    /// Returns None if it's impossible or not found
    fn match_bin(&mut self, expected: u8) -> Option<()> {
        if *self.code.get(self.cursor)? != expected {
            None
        } else {
            self.cursor += 1;
            Some(())
        }
    }

    /// Try to scan for a token one at a time.
    ///
    /// Internally advance the cursor on the current character.
    ///
    /// Returns either:
    /// - A [Token], the line and the start position of the found token.
    /// - A [ScanError] right now, the end of the parsing is... an error
    /// May be changed in the future if I move to an Iterator.
    fn scan_token(&mut self) -> Result<(Token, usize, usize), ScanError> {
        // TODO: Update lines.
        // TODO: Manage comments

        self.whitespaces_and_comments();

        // TODO: Use advance?
        //let c = self.code.get(self.cursor).ok_or(ScanError::End)?;
        let c = self.advance();
        //        self.cursor += 1;

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
            b'!' => {
                if let Some(_) = self.match_bin(b'=') {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            b'=' => {
                if let Some(_) = self.match_bin(b'=') {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            b'<' => {
                if let Some(_) = self.match_bin(b'=') {
                    Token::LesserEqual
                } else {
                    Token::Lesser
                }
            }
            b'>' => {
                if let Some(_) = self.match_bin(b'=') {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
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

#[derive(Debug)]
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
            writeln!(f)?;
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
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
        let mut scanner = Scanner::new(code);
        loop {
            // TODO manage error real world will crash at the end of file ahah.
            match scanner.scan_token() {
                Ok((tok, line, start)) => {
                    println!("tok: {tok:#?} {line} {start}");
                }
                Err(ScanError::End) => unimplemented!("End of file!"),
                Err(_) => return Err(InterpretError::Compile),
            }
        }
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
            println!();
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
        assert_eq!(scan.scan_token(), Ok((Token::String, 1, 2)));
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
    fn test_scan_tok_real() {
        let code = r#"if else fun "hello""#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::If, 1, 2)));
        assert_eq!(scan.scan_token(), Ok((Token::Else, 1, 4)));
        assert_eq!(scan.scan_token(), Ok((Token::Fun, 1, 3)));
        assert_eq!(scan.scan_token(), Ok((Token::String, 1, 6)));
    }

    #[test]
    fn test_scan_math() {
        let code = r#"+ > >= <= < =="#;

        // TODO : Fix start calculation it's broken.
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Plus, 1, 1)));
        assert_eq!(scan.scan_token(), Ok((Token::Greater, 1, 3)));
        assert_eq!(scan.scan_token(), Ok((Token::GreaterEqual, 1, 6)));
        assert_eq!(scan.scan_token(), Ok((Token::LesserEqual, 1, 9)));
        assert_eq!(scan.scan_token(), Ok((Token::Lesser, 1, 11)));
        assert_eq!(scan.scan_token(), Ok((Token::EqualEqual, 1, 14)));
    }
}
