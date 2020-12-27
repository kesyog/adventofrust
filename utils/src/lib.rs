use num_integer::Integer;
use regex::Regex;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::string::ToString;

pub mod nom_parsers;

/// Return an iterator over the lines in the given file.
///
/// Performs an extra copy in order to return an owned value. Do it yourself if you really care
/// about speed
pub fn get_lines_from_file(path: &str) -> Vec<String> {
    let data = fs::read_to_string(Path::new(path)).unwrap();
    data.split_terminator('\n')
        .map(ToString::to_string)
        .collect()
}

pub fn trim_and_split(string: &str, split: &str) -> Vec<String> {
    string
        .trim()
        .split_terminator(split)
        .map(|s| s.trim().to_string())
        .collect()
}

/// Return a list of all integers in a string
pub fn find_all_integers<T>(string: &str) -> Vec<T>
where
    T: Integer + FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(string)
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
