use num_integer::Integer;
use regex::Regex;
use std::fmt::Debug;
use std::str::{self, FromStr};

pub mod nom_parsers;

/// Return an iterator over the given file split by the given pattern. All leading and trailing
/// whitespace are trimmed from the start and end of every line.
#[inline]
pub fn trim_and_split<'a>(string: &'a str, split: &'a str) -> impl Iterator<Item = &'a str> {
    string.trim().split_terminator(split).map(str::trim)
}

#[inline]
pub fn split_and_parse<'a, T>(string: &'a str, split: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    trim_and_split(string, split).map(|i| i.parse::<T>().unwrap())
}

/// Return a list of all integers in a string
#[inline]
pub fn find_all_integers<T>(string: impl AsRef<str>) -> Vec<T>
where
    T: Integer + FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(string.as_ref())
        .map(|m| m.as_str().parse::<T>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_integers() {
        assert!(find_all_integers::<i32>("").is_empty());
        assert!(find_all_integers::<i32>("lorem ipsum").is_empty());

        let nums = find_all_integers("123 456 apples -789");
        assert_eq!(3, nums.len());
        for (actual, expected) in nums.into_iter().zip(vec![123_i64, 456, -789]) {
            assert_eq!(expected, actual);
        }
    }
}
