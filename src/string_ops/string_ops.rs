use std::string::String;

pub struct StringUtils {}


#[allow(dead_code, unused_variables)]
impl StringUtils {
  pub fn char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
  }

  pub fn concat(s: &str, ss: &str) -> String {
    [s, ss].concat()
  }

  pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
  }

  pub fn ends_with(s: &str, tail: &str) -> bool {
    s.ends_with(tail)
  }

  pub fn index_of(s: &str, search: &str) -> Option<usize> {
    None
  }

  pub fn includes(s: &str, search: &str) -> bool {
    true
  }

  pub fn last_index_of(s: &str, search: &str) -> Option<usize> {
    None
  }

  pub fn pad_start(s: &str, pad_string: &str) -> String {
    String::from("")
  }

  pub fn pad_end(s: &str, pad_string: &str) -> String {
    String::from("")
  }

  pub fn replace(s: &str, replaced: &str, replace: &str) -> String {
    String::from("")
  }

  pub fn slice(s: &str, begin: usize, end: usize) -> String {
    String::from("")
  }

  pub fn split(s: &str, spliter: &str) -> Vec<String> {
    vec![]
  }

  pub fn to_lower_case(s: &str) -> String {
    String::from("")
  }

  pub fn to_upper_case(s: &str) -> String {
    String::from("")
  }

  pub fn strim(s: &str) -> String {
    String::from("")
  }

  pub fn strim_start(s: &str) -> String {
    String::from("")
  }

  pub fn strim_end(s: &str) -> String {
    String::from("")
  }

}

#[cfg(test)]
mod string_utils_tests {
  use super::*;

  #[test]
  fn test_char_at () {
    assert_eq!(StringUtils::char_at("hello", 1), Some('e'));
    assert_eq!(StringUtils::char_at("hello", 10), None);
    assert_eq!(StringUtils::char_at("", 0), None);
  }

  #[test]
  fn test_concat () {
    assert_eq!(StringUtils::concat("abc", "def"), "abcdef");
    assert_eq!(StringUtils::concat("", "def"), "def");
    assert_eq!(StringUtils::concat("abc", ""), "abc");
  }

  #[test]
  fn test_starts_with () {
    assert_eq!(StringUtils::starts_with("hello", "h"), true);
    assert_eq!(StringUtils::starts_with("hello", "hello"), true);
    assert_eq!(StringUtils::starts_with("hello", "helloo"), false);
    assert_eq!(StringUtils::starts_with("hello", ""), true);
    assert_eq!(StringUtils::starts_with("hello", "e"), false);
    assert_eq!(StringUtils::starts_with("", "hello"), false);
    assert_eq!(StringUtils::starts_with("", ""), true);

  }

  #[test]
  fn test_ends_with () {
    assert_eq!(StringUtils::ends_with("hello", "o"), true);
    assert_eq!(StringUtils::ends_with("hello", "hello"), true);
    assert_eq!(StringUtils::ends_with("hello", "helloo"), false);
    assert_eq!(StringUtils::ends_with("hello", ""), true);
    assert_eq!(StringUtils::ends_with("hello", "e"), false);
    assert_eq!(StringUtils::ends_with("", "hello"), false);
    assert_eq!(StringUtils::ends_with("", ""), true);
  }

}
