use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::IResult;

#[macro_export]
macro_rules! run {
    ($day:tt) => {{
        use aoc2022::$day::run;
        let input = include_str!(std::concat!("../../inputs/", std::stringify!($day), ".txt"));
        let (part_1, part_2) = run(input)?;

        println!("Part 1: {}", part_1);
        println!("Part 2: {}", part_2);

        Ok(())
    }};
}

// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
