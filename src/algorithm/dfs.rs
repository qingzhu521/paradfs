#![allow(dead_code)]

use std::collections::HashSet;
use std::thread;
use std::sync::Arc;
use std::cmp::min;
use crate::structure::{Graph, GraphPath};

/// 从now开始往外探，最多探k度
/// 停止条件：
///     1. 遇到超点，将当前路径存入hot_path_map
///     2. 遇到target，将当前路径加入result
///     3. 探完k度，直接返回
/// 参数说明：
///     hot_path_map：key表示终点的id，这个点一定是超点，value表示从起点到这个超点的所有k度内的不含超点路径
///     path：表示当前dfs到的路径
///     visit：表示当前已经访问过的点
///     rev：true表示反向探，false表示正向探
///     result：表示当前已经找到的答案，不包括起点和终点
pub fn dfs(
    now: i64, 
    target: i64, 
    k: u32, 
    graph: &Graph, 
    result: &mut Vec<GraphPath>,
    path: &mut GraphPath, 
    rev: bool, 
    visit: &mut HashSet<i64>) {
    if now == target {
        // 停止条件2
        let ans = Vec::from(&path[0..path.len() - 1]);
        result.push(ans);
        return;
    } else if k == 0 {
        // 停止条件3
        return;
    }
    visit.insert(now);
    let next = if rev { graph.in_v(now) } else { graph.out_v(now) };
    if let Some(nodes) = next {
        for v in nodes.iter() {
            if !visit.contains(v) {
                path.push(*v);
                dfs(*v, target, k - 1, graph, result, path, rev, visit);
                path.pop();
            }
        }
    }
    visit.remove(&now);
}

pub fn dfs_parallel(
    now: i64, 
    target: i64, 
    k: u32, 
    graph: Arc<Graph>,  
    result: &mut Vec<GraphPath>,
    path: &mut GraphPath, 
    rev: bool, 
    visit: &mut HashSet<i64>) {
    let mut temp_result = Vec::new();

    visit.insert(now);
    dfs_for_temp(
        path, 
        now,
        target, 
        min(3, k / 2), 
        graph.as_ref(), 
        result, 
        &mut temp_result,
        rev, 
        visit);
    visit.remove(&now);

    dfs_for_continue_parallel(
        Arc::new(temp_result), 
        target, 
        k, 
        graph.clone(), 
        result, 
        rev);
}


fn dfs_for_temp(
    path: &mut GraphPath, 
    start: i64,
    target: i64, 
    k: u32, 
    graph: &Graph, 
    result: &mut Vec<GraphPath>,
    temp_result: &mut Vec<GraphPath>,
    rev: bool, 
    visit: &mut HashSet<i64>) {
    let now = if path.is_empty() {
        start
    } else  {
        path[path.len() - 1]
    };
    if k == 0 {
        // 停止条件3
        let ans = Vec::from(&path[0..path.len()]);
        temp_result.push(ans);
        return;
    } else if now == target {
        // 停止条件2
        let ans = Vec::from(&path[0..path.len() - 1]);
        result.push(ans);
        return;
    }

    let next = if rev { graph.in_v(now) } else { graph.out_v(now) };
    if let Some(nodes) = next {
        for v in nodes.iter() {
            if !visit.contains(v) {
                visit.insert(*v);
                path.push(*v);
                dfs_for_temp(
                    path, 
                    start,
                    target, 
                    k - 1, 
                    graph, 
                    result,
                    temp_result,
                    rev, 
                    visit);
                path.pop();
                visit.remove(v);
            }
        }
    }
}


fn dfs_for_continue(
    temp_result: &mut Vec<GraphPath>, 
    target: i64, 
    k: u32, 
    graph: &Graph, 
    result: &mut Vec<GraphPath>,
    rev: bool, 
) {
    for mut path in temp_result.drain(..) {
        let mut vesited = HashSet::<i64>::new();
        for ele in path.iter() {
            vesited.insert(*ele);
        }
        let len = path.len();
        let now = path[len - 1];
        dfs(
            now, 
            target, 
            k - len as u32, 
            graph, 
            result, &mut path, 
            rev, 
            &mut vesited);
    }
}

