#![allow(dead_code)]
use std::mem;
use std::intrinsics::copy_nonoverlapping;
use std::intrinsics::copy;

pub struct ByteBuffer {
    data: Vec<u8>,
    pos: usize,
    limit: usize,
    ptr: *mut u8,
}

impl ByteBuffer {
    pub fn new(size: usize) -> Self {
        let mut data = vec![0u8; size];
        let ptr = data.as_mut_ptr();
        ByteBuffer {
            data,
            pos: 0,
            limit: size,
            ptr,
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn limit(&self) -> usize {
        self.limit
    }

    #[inline]
    pub fn set_position(&mut self, position: usize) {
        self.pos = position;
    }

    #[inline]
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    #[inline]
    pub fn remaining(&self) -> usize {
        if self.limit <= self.pos {
            0
        } else {
            self.limit - self.pos
        }
    }

    #[inline]
    pub fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }

    #[inline]
    pub fn flip(&mut self) {
        self.limit = self.pos;
        self.pos = 0;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.pos = 0;
        self.limit = self.capacity();
    }

    pub fn get<T: Copy>(&mut self) -> Result<T, ()> {
        let size = mem::size_of::<T>();
        if size > self.remaining() {
            Err(())
        } else {
            let ret = unsafe { Ok(*(self.ptr.offset(self.pos as isize) as *const T)) };
            self.pos += size;
            ret
        }
    }

    pub fn get_at<T: Copy>(&self, pos: usize) -> Result<T, ()> {
        let size = mem::size_of::<T>();
        if size + pos > self.limit {
            Err(())
        } else {
            unsafe {Ok(*(self.ptr.offset(pos as isize) as *const T))}
        }
    }

    pub fn put<T: Copy>(&mut self, data: T) -> Result<(), ()> {
        let size = mem::size_of_val(&data);
        if size > self.remaining() {
            Err(())
        } else {
            let p = &data as *const T as *const u8;
            unsafe {
                copy_nonoverlapping(p, self.ptr.offset(self.pos as isize), size);
            }
            self.pos += size;
            Ok(())
        }
    }

    pub fn put_at<T: Copy>(&self, pos: usize, data: T) -> Result<(), ()> {
        let size = mem::size_of::<T>();
        if pos + size > self.limit {
            Err(())
        }  else {
            let p = &data as *const T as *const u8;
            unsafe {
                copy_nonoverlapping(p, self.ptr.offset(pos as isize), size);
            }
            Ok(())
        }
    }

    #[inline]
    pub fn array(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    pub fn compact(&mut self) {
        unsafe {copy(self.ptr.offset(self.pos as isize), self.ptr, self.remaining());}
        self.pos = self.remaining();
        self.limit = self.capacity();
    }

}

use std::fmt;
use std::fmt::Debug;

impl Debug for ByteBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position: {}, limit: {}, capacity: {}", self.position(), self.limit(), self.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_must_use)]
    #[test]
    fn test_basic() {
        let mut buf = ByteBuffer::new(1024);
        buf.put(1usize);
        buf.flip();
        let x = buf.get::<usize>().unwrap();
        assert_eq!(x, 1);

        #[derive(Debug, Copy, Clone)]
        struct Point {
            x: i32,
            y: i64,
        }

        buf.clear();
        buf.put(Point{x:1, y:2});
        buf.flip();
        let p = buf.get::<Point>().unwrap();
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);

    }

    #[allow(unused_must_use)]
    #[test]
    fn test_compact() {
        let mut buf = ByteBuffer::new(32);
        buf.put(1usize);
        buf.put(2usize);
        buf.flip();

        assert_eq!(buf.get::<usize>().unwrap(), 1);
        buf.compact();
        buf.flip();
        assert_eq!(buf.get::<usize>().unwrap(), 2);
    }

}