use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::structure::{Graph, AdjacentList};

pub fn load_data(static_path: String, dynamic_path: String) -> Result<Graph, LoadError> {
    let mut graph = Graph::new(AdjacentList::new(), AdjacentList::new());
    let f = File::open(static_path)?;
    println!("start to load static data");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let l = line?;
        let items: Vec<&str> = l.split_whitespace().collect();
        let src_id = items[0].parse()?;
        let dst_id = items[1].parse()?;
        graph.add_undirected_edge(src_id, dst_id);
    }
    println!("start to load dynamic data");
    let f = File::open(dynamic_path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let l = line?;
        let items: Vec<&str> = l.split_whitespace().collect();
        let src_id = items[0].parse()?;
        let dst_id = items[1].parse()?;
        graph.add_directed_edge(src_id, dst_id);
    }
    println!("load data success");
    Ok(graph)
}

#[derive(Debug, Clone)]
pub struct LoadError {
    err_msg: String,
}

impl LoadError {
    pub fn new(err_msg: String) -> Self {
        LoadError {
            err_msg,
        }
    }
}

impl From<::std::io::Error> for LoadError {
    fn from(err: ::std::io::Error) -> Self {
        LoadError {
            err_msg: format!("{:?}", err),
        }
    }
}

impl From<::std::num::ParseIntError> for LoadError {
    fn from(err: ::std::num::ParseIntError) -> Self {
        LoadError {
            err_msg: format!("{:?}", err),
        }
    }
}

