use crate::structure::{Graph, AdjacentList};
use crate::common::io::*;
use std::time::Instant;

pub fn load_binary_graph(dir: String) -> Graph {
    println!("start to load binary graph");
    let now = Instant::now();
    let adj_path = fs::create_path(&vec![dir.as_str(), "adj"]);
    let rev_adj_path = fs::create_path(&vec![dir.as_str(), "rev_adj"]);

    let adj = read_data(adj_path);
    let rev_adj = read_data(rev_adj_path);
    println!("finish to load binary graph, cost {:?}", now.elapsed());
    Graph::new(adj, rev_adj)
}

fn read_data(path: String) -> AdjacentList {
    let mut buf = ByteBuffer::new(128<<20);
    let mut fc = FileChannel::open(path);
    let mut ret = AdjacentList::new();
    fc.read(&mut buf).unwrap();
    buf.flip();
    let mut cnt = 0;
    loop {
        if buf.remaining() < 12 {
            buf.compact();
            let size = fc.read(&mut buf).unwrap();
            buf.flip();
            if size == 0 {
                break;
            }
        }
        let src_id = buf.get::<i64>().unwrap();
        let count = buf.get::<u32>().unwrap();
        let mut tmp = Vec::with_capacity(count as usize);
        for _ in 0..count {
            if buf.remaining() < 8 {
                buf.compact();
                fc.read(&mut buf).unwrap();
                buf.flip();
            }
            tmp.push(buf.get().unwrap());
        }
        ret.insert(src_id, tmp);
        cnt += 1;
        if cnt % 1000000 == 0 {
            println!("load {} nodes", cnt);
        }
    }
    ret
}
