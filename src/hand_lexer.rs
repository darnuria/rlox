use core::fmt;
use std::str::FromStr;
use std::{
    fmt::{Display, Formatter},
};

use crate::lexer::{
    ScanError, Token, keyword_or_ident,
};

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
        while let Some(c) = self.peek() {
            if *c == b'"' || self.is_at_end() {
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
        // if self.cursor - self.start == 1 {
        //     self.cursor += 1;
        // }
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
        let s = unsafe { std::str::from_utf8_unchecked(&self.code[self.start..self.cursor]) };
        Ok(Token::Number(
            f32::from_str(s).map_err(|e| ScanError::NumberNotRecognized)?,
        ))
    }

    #[inline]
    fn advance(&mut self) -> Option<u8> {
        let c = *self.code.get(self.cursor)?;
        self.cursor += 1;
        Some(c)
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
        Ok(keyword_or_ident(s))
    }

    #[inline]
    fn peek(&self) -> Option<&u8> {
        self.code.get(self.cursor)
    }

    fn is_at_end(&self) -> bool {
        self.cursor == self.code.len()
    }

    fn peek_next(&self) -> Option<&u8> {
        self.code.get(self.cursor + 1)
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
        self.start = self.cursor;

        // TODO: Use advance?
        //let c = self.code.get(self.cursor).ok_or(ScanError::End)?;
        let c = self.advance().expect("End of scanning");
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

#[cfg(test)]
mod scanner {
    use super::*;

    #[test]
    fn test_token_no_string() {
        let code = r#"NoString"#;
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Err(ScanError::UnknownToken));
    }

    #[test]
    fn test_token_string() {
        let code = "\"test\"";
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::String, 1, 6)));
    }

    #[test]
    fn test_unmatched_string() {
        let code = "\"";
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
        assert_eq!(scan.scan_token(), Ok((Token::Number(0.), 1, 1)));
    }

    #[test]
    fn test_number_lenght() {
        let code = r#"123456789"#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Number(123456789.), 1, 9)));
    }

    #[test]
    fn test_number_end_fractionnal() {
        let code = r#"123456789."#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Number(123456789.), 1, 10)));
    }

    #[test]
    fn test_number_fractional_part() {
        let code = r#"12345.6789"#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Number(12345.6789), 1, 10)));
    }

    #[test]
    fn test_scan_tok_real() {
        let code = r#"if else fun "hello", self print and for let nil loop return while"#;

        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::If, 1, 2)));
        assert_eq!(scan.scan_token(), Ok((Token::Else, 1, 4)));
        assert_eq!(scan.scan_token(), Ok((Token::Fun, 1, 3)));
        assert_eq!(scan.scan_token(), Ok((Token::String, 1, 7)));
        assert_eq!(scan.scan_token(), Ok((Token::Comma, 1, 1)));
        assert_eq!(scan.scan_token(), Ok((Token::TokSelf, 1, 4)));
        assert_eq!(scan.scan_token(), Ok((Token::Print, 1, 5)));
        assert_eq!(scan.scan_token(), Ok((Token::And, 1, 3)));
        assert_eq!(scan.scan_token(), Ok((Token::For, 1, 3)));
        assert_eq!(scan.scan_token(), Ok((Token::Let, 1, 3)));
        assert_eq!(scan.scan_token(), Ok((Token::Nil, 1, 3)));
        assert_eq!(scan.scan_token(), Ok((Token::Loop, 1, 4)));
        assert_eq!(scan.scan_token(), Ok((Token::Return, 1, 6)));
        assert_eq!(scan.scan_token(), Ok((Token::While, 1, 5)));
    }

    #[test]
    fn test_scan_math() {
        let code = r#"+ > >= <= < =="#;

        // TODO : Fix start calculation it's broken.
        let mut scan = Scanner::new(code);
        assert_eq!(scan.scan_token(), Ok((Token::Plus, 1, 1)));
        assert_eq!(scan.scan_token(), Ok((Token::Greater, 1, 1)));
        assert_eq!(scan.scan_token(), Ok((Token::GreaterEqual, 1, 2)));
        assert_eq!(scan.scan_token(), Ok((Token::LesserEqual, 1, 2)));
        assert_eq!(scan.scan_token(), Ok((Token::Lesser, 1, 1)));
        assert_eq!(scan.scan_token(), Ok((Token::EqualEqual, 1, 2)));
    }
}
