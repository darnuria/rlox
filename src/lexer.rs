use nom::{
    bytes::complete::{is_not, tag, take_until},
    character::complete::multispace0,
    combinator::value,
    error::ParseError,
    sequence::delimited,
    IResult,
};

/// TODO: Add lines numbers
/// Eats // or */
pub fn comments_multi_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("/*")(input)?;
    let (input, _) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;
    Ok((input, ()))
}

pub fn comments_single_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = is_not("\n")(input)?;
    Ok((input, ()))
}

/// Taken from nom_recipes
/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn start_end_trailling_spaces<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
