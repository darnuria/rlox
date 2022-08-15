use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until, take_while},
    character::{
        complete::{alphanumeric0, digit0, digit1, multispace0},
        is_alphabetic, is_digit,
    },
    combinator::value,
    error::ParseError,
    number::complete::float,
    sequence::delimited,
    IResult,
};

use super::Token;

/// TODO: Add lines numbers
/// Eats // or */
#[inline]
pub fn comments_multi_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("/*")(input)?;
    let (input, _) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;
    Ok((input, ()))
}

#[inline]
pub fn comments_single_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = is_not("\n")(input)?;
    Ok((input, ()))
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
pub fn string(input: &[u8]) -> IResult<&[u8], (Token, &[u8])> {
    let (input, _) = tag("\"")(input)?;
    let (input, string_raw) = take_until("\"")(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, (Token::String, string_raw)))
}

#[inline]
pub fn numbers(input: &[u8]) -> IResult<&[u8], (Token, f32)> {
    let (input, number) = float(input)?;
    Ok((input, (Token::Number, number)))
}

/* Delimiters */
fn left_parens(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::LeftParens, tag("("))(input)
}

fn keywords_and_identifiers(input: &[u8]) -> IResult<&[u8], (Token, &[u8])> {
    let underscore_alphadigit = |c| is_alphabetic(c) || is_digit(c) || c == b'_';
    let (input, ident) = take_while(underscore_alphadigit)(input)?;
    let token = crate::keyword_or_ident(ident);
    //.map_err(
    //     //manage userToken
    //     unimplemented!(),
    // );
    Ok((input, (token, ident)))
}

fn right_parens(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::RightParens, tag(")"))(input)
}

fn left_brace(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::LeftBrace, tag("{"))(input)
}
fn right_brace(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::RightBrace, tag("}"))(input)
}

/* Punctuation */
fn semicolon(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Semicolon, tag(";"))(input)
}

fn comma(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Comma, tag(","))(input)
}

fn bang_equal(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::BangEqual, tag("!="))(input)
}

fn equal_equal(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::EqualEqual, tag("=="))(input)
}

fn greater_equal(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::GreaterEqual, tag(">="))(input)
}

fn lesser_equal(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::LesserEqual, tag("<="))(input)
}

/* Operators unary only.
If you add something like += please move them to the alt version !
*/
fn dot(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Dot, tag("."))(input)
}

fn minus(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Minus, tag("-"))(input)
}

fn plus(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Plus, tag("+"))(input)
}

fn slash(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Slash, tag("/"))(input)
}

fn star(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Star, tag("*"))(input)
}

fn lesser(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Lesser, tag("<"))(input)
}

fn greater(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Greater, tag(">"))(input)
}

fn equal(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Equal, tag("="))(input)
}

fn bang(input: &[u8]) -> IResult<&[u8], Token> {
    value(Token::Bang, tag("!"))(input)
}
