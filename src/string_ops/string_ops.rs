// extern crate panic_halt;
use core::fmt::Write;
use std::string::String;

use cortex_m_rt::{ entry, exception, ExceptionFrame };
use cortex_m_semihosting::hio;

#[exception]
fn HardFault (ef: &ExceptionFrame) -> ! {
  if let Ok(mut hstout) = hio::hstdout() {
    writeln!(hstout, "{:#?}", ef).ok();
  }

  loop {}
}

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
    s.find(search)
  }

  pub fn includes(s: &str, search: &str) -> bool {
    s.contains(search)
  }

  pub fn last_index_of(s: &str, search: &str) -> Option<usize> {
    s.rfind(search)
  }

  pub fn pad_start(s: &str, len: usize, padding: &str) -> String {
    let slen = s.len();
    let diff_len = len as i32 - slen as i32;

    if diff_len < 1 {
      return s.to_string();
    }

    let diff_len = diff_len as f32;
    let padding_len = padding.len() as f32;
    let count = (diff_len / padding_len).ceil();

    [&padding.repeat(count as usize)[0..diff_len as usize], s].concat()
  }

  pub fn pad_end(s: &str, len: usize, padding: &str) -> String {
    let slen = s.len();
    let diff_len = len as i32 - slen as i32;
    if diff_len < 1 {
      return s.to_string();
    }

    let diff_len = diff_len as f32;
    let padding_len = padding.len() as f32;
    let count = (diff_len / padding_len).ceil();

    [s, &padding.repeat(count as usize)[0..diff_len as usize]].concat()
  }

  pub fn replace(s: &str, replaced: &str, replace: &str) -> String {
    s.replace(replaced, replace)
  }

  pub fn slice(s: &str, begin: usize, end: usize) -> String {
    if begin >= s.len() || end <= 0 || end <= begin {
      return String::from("");
    }

    let end = if end > s.len() {
      s.len()
    } else {
      end
    };

    s[begin .. end].to_string()
  }

  pub fn split(s: &str, spliter: &str) -> Vec<String> {
    let vec: Vec<&str> = s.split(spliter).collect::<Vec<&str>>();
    vec.iter().map(|x| x.to_string()).collect()
  }

  pub fn to_lower_case(s: &str) -> String {
    s.to_lowercase()
  }

  pub fn to_upper_case(s: &str) -> String {
    s.to_uppercase()
  }

  pub fn trim(s: &str) -> String {
    s.trim().to_string()
  }

  pub fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
  }

  pub fn trim_end(s: &str) -> String {
    s.trim_end().to_string()
  }
}

#[cfg(test)]
mod string_utils_tests {
  use super::*;

  #[test]
  fn test_char_at() {
    assert_eq!(StringUtils::char_at("hello", 1), Some('e'));
    assert_eq!(StringUtils::char_at("hello", 10), None);
    assert_eq!(StringUtils::char_at("", 0), None);
  }

  #[test]
  fn test_concat() {
    assert_eq!(StringUtils::concat("abc", "def"), "abcdef");
    assert_eq!(StringUtils::concat("", "def"), "def");
    assert_eq!(StringUtils::concat("abc", ""), "abc");
  }

  #[test]
  fn test_starts_with() {
    assert_eq!(StringUtils::starts_with("hello", "h"), true);
    assert_eq!(StringUtils::starts_with("hello", "hello"), true);
    assert_eq!(StringUtils::starts_with("hello", "helloo"), false);
    assert_eq!(StringUtils::starts_with("hello", ""), true);
    assert_eq!(StringUtils::starts_with("hello", "e"), false);
    assert_eq!(StringUtils::starts_with("", "hello"), false);
    assert_eq!(StringUtils::starts_with("", ""), true);
  }

  #[test]
  fn test_ends_with() {
    assert_eq!(StringUtils::ends_with("hello", "o"), true);
    assert_eq!(StringUtils::ends_with("hello", "hello"), true);
    assert_eq!(StringUtils::ends_with("hello", "helloo"), false);
    assert_eq!(StringUtils::ends_with("hello", ""), true);
    assert_eq!(StringUtils::ends_with("hello", "e"), false);
    assert_eq!(StringUtils::ends_with("", "hello"), false);
    assert_eq!(StringUtils::ends_with("", ""), true);
  }

  #[test]
  fn test_index_of () {
    assert_eq!(StringUtils::index_of("", ""), Some(0));
    assert_eq!(StringUtils::index_of("hello", ""), Some(0));
    assert_eq!(StringUtils::index_of("hello", "world"), None);
    assert_eq!(StringUtils::index_of("hello", "hello"), Some(0));
    assert_eq!(StringUtils::index_of("", "hello"), None);
  }

