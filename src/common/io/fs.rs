#![allow(dead_code)]
#![allow(unused_must_use)]

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Error;

#[inline]
pub fn rm<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    fs::remove_file(path)
}

pub fn ls<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Error> {
    let mut ret = Vec::new();
    let paths = fs::read_dir(path)?;
    for path in paths {
        let path_buf = path.unwrap().path();
        let filename = path_buf.to_str().unwrap().to_owned();
        ret.push(filename);
    }
    Ok(ret)
}

#[inline]
pub fn mkdir<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    fs::create_dir_all(path)
}

#[inline]
pub fn rmr<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    fs::remove_dir_all(path)
}

#[inline]
pub fn touch<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    File::create(path)?;
    Ok(())
}

#[inline]
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

#[inline]
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

pub fn create_path<P: AsRef<Path>>(components: &Vec<P>) -> String {
    let mut path_buf = PathBuf::new();
    for c in components.iter() {
        path_buf.push(c);
    }
    path_buf.to_str().unwrap().to_owned()
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DIR: &str = "test_dir";

    #[test]
    fn test_basic_op() {
//        rmr(TEST_DIR).unwrap();
//        let test_dir = TEST_DIR;
//        assert!(!exists(test_dir));
//        mkdir(test_dir);
//        assert!(exists(test_dir) && is_dir(test_dir));
//
//        let test_path1 = "test_dir/aaa";
//        let test_path2 = create_path(&vec![test_dir, "aaa"]);
//        assert_eq!(test_path1, test_path2.as_str());
//
//        touch(test_path1);
//        assert!(exists(test_path1));
//
//        let test_path3 = create_path(&vec![test_dir, "bbb"]);
//        touch(test_path3.as_str());
//        assert!(exists(test_path3.as_str()));
//
//        let files = ls(test_dir).unwrap();
//        assert_eq!(files.len(), 2);
//        assert!(files.contains(&test_path2));
//        assert!(files.contains(&test_path3));
//
//
//        rmr(test_dir);
//        assert!(!exists(test_dir));

    }


}