fn dfs_for_continue_parallel(
    temp_result: Arc<Vec<GraphPath>>, 
    target: i64, 
    k: u32, 
    graph: Arc<Graph>, 
    result: &mut Vec<GraphPath>,
    rev: bool, 
) {
    let mut handlers = vec![];
    for i in 0..4 {
        let temp_result = temp_result.clone();
        let graph = graph.clone();
        let handler = thread::spawn(move || -> Vec<GraphPath> {
            let mut result = Vec::new();
            for (idx, p) in temp_result.iter().enumerate() {
                if idx % 4 == i {
                    let mut path = p.clone();
                    let mut vesited = HashSet::<i64>::new();
                    for ele in path.iter() {
                        vesited.insert(*ele);
                    }
                    let len = path.len();
                    let now = path[len - 1];
  
                    dfs(
                        now, 
                        target, 
                        k - len as u32, 
                        graph.as_ref(), 
                        &mut result, 
                        &mut path, 
                        rev, 
                        &mut vesited);
                    }
            }
            (result)
        });

        handlers.push(handler);
    }
    
    for handle in handlers.drain(..) {
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
}


#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn test_dfs() {
        let now = 1;
        let target = 0;
        let k = 4;
        let mut graph = Graph::empty();
        
        graph.add_directed_edge(1, 1000000);
        graph.add_directed_edge(1, 2000);
        graph.add_directed_edge(2000, 2000000);
        graph.add_directed_edge(2000, 2500);
        graph.add_directed_edge(2500, 3000);
        graph.add_directed_edge(1, 3000);
        graph.add_directed_edge(3000, 300000);
        graph.add_directed_edge(300000, 3000000);
        graph.add_directed_edge(3000000, 3333);
        graph.add_directed_edge(3333, 333);
        graph.add_directed_edge(333, 0);
        graph.add_directed_edge(2000000, 2222);
        graph.add_directed_edge(2222, 0);
        graph.add_directed_edge(1000000, 0);
        graph.add_directed_edge(333, 2233);
        graph.add_directed_edge(2233, 2222);

        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 0);
        graph.add_directed_edge(1, 3);
        graph.add_directed_edge(3, 4);
        graph.add_directed_edge(4, 0);
        graph.add_directed_edge(2, 5);
        graph.add_directed_edge(5, 3);

        graph.add_undirected_edge(1, 9);
        graph.add_undirected_edge(9, 10);
        graph.add_directed_edge(9, 0);
        graph.add_directed_edge(1, 100);
        graph.add_directed_edge(100, 200);
        graph.add_directed_edge(200, 300);
        graph.add_directed_edge(300, 0);


        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut visit = HashSet::new();
        dfs(now, target, k, &graph, &mut result, &mut path, false, &mut visit);
        assert!(visit.is_empty());
        assert!(path.is_empty());
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![3, 4]));
        assert!(result.contains(&vec![9]));
        assert!(result.contains(&vec![100, 200, 300]));
        assert!(result.contains(&vec![1000000]));
        assert!(result.contains(&vec![2000, 2000000, 2222]));
      
        let now = 0;
        let target = 1;
        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut visit = HashSet::new();
        dfs(now, target, k, &graph, &mut result, &mut path, true, &mut visit);
        assert!(visit.is_empty());
        assert!(path.is_empty());
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![4, 3]));
        assert!(result.contains(&vec![9]));
        assert!(result.contains(&vec![300, 200, 100]));
        assert!(result.contains(&vec![1000000]));
        assert!(result.contains(&vec![2222, 2000000, 2000]));
    }

    #[test]
    fn test_dfs_seprate() {

        let mut graph = Graph::empty();
        
        graph.add_directed_edge(1       , 1000000   );
        graph.add_directed_edge(1       , 2000      );
        graph.add_directed_edge(2000    , 2000000   );
        graph.add_directed_edge(2000    , 2500      );
        graph.add_directed_edge(2500    , 3000      );
        graph.add_directed_edge(1       , 3000      );
        graph.add_directed_edge(3000    , 300000    );
        graph.add_directed_edge(300000  , 3000000   );
        graph.add_directed_edge(3000000 , 3333      );
        graph.add_directed_edge(3333    , 333       );
        graph.add_directed_edge(333     , 0         );
        graph.add_directed_edge(2000000 , 2222      );
        graph.add_directed_edge(2222    , 0         );
        graph.add_directed_edge(1000000 , 0         );
        graph.add_directed_edge(333     , 2233      );
        graph.add_directed_edge(2233    , 2222      );

        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 0);
        graph.add_directed_edge(1, 3);
        graph.add_directed_edge(3, 4);
        graph.add_directed_edge(4, 0);
        graph.add_directed_edge(2, 5);
        graph.add_directed_edge(5, 3);

        graph.add_undirected_edge(1, 9);
        graph.add_undirected_edge(9, 10);
        graph.add_directed_edge(9, 0);
        graph.add_directed_edge(1, 100);
        graph.add_directed_edge(100, 200);
        graph.add_directed_edge(200, 300);
        graph.add_directed_edge(300, 0);

        let now = 1;
        let target = 0;
        let k = 3;
        let mut result = Vec::new();
        let mut temp_result = Vec::new();
        let mut path = Vec::new();
        let mut visit = HashSet::new();
        visit.insert(now);
        dfs_for_temp(
            &mut path, 
            now,
            target, 
            k, 
            &graph, 
            &mut result, 
            &mut temp_result,
            false, 
            &mut visit);
        visit.remove(&now);

        
        assert!(visit.is_empty());
        assert!(path.is_empty());
        assert_eq!(result.len(), 3);
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![9]));
        assert!(result.contains(&vec![1000000]));

        assert_eq!(temp_result.len(), 6);
        assert!(temp_result.contains(&vec![2000, 2500, 3000]));
        assert!(temp_result.contains(&vec![2, 5, 3]));
        assert!(temp_result.contains(&vec![3000, 300000, 3000000]));
        assert!(temp_result.contains(&vec![100, 200, 300]));
        assert!(temp_result.contains(&vec![3, 4, 0]));
        assert!(temp_result.contains(&vec![2000, 2000000, 2222]));

        dfs_for_continue(
            &mut temp_result, 
            0, 
            4, 
            &graph, 
            &mut result, 
            false);

        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![3, 4]));
        assert!(result.contains(&vec![9]));
        assert!(result.contains(&vec![100, 200, 300]));
        assert!(result.contains(&vec![2000, 2000000, 2222]));
        assert!(result.contains(&vec![1000000]));
    }

    #[test]
    fn test_dfs_parallel() {

        let mut graph = Graph::empty();

        graph.add_directed_edge(1       , 1000000   );
        graph.add_directed_edge(1       , 2000      );
        graph.add_directed_edge(2000    , 2000000   );
        graph.add_directed_edge(2000    , 2500      );
        graph.add_directed_edge(2500    , 3000      );
        graph.add_directed_edge(1       , 3000      );
        graph.add_directed_edge(3000    , 300000    );
        graph.add_directed_edge(300000  , 3000000   );
        graph.add_directed_edge(3000000 , 3333      );
        graph.add_directed_edge(3333    , 333       );
        graph.add_directed_edge(333     , 0         );
        graph.add_directed_edge(2000000 , 2222      );
        graph.add_directed_edge(2222    , 0         );
        graph.add_directed_edge(1000000 , 0         );
        graph.add_directed_edge(333     , 2233      );
        graph.add_directed_edge(2233    , 2222      );

        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 0);
        graph.add_directed_edge(1, 3);
        graph.add_directed_edge(3, 4);
        graph.add_directed_edge(4, 0);
        graph.add_directed_edge(2, 5);
        graph.add_directed_edge(5, 3);

        graph.add_undirected_edge(1, 9);
        graph.add_undirected_edge(9, 10);
        graph.add_directed_edge(9, 0);
        graph.add_directed_edge(1, 100);
        graph.add_directed_edge(100, 200);
        graph.add_directed_edge(200, 300);
        graph.add_directed_edge(300, 0);

        let now = 1;
        let target = 0;
        let k = 3;
        let mut result = Vec::new();
        let mut temp_result = Vec::new();
        let mut path = Vec::new();
        let mut visit = HashSet::new();
        visit.insert(now);
        dfs_for_temp(
            &mut path, 
            now,
            target, 
            k, 
            &graph, 
            &mut result, 
            &mut temp_result,
            false, 
            &mut visit);
        visit.remove(&now);

        
        assert!(visit.is_empty());
        assert!(path.is_empty());
        assert_eq!(result.len(), 3);
        assert!(result.contains(&vec![1000000]));
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![9]));

        assert_eq!(temp_result.len(), 6);
        assert!(temp_result.contains(&vec![2000, 2500, 3000]));
        assert!(temp_result.contains(&vec![2, 5, 3]));
        assert!(temp_result.contains(&vec![3000, 300000, 3000000]));
        assert!(temp_result.contains(&vec![100, 200, 300]));
        assert!(temp_result.contains(&vec![3, 4, 0]));
        assert!(temp_result.contains(&vec![2000, 2000000, 2222]));


        dfs_for_continue_parallel(
            Arc::new(temp_result), 
            0, 
            4, 
            Arc::new(graph), 
            &mut result, 
            false);

        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![3, 4]));
        assert!(result.contains(&vec![9]));
        assert!(result.contains(&vec![100, 200, 300]));
        assert!(result.contains(&vec![2000, 2000000, 2222]));
        assert!(result.contains(&vec![1000000]));
    }

    
}