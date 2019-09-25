use std::collections::HashSet;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::cmp::min;
use std::time::Instant;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::VecDeque;
use crate::structure::{Graph, GraphPath};
use super::dfs::dfs_for_temp;
use super::dfs::dfs;

pub fn dfs_for_continue_parallel(
    temp_result: Arc<Vec<GraphPath>>, 
    target: i64, 
    k: u32, 
    graph: Arc<Graph>, 
    result: &mut Vec<GraphPath>,
    rev: bool, 
    part_ans: Option<&mut Vec<GraphPath>>
) {
    let mut handlers = vec![];
    let ans_flag = part_ans.is_some(); 
    for i in 0..4 {
        let temp_result = temp_result.clone();
        let graph = graph.clone();
        let handler = thread::spawn(move || -> Vec<GraphPath> {
            let cur = Instant::now();
            let mut result = Vec::new();
            let mut part_answer = Vec::new();
            let mut part_a = if ans_flag {
                Some(&mut part_answer)
            } else {
                None
            };
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
                        &mut part_a,
                        &mut path, 
                        rev, 
                        &mut vesited);
                    }
            }
            let stop = cur.elapsed();
            println!("thread {} {:?}", i, stop);
            (result)
        });

        handlers.push(handler);
    }
    
    for handle in handlers.drain(..) {
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
}

pub fn dfs_parallel(
    now: i64, 
    target: i64, 
    k: u32, 
    graph: Arc<Graph>,  
    result: &mut Vec<GraphPath>,
    part_ans: Option<&mut Vec<GraphPath>>,
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
        rev,
        part_ans
    );
}


pub fn dfs_send_path(
    temp_result_sender: &Vec<Sender<GraphPath>>,
    empty_group_receive: &Receiver<i64>,
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
                let received = empty_group_receive.recv();
                let is_skip = match received {
                    Ok(thread_index) =>
                     {
                         let _ = temp_result_sender[thread_index as usize].send(path.clone());
                         true
                     }
                     _ => false
                };
                if is_skip {
                    path.pop();
                    continue;
                }
                dfs_send_path(
                    temp_result_sender,
                    empty_group_receive, 
                    *v, target, 
                    k - 1, 
                    graph, 
                    result, 
                    path, 
                    rev, 
                    visit);
                path.pop();
            }
        }
    }
    visit.remove(&now);
}


