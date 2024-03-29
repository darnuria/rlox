// Try https://github.com/fflorent/nom_locate
// for line num + count + pos
use core::fmt;
use std::fmt::{Display, Formatter};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while, take},
    character::{complete::multispace0, is_alphabetic, is_digit},
    combinator::value,
    error::ParseError,
    number::complete::{double},
    sequence::delimited,
    IResult, Offset,
};

// use nom_locate::LocatedSpan;
pub type Span<'a> = nom_locate::LocatedSpan<&'a [u8]>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    /// (
    LeftParens,
    /// )
    RightParens,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftSquare,
    /// ]
    RightSquare,
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
    //    Identifier(&'a [u8]),
    IdentifierNoData,
    /// "[.]*"
    String(&'a [u8]),
    /// 0-9
    Number(f64),

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

    SingleComment,
    MultiComment,

    Unknown,
    EOF,
}

struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
}

// struct TokenPos<'a> {
//     pub position: Span<'a>,
//     pub token: Token
// }

#[derive(Debug, PartialEq, Eq)]
pub enum ScanError {
    UnknownToken,
    UnmatchedString,
    NumberNotRecognized,
    End,
}

enum RloxParseError {
    TooManyConstant,
    UnclosedParens,
    ExpectedExpression,
}

impl From<RloxParseError> for &'static str {
    fn from(err: RloxParseError) -> Self {
        match err {
            RloxParseError::TooManyConstant => "Too many constants in one chunk.",
            RloxParseError::UnclosedParens => "Expect ')' after expression.",
            RloxParseError::ExpectedExpression => "Expect expression.",
        }
    }
}

impl Display for RloxParseError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub fn keyword_or_ident(input: &[u8]) -> Token {
    match input {
        b"" => Token::EOF,
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
        _ => Token::IdentifierNoData,
        //dent => Token::Identifier(ident),
    }
}

pub fn scan_token(input: Span) -> IResult<Span, Token> {
    let inner = alt((
        comments_multi_line,
        comments_single_line,
        numbers,
        string, // need change in tokens.
        operators,
        delimiters,
        keywords_and_identifiers,
        unknown,
    ));

    delimited(multispace0, inner, multispace0)(input)
}

/// TODO: Add lines numbers
/// Eats // or */
#[inline]
pub fn comments_multi_line(input: Span) -> IResult<Span, Token> {
    let (input, _) = tag("/*")(input)?;
    let (input, _) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;
    Ok((input, Token::MultiComment))
}

#[inline]
pub fn comments_single_line(input: Span) -> IResult<Span, Token> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = is_not("\n")(input)?;
    Ok((input, Token::SingleComment))
}

/// Taken from nom_recipes
/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
#[inline]
pub fn start_end_trailling_spaces<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// TODO count lines.
#[inline]
pub fn string(input: Span) -> IResult<Span, Token> {
    let (input, _) = tag("\"")(input)?;
    let (input, string_raw) = take_until("\"")(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Token::String(&string_raw)))
}

#[inline]
pub fn numbers(input: Span) -> IResult<Span, Token> {
    let (input, number) = double(input)?;
    Ok((input, Token::Number(number)))
}

fn keywords_and_identifiers(input: Span) -> IResult<Span, Token> {
    let underscore_alphadigit = |c| is_alphabetic(c) || is_digit(c) || c == b'_';
    let (input, ident) = take_while(underscore_alphadigit)(input)?;
    let token = keyword_or_ident(ident.fragment());
    //.map_err(
    //     //manage userToken
    //     unimplemented!(),
    // );
    Ok((input, token))
}

/* Delimiters */
fn delimiters(input: Span) -> IResult<Span, Token> {
    alt((
        left_parens,
        right_parens,
        right_square,
        left_square,
        right_brace,
        left_brace,
        right_brace,
    ))(input)
}

fn left_parens(input: Span) -> IResult<Span, Token> {
    value(Token::LeftParens, tag("("))(input)
}

fn right_parens(input: Span) -> IResult<Span, Token> {
    value(Token::RightParens, tag(")"))(input)
}

fn left_brace(input: Span) -> IResult<Span, Token> {
    value(Token::LeftBrace, tag("{"))(input)
}

fn right_brace(input: Span) -> IResult<Span, Token> {
    value(Token::RightBrace, tag("}"))(input)
}

fn left_square(input: Span) -> IResult<Span, Token> {
    value(Token::LeftSquare, tag("["))(input)
}

fn right_square(input: Span) -> IResult<Span, Token> {
    value(Token::RightSquare, tag("]"))(input)
}

/* operators */
#[inline]
fn operators(input: Span) -> IResult<Span, Token> {
    alt((
        semicolon,
        comma,
        star,
        lesser_equal,
        greater_equal,
        equal_equal,
        bang_equal,
        greater,
        slash,
        minus,
        lesser,
        plus,
        dot,
        bang,
        equal,
    ))(input)
}

fn semicolon(input: Span) -> IResult<Span, Token> {
    value(Token::Semicolon, tag(";"))(input)
}

fn comma(input: Span) -> IResult<Span, Token> {
    value(Token::Comma, tag(","))(input)
}

fn bang_equal(input: Span) -> IResult<Span, Token> {
    value(Token::BangEqual, tag("!="))(input)
}

fn equal_equal(input: Span) -> IResult<Span, Token> {
    value(Token::EqualEqual, tag("=="))(input)
}

fn greater_equal(input: Span) -> IResult<Span, Token> {
    value(Token::GreaterEqual, tag(">="))(input)
}

