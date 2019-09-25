use paradfs::structure::GraphPath;
use paradfs::utils::load_source_text::load_data;
use paradfs::algorithm::dfs::dfs;
use paradfs::algorithm::dfs_parallel::dfs_parallel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let static_path = "Amazon.txt.static".to_string();
    let dyn_path = "Amazon.txt.dynamic".to_string();
    let graph = load_data(static_path, dyn_path).unwrap();
    
    let now = 344;
    let target = 86800;
    let k = 6;
    let mut result = vec![];
    let mut path = GraphPath::new();
    let mut visited = HashSet::new();

    let cur = Instant::now();
    dfs(
        now,
        target,
        k,
        &graph,
        &mut result,
        &mut None,
        &mut path,
        false,
        &mut visited
    );
    assert!(visited.is_empty());
    let stop = cur.elapsed();
    println!("{:?}", stop);
    result.clear();

    let cur = Instant::now();
    dfs_parallel(
        now,
        target,
        k,
        Arc::new(graph),
        &mut result,
        None,
        &mut path,
        false,
        &mut visited
    );
    // dfs_parallel()
    let stop = cur.elapsed();
    println!("{:?}", stop);
}