pub fn dfs_for_continue_parallel_balance_stealing(
    temp_result: Arc<Vec<GraphPath>>, 
    target: i64, 
    k: u32, 
    graph: Arc<Graph>, 
    result: &mut Vec<GraphPath>,
    rev: bool, 
) {

    let mut handlers = vec![];

    let mut path_sender = vec![];
    let mut path_receiver = vec![];
    let mut require_sender = vec![];
    let mut require_receiver = vec![];

    for _i in 0..4 {
        let (temp_path_send, temp_path_receive) = channel::<GraphPath>();
        path_sender.push(temp_path_send);
        path_receiver.push(temp_path_receive);

        let (empty_require_send, empty_require_receive) = channel::<i64>();
        require_sender.push(empty_require_send);
        require_receiver.push(empty_require_receive);
    }

    let mut path_receiver_drain = path_receiver.drain(..);
    let mut require_receiver_drain = require_receiver.drain(..);

    for i in 0..4 {
        let path_sender_cloner = path_sender.clone();
        let require_sender_cloner = require_sender.clone();

        let this_path_receiver = path_receiver_drain.next().unwrap();
        let this_require_receiver = require_receiver_drain.next().unwrap();
        

        let temp_result = temp_result.clone();
        let graph = graph.clone();

        let handler = thread::Builder::new()
        .name(i.to_string())
        .spawn(move || -> Vec<GraphPath> {
            let mut result_part = Vec::new();
            let cur = Instant::now();
            let mut to_process = VecDeque::new();
            for (idx, path) in temp_result.iter().enumerate() {
                if idx % 4 == i {
                    to_process.push_back(path.clone());
                }
            }
            loop {
                if let Some(mut ele) = to_process.pop_front() {
                    let path = &mut ele;
                    let mut vesited = HashSet::<i64>::new();
                    for ele in path.iter() {
                        vesited.insert(*ele);
                    }
                    let len = path.len();
                    let now = path[len - 1];
    
                    dfs_send_path(
                        &path_sender_cloner,
                        &this_require_receiver,
                        now, 
                        target, 
                        k - len as u32, 
                        graph.as_ref(), 
                        &mut result_part, 
                        path, 
                        rev, 
                        &mut vesited);
                    
                } else {
                    let res = if i + 1 < 4 {
                        require_sender_cloner[i + 1].send(i as i64)
                    } else {
                        require_sender_cloner[0].send(i as i64)
                    };
                    if let Ok(_) = res {
                        if let Ok(path) = this_path_receiver.recv() {
                            to_process.push_back(path);
                        }
                    } else {
                        break;
                    }
                }
            };
            let _stop = cur.elapsed();
            result_part
        }).unwrap();

        handlers.push(handler);
    }
    
    for handle in handlers.drain(..) {
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
}


pub fn dfs_lock(
    cnt: Arc<Mutex<i64>>,
    take_path: Arc<Mutex<Vec<GraphPath>>>,
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
                let mut lock = cnt.try_lock();
                let mut is_skip = false;
                if let Ok(ref mut mutex) = lock {
                    **mutex -= 1;
                    is_skip = true;

                    take_path.lock().unwrap().push(path.clone());
                }
                
                if is_skip {
                    path.pop();
                    continue;
                }
                dfs_lock(cnt.clone(), take_path.clone(), *v, target, k - 1, graph, result, path, rev, visit);
                path.pop();
            }
        }
    }
    visit.remove(&now);
}

//每个人dfs的过程中先读共享内存
//如果有人请求那么就往内存里面放一条路径
//如果没有就继续运行。

//在loop中如果自己的队列执行完毕了，就往路径中放东西
//共享内存实现
pub fn dfs_in_mutex(
    temp_result: Arc<Vec<GraphPath>>, 
    target: i64, 
    k: u32, 
    graph: Arc<Graph>, 
    result: &mut Vec<GraphPath>,
    rev: bool, 
) {
    let counter = Arc::new(Mutex::new(0));
    let take_path = Arc::new(Mutex::new(vec![]));

    let mut handlers = vec![];
    for i in 0..4 {
        let cnt = counter.clone();
        let tp = take_path.clone();

        let temp_result = temp_result.clone();
        let graph = graph.clone();
        let handler = thread::spawn(move || -> Vec<GraphPath> {
            let mut result_part = Vec::new();
            let cur = Instant::now();

            let mut to_process = VecDeque::new();
            for (idx, path) in temp_result.iter().enumerate() {
                if idx % 4 == i {
                    to_process.push_back(path.clone());
                }
            }

            loop {
                if let Some(mut ele) = to_process.pop_front() {
                    let path = &mut ele;
                    let mut vesited = HashSet::<i64>::new();
                    for ele in path.iter() {
                        vesited.insert(*ele);
                    }
                    let len = path.len();
                    let now = path[len - 1];
    
                    // dfs
                    dfs_lock(
                        cnt.clone(),
                        tp.clone(),
                        now, 
                        target, 
                        k - len as u32, 
                        graph.as_ref(), 
                        &mut result_part, 
                        path, 
                        rev, 
                        &mut vesited);
                } else {
                    if let Ok(mut cnter) = cnt.lock() {
                        *cnter += 1;
                        if let Ok(mut get_p) = tp.lock() {
                            let path = get_p.pop().unwrap();
                            to_process.push_back(path);
                        } else {
                            break;
                        }
                    }
                }

            }
            let _stop = cur.elapsed();
            result_part
        });

        handlers.push(handler);
    }
    
    for handle in handlers.drain(..) {
        let mut res = handle.join().unwrap();
        result.append(&mut res);
    }
}

#[cfg(test)]
mod test {
    use super::*;
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