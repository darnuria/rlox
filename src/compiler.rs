use crate::{lexer::Token, Chunk, Opcode};

impl<'a> Token<'a> {
    fn token_to_opcode(&self, chunk: &mut Chunk) {
        match self {
            Token::LeftParens => todo!(),
            Token::RightParens => todo!(),
            Token::LeftBrace => todo!(),
            Token::RightBrace => todo!(),
            Token::LeftSquare => todo!(),
            Token::RightSquare => todo!(),
            Token::Comma => todo!(),
            Token::Dot => todo!(),
            Token::Minus => todo!(),
            Token::Plus => todo!(),
            Token::Semicolon => todo!(),
            Token::Slash => todo!(),
            Token::Star => todo!(),
            Token::Bang => todo!(),
            Token::BangEqual => todo!(),
            Token::Equal => todo!(),
            Token::EqualEqual => todo!(),
            Token::Greater => todo!(),
            Token::GreaterEqual => todo!(),
            Token::Lesser => todo!(),
            Token::LesserEqual => todo!(),
            Token::IdentifierNoData => todo!(),
            Token::String(_) => todo!(),
            Token::Number(n) => {
                let idx_in_code = chunk.write_value(*n);
                chunk.write_opcode(Opcode::Constant(idx_in_code), 42)
            }
            Token::And => todo!(),
            Token::Or => todo!(),
            Token::Struct => todo!(),
            Token::If => todo!(),
            Token::Else => todo!(),
            Token::True => todo!(),
            Token::False => todo!(),
            Token::Fun => todo!(),
            Token::Loop => todo!(),
            Token::While => todo!(),
            Token::For => todo!(),
            Token::Nil => todo!(),
            Token::Return => todo!(),
            Token::Let => todo!(),
            Token::Print => todo!(),
            Token::TokSelf => todo!(),
            Token::SingleComment => todo!(),
            Token::MultiComment => todo!(),
            Token::Unknown => todo!(),
            Token::EOF => todo!(),
        }
    }
}
