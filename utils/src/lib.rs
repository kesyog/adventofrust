use num_integer::Integer;
use regex::Regex;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Return an iterator over the lines in the given file.
///
/// Performs an extra copy in order to return an owned value. Do it yourself if you really care
/// about speed
pub fn get_lines_from_file(path: &str) -> impl Iterator<Item = String> {
    let data = fs::read_to_string(Path::new(path)).unwrap();
    let res: Vec<String> = data.split_terminator('\n').map(|s| s.to_string()).collect();
    res.into_iter()
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
