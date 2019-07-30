use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;
use crate::structure::Graph;
use crate::common::io::*;

/// 参数：动态数据目录和静态数据目录
/// 返回值：正向边邻接表和反向边邻接表
pub fn load<P: AsRef<Path>>(dynamic_data_path: P, static_data_path: P) -> Graph {
    let mut adj = HashMap::new();
    let mut rev_adj = HashMap::new();
    load_dynamic(dynamic_data_path, &mut adj, &mut rev_adj);
    load_static(static_data_path, &mut adj, &mut rev_adj);
    Graph::new(adj, rev_adj)
}


/// 参数：动态数据目录
/// 返回值(由参数返回)：正向边邻接表和反向边邻接表
fn load_dynamic<P: AsRef<Path>>(path: P, adj: &mut HashMap<i64, Vec<i64>>, rev_adj: &mut HashMap<i64, Vec<i64>>) {
    let now = Instant::now();
    let mut buf = ByteBuffer::new(128<<20);
    let files = fs::ls(path).unwrap();
    let mut cnt = 0;
    for file in files {
        buf.clear();
        let mut fc = FileChannel::open(file);
        fc.read(&mut buf).unwrap();
        buf.flip();
        loop {
            if buf.remaining() < 4 + 8 {
                buf.compact();
                let size = fc.read(&mut buf).unwrap();
                buf.flip();
                if size == 0 {
                    break;
                }
            }
            let src_id = buf.get::<i64>().unwrap().to_be();
            let count = buf.get::<i32>().unwrap().to_be();
            for _ in 0..count {
                if buf.remaining() < 8 {
                    buf.compact();
                    fc.read(&mut buf).unwrap();
                    buf.flip();
                }
                let dst_id = buf.get::<i64>().unwrap().to_be();
                adj.entry(src_id).or_insert(Vec::new()).push(dst_id);
                rev_adj.entry(dst_id).or_insert(Vec::new()).push(src_id);
            }
            if !buf.has_remaining() {
                buf.clear();
                let size = fc.read(&mut buf).unwrap();
                buf.flip();
                if size == 0 {
                    break;
                }
            }
            buf.get::<i8>().unwrap();
            cnt += 1;
            if cnt % 1000000 == 0{
                println!("load {} nodes, cost {:?}", cnt, now.elapsed());
            }
        }
    }

    println!("load dynamic success, cost {:?}", now.elapsed());
}

/// 参数：静态数据目录
/// 数据说明：这里存的是无向边，因此每个边都得存两遍（原始数据已经特殊处理过了，因此存两遍没有重复）
/// 返回值（由参数返回）：正向边邻接表和反向边邻接表
fn load_static<P: AsRef<Path>>(path: P, adj: &mut HashMap<i64, Vec<i64>>, rev_adj: &mut HashMap<i64, Vec<i64>>) {
    let now = Instant::now();
    let mut buf = ByteBuffer::new(128<<20);
    let files = fs::ls(path).unwrap();
    let mut cnt = 0;
    for file in files {
        buf.clear();
        let mut fc = FileChannel::open(file);
        fc.read(&mut buf).unwrap();
        buf.flip();
        loop {
            if buf.remaining() < 4 + 8 {
                buf.compact();
                let size = fc.read(&mut buf).unwrap();
                buf.flip();
                if size == 0 {
                    break;
                }
            }
            let src_id = buf.get::<i64>().unwrap().to_be();
            let count = buf.get::<i32>().unwrap().to_be();
            for _ in 0..count {
                if buf.remaining() < 8 {
                    buf.compact();
                    fc.read(&mut buf).unwrap();
                    buf.flip();
                }
                let dst_id = buf.get::<i64>().unwrap().to_be();
                adj.entry(src_id).or_insert(Vec::new()).push(dst_id);
                adj.entry(dst_id).or_insert(Vec::new()).push(src_id);
                rev_adj.entry(src_id).or_insert(Vec::new()).push(dst_id);
                rev_adj.entry(dst_id).or_insert(Vec::new()).push(src_id);
            }
            if !buf.has_remaining() {
                buf.clear();
                let size = fc.read(&mut buf).unwrap();
                buf.flip();
                if size == 0 {
                    break;
                }
            }
            buf.get::<i8>().unwrap();
            cnt += 1;
            if cnt % 1000000 == 0 {
                println!("load {} nodes, cost {:?}", cnt, now.elapsed());
            }
        }
    }

    println!("load static success, cost {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_load_dynamic() {
        let test_dir = "test_data";
//        load("/Users/wubincen/project/rust/dmt_paper/utils/part-r-00000");
        let mut adj = HashMap::new();
        let mut rev_adj = HashMap::new();
        load_dynamic(test_dir, &mut adj, &mut rev_adj);
    }

    #[ignore]
    #[test]
    fn test_load_static() {
        let test_dir = "test_data";
        let mut adj = HashMap::new();
        let mut rev_adj = HashMap::new();
        load_static(test_dir, &mut adj, &mut rev_adj);
    }
}