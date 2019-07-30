#![allow(dead_code)]
#![allow(unused_must_use)]
use std::fs;
use std::fs::File;
use std::path::{Path};
use std::io::prelude::*;
use std::io::Error;
use std::slice;

use super::*;

pub struct FileChannel {
    f: File,
    filename: String,
}

impl FileChannel {
    pub fn create<P: AsRef<Path>>(p: P) -> Self {
        let f = File::create(p.as_ref()).unwrap();
        FileChannel {
            f,
            filename: p.as_ref().to_str().unwrap().to_owned(),
        }
    }

    pub fn open<P: AsRef<Path>>(p: P) -> Self {
        let f = File::open(p.as_ref()).unwrap();
        FileChannel {
            f,
            filename: p.as_ref().to_str().unwrap().to_owned(),
        }
    }

    /// after write, the buf is cleared
    pub fn write(&mut self, buf: &mut ByteBuffer) -> Result<(), Error> {
        let ptr = buf.array().as_mut_ptr();
        let data = unsafe {
            slice::from_raw_parts_mut(ptr.offset(buf.position() as isize), buf.limit() - buf.position())
        };
        self.f.write_all(data)?;
        buf.clear();
        Ok(())
    }

    /// read at most bytes buf remaining
    pub fn read(&mut self, buf: &mut ByteBuffer) -> Result<usize, Error> {
        let ptr = buf.array().as_mut_ptr();
        let position = buf.position();
        let data = unsafe {
            slice::from_raw_parts_mut(ptr.offset(position as isize), buf.limit() - position)
        };
        let size = self.f.read(data)?;
        buf.set_position(position + size);
        Ok(size)
    }

    #[inline]
    pub fn len(&self) -> u64 {
        fs::metadata(self.filename.as_str()).unwrap().len()
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::fs::*;

    static TEST_DIR: &str = "test_dir";

    #[test]
    fn test_file_channel() {
        mkdir(TEST_DIR);
        let test_file = create_path(&vec![TEST_DIR, "aaa"]);
        let mut fc = FileChannel::create(test_file.as_str());
        assert_eq!(fc.len(), 0);
        let mut buf = ByteBuffer::new(1024);
        for i in 0..20usize {
            buf.put(i);
        }
        buf.flip();
        let limit = buf.limit();
        fc.write(&mut buf);
        assert_eq!(fc.len() as usize, limit);
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.limit(), buf.capacity());

        let mut fc_read = FileChannel::open(test_file.as_str());
        fc_read.read(&mut buf).unwrap();
        buf.flip();
        let mut now = 0usize;
        while buf.has_remaining() {
            assert_eq!(now, buf.get::<usize>().unwrap());
            now += 1;
        }

        buf.clear();
        let mut fc_read = FileChannel::open(test_file.as_str());
        for _ in 0..10 {
            buf.put(100usize).unwrap();
        }
        fc_read.read(&mut buf).unwrap();
        buf.flip();
        assert_eq!(buf.limit(), 80 + (fc_read.len() as usize));

        rmr(TEST_DIR);
    }


}