// 把load上来的数据 write出去
use crate::common::io::*;
use crate::structure::{AdjacentList, Graph};
use std::time::Instant;

pub fn build_binary_graph(graph: &Graph, output_dir: String) {
    println!("start to build graph");
    let now = Instant::now();
    fs::mkdir(output_dir.as_str()).unwrap();
    let adj_path = fs::create_path(&vec![output_dir.as_str(), "adj"]);
    let rev_adj_path = fs::create_path(&vec![output_dir.as_str(), "rev_adj"]);
    write_data(adj_path, &graph.adj);
    write_data(rev_adj_path, &graph.rev_adj);
    println!("finish to build graph, cost {:?}", now.elapsed());
}

fn write_data(path: String, map: &AdjacentList) {
    if fs::exists(path.as_str()) {
        println!("{} already exists", path);
        return;
    }
    let mut buf = ByteBuffer::new(128<<20); // 128MB
    let mut fc = FileChannel::create(path);
    for (id, adj) in map.iter() {
        if buf.remaining() < 12 + adj.len() * 8 {
            buf.flip();
            fc.write(&mut buf).unwrap();
            println!("write {} B", buf.capacity());
        }
        buf.put(*id).unwrap();
        buf.put(adj.len() as u32).unwrap();
        for x in adj.iter() {
            buf.put(*x).unwrap();
        }
    }
    buf.flip();
    fc.write(&mut buf).unwrap();
}