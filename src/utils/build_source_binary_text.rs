use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::structure::AdjacentList;
use crate::common::io::*;
use super::load_source_text::LoadError;

pub fn load_text_data(path: String) -> Result<AdjacentList, LoadError> {
    let mut adj = AdjacentList::new();
    let f = File::open(path)?;
    println!("start to load static data");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let l = line?;
        let items: Vec<&str> = l.split_whitespace().collect();
        let src_id = items[0].parse()?;
        let dst_id = items[1].parse()?;
        adj.entry(src_id).or_insert(Vec::new()).push(dst_id);
    }
    
    Ok(adj)
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
pub fn build_source_binary(output_dir: String, static_path: String, dynamic_path: String) {
    let static_adj = load_text_data(static_path).unwrap();
    let static_out = fs::create_path(&vec![output_dir.as_str(), "static"]);
    write_data(static_out, &static_adj);
    let dyn_adj = load_text_data(dynamic_path).unwrap();
    let dyn_out = fs::create_path(&vec![output_dir.as_str(), "dyn"]);
    write_data(dyn_out, &dyn_adj);
}