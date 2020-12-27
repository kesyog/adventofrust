//! Nom parser combinator functions
//!
//! First foray into nom. Everything is gross.

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::{separated_pair, tuple},
    IResult,
};

pub fn signed_int(input: &str) -> IResult<&str, i32> {
    let signed_integer_parser = recognize(tuple((opt(tag("-")), digit1)));
    map_res(signed_integer_parser, str::parse)(input)
}

pub fn coordinates(input: &str) -> IResult<&str, (i32, i32)> {
    Ok(separated_pair(signed_int, tag(", "), signed_int)(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coordinates() {
        assert_eq!(coordinates("12, 2").unwrap(), ("", (12, 2)));
        assert_eq!(coordinates("-1, 404").unwrap(), ("", (-1, 404)));
        assert_eq!(coordinates("-2, -21").unwrap(), ("", (-2, -21)));
        assert_eq!(coordinates("0, -1").unwrap(), ("", (0, -1)));
    }
}