fn lesser_equal(input: Span) -> IResult<Span, Token> {
    value(Token::LesserEqual, tag("<="))(input)
}

/* Operators unary only.
If you add something like += please move them to the alt version !
*/
fn dot(input: Span) -> IResult<Span, Token> {
    value(Token::Dot, tag("."))(input)
}

fn minus(input: Span) -> IResult<Span, Token> {
    value(Token::Minus, tag("-"))(input)
}

fn plus(input: Span) -> IResult<Span, Token> {
    value(Token::Plus, tag("+"))(input)
}

fn slash(input: Span) -> IResult<Span, Token> {
    value(Token::Slash, tag("/"))(input)
}

fn star(input: Span) -> IResult<Span, Token> {
    value(Token::Star, tag("*"))(input)
}

fn lesser(input: Span) -> IResult<Span, Token> {
    value(Token::Lesser, tag("<"))(input)
}

fn greater(input: Span) -> IResult<Span, Token> {
    value(Token::Greater, tag(">"))(input)
}

fn equal(input: Span) -> IResult<Span, Token> {
    value(Token::Equal, tag("="))(input)
}

fn bang(input: Span) -> IResult<Span, Token> {
    value(Token::Bang, tag("!"))(input)
}

fn unknown(input: Span) -> IResult<Span, Token> {
    nom::combinator::map(take(1usize), |_| Token::Unknown)(input)
}

#[cfg(test)]
mod test_lexer {
    use super::*;
    //    use nom_locate::LocatedSpan;

    fn assert_token_span<'a>(
        code: Span<'a>,
        expected: Token<'a>,
        offset: usize,
        line: u32,
    ) -> IResult<Span<'a>, Token<'a>> {
        let span = code;
        let ret = scan_token(span);
        let (span, tok) = ret?;
        assert_eq!(tok, expected, "Token didn't match.");
        // todo debug space eating.
        //assert_eq!(span.location_offset(), offset, "Offset not equals.");
        assert_eq!(span.location_line(), line, "Line count different.");
        //assert_eq!(span.fragment(), &&code[offset..]);
        Ok((span, tok))
    }

    #[test]
    fn number_sigle() {
        let code = br#"0"#;
        assert_token_span(Span::new(code), Token::Number(0.), 1, 1).unwrap();
    }

    #[test]
    fn test_number_lenght() {
        let code = br#"123456789"#;
        assert_token_span(Span::new(code), Token::Number(123456789.), 9, 1).unwrap();
    }

    #[test]
    fn test_number_end_fractionnal() {
        let code = br#"123456789."#;
        assert_token_span(Span::new(code), Token::Number(123456789.), 10, 1).unwrap();
    }

    #[test]
    fn test_number_fractional_part() {
        let code = br#"12345.6789"#;
        assert_token_span(Span::new(code), Token::Number(12345.6789), 10, 1).unwrap();
    }

    #[test]
    fn test_keyword_alone() {
        let code = b"if";
        assert_token_span(Span::new(code), Token::If, 2, 1).unwrap();
    }

    #[test]
    fn test_scan_tok_real() {
        let code = br#"if else fun, self print and for let nil loop return while"#;
        let expected = [
            (Token::If, 3, 1),
            // need rework
            // (code, Token::String, 7, 1);
            (Token::Else, 7, 1),
            (Token::Fun, 4, 1),
            (Token::Comma, 20, 1),
            (Token::TokSelf, 21, 1),
            (Token::Print, 26, 1),
            (Token::And, 32, 1),
            (Token::For, 36, 1),
            (Token::Let, 41, 1),
            (Token::Nil, 44, 1),
            (Token::Loop, 48, 1),
            (Token::Return, 53, 1),
            (Token::While, 62, 1),
        ];

        let mut code = Span::new(code);
        for (token, offset, line) in expected {
            code = assert_token_span(code, token, offset, line)
                .expect("Should have been parsed.")
                .0;
        }
    }

    #[test]
    fn test_scan_math() {
        let code = br#"+ > >= <= < =="#;
        let expected = [
            (Token::Plus, 1, 1),
            (Token::Greater, 1, 1),
            (Token::GreaterEqual, 1, 1),
            (Token::LesserEqual, 1, 1),
            (Token::Lesser, 1, 1),
            (Token::EqualEqual, 1, 1),
        ];

        let mut code = Span::new(code);
        for (token, offset, line) in expected {
            code = assert_token_span(code, token, offset, line)
                .expect("Should have been parsed.")
                .0;
        }
    }

    #[test]
    fn test_token_no_string() {
        let code = br#"NoString"#;
        let code = Span::new(code);
        let ret = scan_token(code);
        assert!(string(code).is_err());
        // assert_eq!(ret, Err(ScanError::UnknownToken));
    }

    #[test]
    fn test_token_string() {
        let code = b"\"test\"";
        let code = Span::new(code);
        assert_token_span(code, Token::String(b"test"), 6, 1).expect("Should have been parsed.");
    }

    #[test]
    fn test_unmatched_string() {
        let code = b"\"";
        let code = Span::new(code);
        let ret = scan_token(code);
        assert!(string(code).is_err());
        // assert_eq!(ret, Err(ScanError::UnmatchedString));
    }

    #[test]
    fn test_empty_string() {
        let code = br#""""#;

        let code = Span::new(code);
        assert_token_span(code, Token::String(b""), 2, 1).expect("Should have been parsed.");
    }

    #[test]
    fn test_unmatched_char() {
        let code = br#"e""#;

        let code = Span::new(code);
        // assert_token_span(code, Err(ScanError::UnknownToken));
        assert!(string(code).is_err());
    }
}
