use std::fmt::Debug;

fn concat<T: Clone + Copy> (arr1: &Vec<T>, arr2: &Vec<T>) -> Vec<T> {
  [arr1.as_slice(), arr2.as_slice()].concat()
}

// 返回一个新的Array Iterator对象，该对象包含数组中的每个索引的键和值
fn entries<T: Sized> (arr: &Vec<T>) -> impl Iterator<Item=(usize, &T)> {
  let len = arr.len();
  (0..len).zip(arr.into_iter()).into_iter()
}

fn every () {
  
}

fn fill () {
  
}

fn filter () {
  
}

fn find () {
  
}

fn find_index () {
  
}

fn flat () {
  
}

fn flat_map () {
  
}

fn for_each () {
  
}

fn includes () {
  
}

fn index_of () {
  
}

fn join () {
  
}


fn keys () {
  
}

fn last_index_of () {
  
}

fn map () {
  
}


fn pop () {
  
}

fn push () {
  
}

fn reduce () {
  
}

fn reduce_right () {
  
}

fn reverse () {
  
}

fn shift () {
  
}

fn slice () {
  
}

fn some () {
  
}

fn sort () {
  
}

fn splice () {
  
}

fn unshift () {
  
}

fn values () {
  
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
