use std::fmt::Debug;
use std::any::Any;
use std::iter::FromIterator;
use std::iter::Map;

fn concat<T: Clone + Copy> (arr1: &Vec<T>, arr2: &Vec<T>) -> Vec<T> {
  [arr1.as_slice(), arr2.as_slice()].concat()
}

// 返回一个新的Array Iterator对象，该对象包含数组中的每个索引的键和值
fn entries<T: Sized> (arr: &Vec<T>) -> impl Iterator<Item=(usize, &T)> {
  let len = arr.len();
  (0..len).zip(arr.into_iter()).into_iter()
}

fn every<T: Sized> (arr: &Vec<T>, f: fn(&T) -> bool) -> bool {
  arr.into_iter().all(f)
}

fn fill<T: Sized + Copy> (arr: &mut Vec<T>, val: T, start: usize, end: usize) {
  let len = arr.len();
  let start = 0.max(start);
  let end = len.min(end);
  if start > end {
    panic!("start must less equal than end")
  }

  for i in start..end {
    arr[i] = val;
  }
}

fn filter<T: Sized + Copy> (arr: &Vec<T>, f: fn(&&T) -> bool) -> Vec<T> {
  let iter = arr.into_iter().filter(f);
  let mut result: Vec<T> = vec![];

  for i in iter {
    result.push(*i);
  }

   result
}

fn find<'a, T: std::cmp::Ord> (arr: &'a Vec<T>, val: &T) -> Option<&'a T> {
  arr.into_iter().find(|&v| {
    v == val
  })
  // let found = arr.binary_search(val);
  // if found.is_ok() {
  //   Some(arr.get(found.unwrap()).unwrap())
  // } else {
  //   None
  // }
}

fn find_index<T: IntoIterator + Copy> (arr: &T, val: T::Item) -> Option<usize>
where
  T::Item: std::cmp::Ord {
  arr.into_iter().position(|v| v == val)
}

fn flat<T: IntoIterator> (arr: Vec<T>) -> Vec<T::Item>
where
  T::Item: IntoIterator {
  arr.into_iter().flatten().collect::<Vec<_>>()
}

// like map + flat
fn flat_map<U: IntoIterator, F> (arr: U, f: F) -> Vec<U::Item>
where
  U: IntoIterator,
  F: FnMut(U::Item) -> U {
  arr.into_iter().flat_map(f).collect::<Vec<U::Item>>()
}

fn for_each<U: IntoIterator, F: FnMut(U::Item)> (arr: U, f: F) {
  arr.into_iter().for_each(f)
}

fn includes<U: IntoIterator + Copy> (arr: &U, val: U::Item) -> bool
where
U::Item: std::cmp::Ord {
  find_index(arr, val).is_some()
}

fn index_of<T: IntoIterator + Copy> (arr: &T, val: T::Item) -> Option<usize>
where
  T::Item: std::cmp::Ord {
  find_index(arr, val)
}

fn join (arr: Vec<&str>, sep: &str) -> String {
  arr.join(sep)
}

fn keys<U: IntoIterator> (arr: U) -> impl Iterator<Item=usize> {
  arr.into_iter().enumerate().map(|arg| arg.0)
}

fn last_index_of <T: ExactSizeIterator + DoubleEndedIterator + Copy> (arr: &mut T, val: T::Item) -> Option<usize>
where
  T::Item: std::cmp::Ord {
  arr.rposition(|v| v == val)
}

fn map<U: IntoIterator, F: FnMut(U::Item)> (arr: U, f: F) -> Map<U::IntoIter, F> {
  arr.into_iter().map(f)
}


fn pop<T> (arr: &mut Vec<T>) -> Option<T> {
  arr.pop()
}

fn push<T> (arr: &mut Vec<T>, val: T) {
  arr.push(val)
}

fn reduce<U: IntoIterator, Acc, F: FnMut(Acc, U::Item) -> Acc> (arr: U, init: Acc, f: F) -> Acc {
  arr.into_iter().fold(init, f)
}

fn reduce_right<U: DoubleEndedIterator, Acc, F: FnMut(Acc, U::Item) -> Acc> (arr: U, init: Acc, f: F) -> Acc {
  let rev_arr = arr.rev();
  rev_arr.fold(init, f)
}

fn reverse<U: DoubleEndedIterator> (arr: U) -> Vec<U::Item> {
  arr.rev().collect()
}

fn shift<T: Copy> (arr: &mut Vec<T>) -> Option<T> {
  if let Some(&first) = arr.first() {
    arr.drain(0..1);
    Some(first)
  } else {
    None
  }

}

fn slice<T: Copy> (arr: &Vec<T>, start: usize, end: usize) -> Vec<T> {
  let mut ret = vec![];

  for i in start..end {
    ret.push(arr[i])
  }
  return ret;
}

fn some<T, F: Fn(&T) -> bool> (arr: &Vec<T>, f: F) -> bool {
  arr.iter().any(f)
}

fn sort<T: std::cmp::Ord, F: FnMut(&T, &T) -> std::cmp::Ordering> (arr: &mut Vec<T>, f: F) {
  arr.sort_by(f)
}

fn splice<T> (arr: &mut Vec<T>, remove_index: usize, insert_element: T) {
  arr.remove(remove_index);
  arr.insert(remove_index, insert_element)
}

fn unshift<T> (arr: &mut Vec<T>, insert_element: T) {
  arr.insert(0, insert_element);
}

fn values<T> (arr: &Vec<T>) -> impl Iterator<Item=&T> {
  arr.iter().enumerate().map(|arg| arg.1)
}


#[cfg(test)]
mod array_ops_tests {
  use super::*;

  #[test]
  fn test_concat () {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 5, 6];
    assert_eq!(concat(&arr1, &arr2), vec![1, 2, 3, 4, 5, 6]);
  }

  #[test]
  fn test_entries () {
    let arr = vec![1, 2, 3];
    let mut _entries = entries(&arr);
    assert_eq!(_entries.next(), Some((0, &1)));
    assert_eq!(_entries.next(), Some((1, &2)));
    assert_eq!(_entries.next(), Some((2, &3)));
  }
}