  #[test]
  fn test_includes () {
    assert_eq!(StringUtils::includes("", ""), true);
    assert_eq!(StringUtils::includes("hello", ""), true);
    assert_eq!(StringUtils::includes("hello", "h"), true);
    assert_eq!(StringUtils::includes("hello", "o"), true);
    assert_eq!(StringUtils::includes("hello", "ll"), true);
    assert_eq!(StringUtils::includes("hello", "world"), false);
  }

  #[test]
  fn test_last_index_of () {
    assert_eq!(StringUtils::last_index_of("", ""), Some(0));
    assert_eq!(StringUtils::last_index_of("hello", "o"), Some(4));
    assert_eq!(StringUtils::last_index_of("hello", "l"), Some(3));
    assert_eq!(StringUtils::last_index_of("hello", ""), Some(5));
    assert_eq!(StringUtils::last_index_of("hello", "h"), Some(0));
    assert_eq!(StringUtils::last_index_of("hello", "world"), None);
  }

  #[test]
  fn test_pad_start () {
    assert_eq!(StringUtils::pad_start("hello", 7, "*"), "**hello");
    assert_eq!(StringUtils::pad_start("hello", 0, "*"), "hello");
    assert_eq!(StringUtils::pad_start("hello", 5, "*"), "hello");
    assert_eq!(StringUtils::pad_start("hello", 4, "*"), "hello");
    assert_eq!(StringUtils::pad_start("hello", 13, "hello"), "hellohelhello");
  }

  #[test]
  fn test_pad_end () {
    assert_eq!(StringUtils::pad_end("hello", 7, "*"), "hello**");
    assert_eq!(StringUtils::pad_end("hello", 0, "*"), "hello");
    assert_eq!(StringUtils::pad_end("hello", 0, ""), "hello");
    assert_eq!(StringUtils::pad_end("hello", 5, "*"), "hello");
    assert_eq!(StringUtils::pad_end("hello", 4, "*"), "hello");
    assert_eq!(StringUtils::pad_end("hello", 13, "hello"), "hellohellohel");
  }


  #[test]
  fn test_replace () {
    assert_eq!(StringUtils::replace("", "world", ""), "");
    assert_eq!(StringUtils::replace("hello world", "world", ""), "hello ");
    assert_eq!(StringUtils::replace("hello world", "world", "hello"), "hello hello");
  }

  #[test]
  fn test_slice () {
    assert_eq!(StringUtils::slice("hello", 0, 0), "");
    assert_eq!(StringUtils::slice("hello", 100, 5), "");
    assert_eq!(StringUtils::slice("hello", 0, 4), "hell");
    assert_eq!(StringUtils::slice("hello", 100, 500), "");
    assert_eq!(StringUtils::slice("hello", 0, 5), "hello");
    assert_eq!(StringUtils::slice("hello", 0, 50), "hello");
  }

  #[test]
  fn test_split () {
    assert_eq!(StringUtils::split("hello", "l"), vec!["he", "", "o"]);
    assert_eq!(StringUtils::split("hello", "o"), vec!["hell", ""]);
    assert_eq!(StringUtils::split("hello", "ll"), vec!["he", "o"]);
    assert_eq!(StringUtils::split("hello", "hello"), vec!["", ""]);
    assert_eq!(StringUtils::split("hello", "world"), vec!["hello"]);
  }

  #[test]
  fn test_to_lowercase () {
    assert_eq!(StringUtils::to_lower_case("HELLO"), "hello");
    assert_eq!(StringUtils::to_lower_case("hello"), "hello");
    assert_eq!(StringUtils::to_lower_case(""), "");
  }

  #[test]
  fn test_to_uppercase () {
    assert_eq!(StringUtils::to_upper_case("HELLO"), "HELLO");
    assert_eq!(StringUtils::to_upper_case("hello"), "HELLO");
    assert_eq!(StringUtils::to_upper_case(""), "");
  }

  #[test]
  fn test_trim () {
    assert_eq!(StringUtils::trim("  hello  "), "hello");
    assert_eq!(StringUtils::trim(""), "");
  }

  #[test]
  fn test_trim_start () {
    assert_eq!(StringUtils::trim_start(""), "");
    assert_eq!(StringUtils::trim_start("hello"), "hello");
    assert_eq!(StringUtils::trim_start("  hello  "), "hello  ");

  }

  #[test]
  fn test_trim_end () {
    assert_eq!(StringUtils::trim_end("  hello  "), "  hello");
    assert_eq!(StringUtils::trim_end(""), "");
    assert_eq!(StringUtils::trim_end("hello"), "hello");
  }
}
