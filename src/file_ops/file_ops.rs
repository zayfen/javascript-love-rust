// file ops
use std::fs::File;
use std::io::prelude::*;
use std::io::{ Error as IOError, BufReader, BufWriter };
use std::path::Path;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::ffi::OsStr;


pub fn create_truncate_file (file_path: &str) -> Result<File, IOError> {
  File::create(file_path)
}



pub fn read_file (file_path: &str) -> Result<String, IOError> {
  
  let mut data = String::from("");

  let f = File::open(file_path)?;

  let mut buf_reader = BufReader::new(f);
  buf_reader.read_to_string(&mut data)?;

  return Ok(data);
}


pub fn write_file (file_path: &str, data: &str) -> Result<usize, IOError> {

  let f = File::open(file_path)?;
  let mut buf_writer = BufWriter::new(f);
  buf_writer.write_all(data.as_bytes())?;
  buf_writer.flush()?;
  Ok(data.len())
}


pub fn delete_file<T: AsRef<Path>> (file_path: T) -> Result<(), IOError> {
  std::fs::remove_file(file_path)
}


pub fn create_dir<T: AsRef<Path>> (dir_path: T) -> Result<(), IOError> {
  std::fs::create_dir(dir_path)
}


pub fn list_dir<T: AsRef<Path>> (dir_path: T) -> Result<Vec<std::path::PathBuf>, IOError> {
  let dir_iter = std::fs::read_dir(dir_path)?;

  let mut list = vec![];
  
  let mut res = dir_iter.map(|entry| {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.is_dir() {
      let mut sub_dirs = list_dir(path).unwrap();
      list.append(&mut sub_dirs);
    } else {
      list.push(path);
    }
  });

  // execute map, for map is lazy
  // so list will fill values
  while res.next().is_some() {
  }

  Ok(list)
}


pub fn remove_dir<T: AsRef<Path>> (dir_path: T) -> Result<(), IOError> {
  std::fs::remove_dir_all(dir_path)
}


// TODO: change file permission mode
pub fn chmod<T: AsRef<Path>> (path: T, mode: u32) -> Option<bool> {
  let file = File::open(path).unwrap();

  return match file.metadata() {
    Ok(metadata) => {
      let mut permissions = metadata.permissions();
      permissions.set_mode(mode);

      return match file.set_permissions(permissions) {
        Ok(_) => Some(true),
        Err(_) => None
      }

    },
    _ => {
      return None
    }
  }
}


// TODO: path join, resolve
pub fn path_join<T: AsRef<OsStr> + ?Sized> (a: &T, b: &T) -> PathBuf {
  let path_a  = Path::new(a);
  let path_b = Path::new(b);
  path_a.join(path_b)
}


#[cfg(test)]
mod file_ops_tests {
  use super::*;
  
  #[test]
  pub fn test_list_dir () {
    let paths = list_dir("/home/zayfen/Github/javascript-love-rust/src/").unwrap();
    println!("{:?}", paths);
    for p in paths {
      dbg!(p.as_os_str());
    }
  }

  #[test]
  pub fn test_path_join () {
    let joined_path = path_join("/home/zayfen/okok/..", "videos");
    assert_eq!("/home/zayfen/okok/../videos", joined_path.as_os_str());
  }
}
