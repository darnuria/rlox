// Try https://github.com/fflorent/nom_locate
// for line num + count + pos

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while},
    character::{complete::multispace0, is_alphabetic, is_digit},
    combinator::value,
    error::ParseError,
    number::complete::float,
    sequence::delimited,
    IResult, Offset,
};

// use nom_locate::LocatedSpan;
type Span<'a> = nom_locate::LocatedSpan<&'a [u8]>;
use super::Token;

// struct TokenPos<'a> {
//     pub position: Span<'a>,
//     pub token: Token
// }

pub fn scan_token(input: Span) -> IResult<Span, Token> {
    let inner = alt((
        comments_multi_line,
        comments_single_line,
        numbers,
        //string, need change in tokens.
        operators,
        delimiters,
        keywords_and_identifiers,
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
    Ok((input, Token::String))
}

#[inline]
pub fn numbers(input: Span) -> IResult<Span, Token> {
    let (input, number) = float(input)?;
    Ok((input, Token::Number(number)))
}

fn keywords_and_identifiers(input: Span) -> IResult<Span, Token> {
    let underscore_alphadigit = |c| is_alphabetic(c) || is_digit(c) || c == b'_';
    let (input, ident) = take_while(underscore_alphadigit)(input)?;
    let token = crate::keyword_or_ident(ident.fragment());
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
        equal,
        comma,
        star,
        greater,
        slash,
        minus,
        lesser,
        plus,
        dot,
        bang,
        equal,
        lesser_equal,
        greater_equal,
        equal_equal,
        bang_equal,
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

#[cfg(test)]
mod test_lexer {
    use super::*;
    //    use nom_locate::LocatedSpan;

    fn assert_token_span(code: Span, expected: Token, offset: usize, line: u32) -> Span {
        let span = code;
        let ret = scan_token(span);
        let (span, tok) = ret.unwrap();
        assert_eq!(tok, expected, "Token didn't match.");
        // todo debug space eating.
        //assert_eq!(span.location_offset(), offset, "Offset not equals.");
        assert_eq!(span.location_line(), line, "Line count different.");
        //assert_eq!(span.fragment(), &&code[offset..]);
        span
    }

    #[test]
    fn number_sigle() {
        let code = br#"0"#;
        assert_token_span(Span::new(code), Token::Number(0.), 1, 1);
    }

    #[test]
    fn test_number_lenght() {
        let code = br#"123456789"#;
        assert_token_span(Span::new(code), Token::Number(123456789.), 9, 1);
    }

    #[test]
    fn test_number_end_fractionnal() {
        let code = br#"123456789."#;
        assert_token_span(Span::new(code), Token::Number(123456789.), 10, 1);
    }

    #[test]
    fn test_number_fractional_part() {
        let code = br#"12345.6789"#;
        assert_token_span(Span::new(code), Token::Number(12345.6789), 10, 1);
    }

    #[test]
    fn test_keyword_alone() {
        let code = b"if";
        assert_token_span(Span::new(code), Token::If, 2, 1);
    }

    #[test]
    fn test_scan_tok_real() {
        let code = br#"if else fun, self print and for let nil loop return while"#;

        let code = Span::new(code);
        let code = assert_token_span(code, Token::If, 3, 1);
        println!("{:?}", std::str::from_utf8(code.fragment()));
        let code = assert_token_span(code, Token::Else, 7, 1);
        let code = assert_token_span(code, Token::Fun, 4, 1);
        // need rework
        // let code = assert_token_span(code, Token::String, 7, 1);
        let code = assert_token_span(code, Token::Comma, 20, 1);
        let code = assert_token_span(code, Token::TokSelf, 21, 1);
        let code = assert_token_span(code, Token::Print, 26, 1);
        let code = assert_token_span(code, Token::And, 32, 1);
        let code = assert_token_span(code, Token::For, 36, 1);
        let code = assert_token_span(code, Token::Let, 41, 1);
        let code = assert_token_span(code, Token::Nil, 44, 1);
        let code = assert_token_span(code, Token::Loop, 48, 1);
        let code = assert_token_span(code, Token::Return, 53, 1);
        let _ = assert_token_span(code, Token::While, 62, 1);
    }

    #[test]
    fn test_scan_math() {
        let code = br#"+ > >= <= < =="#;
        let code = Span::new(code);

        let code = assert_token_span(code, Token::Plus, 1, 1);
        let code = assert_token_span(code, Token::Greater, 1, 1);
        let code = assert_token_span(code, Token::GreaterEqual, 1, 2);
        let code = assert_token_span(code, Token::LesserEqual, 1, 2);
        let code = assert_token_span(code, Token::Lesser, 1, 1);
        let code = assert_token_span(code, Token::EqualEqual, 1, 2);
    }
}
