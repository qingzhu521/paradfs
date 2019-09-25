use std::sync::Arc;
use std::collections::HashSet;
use std::cmp::min;
use crate::structure::Graph;
use crate::structure::GraphPath;
use crate::algorithm::dfs_parallel::dfs_parallel;

pub fn cal_part(
    u: i64,
    v: i64,
    limit: u32,
    graph: Arc<Graph>,
    total: &mut Vec<GraphPath>,
    part_answer: &mut Vec<GraphPath>
) {
    let mut visited = HashSet::new();
    let mut path = GraphPath::new();
    dfs_parallel(
        u, 
        v, 
        limit, 
        graph.clone(), 
        total,
        Some(part_answer), 
        &mut path, 
        false,
        &mut visited
    );
}
pub fn join(
    _left_part_answer: &Vec<GraphPath>, 
    _right_part_answer: &Vec<GraphPath>) {
}

pub fn incremental_path(
    u: i64,
    v: i64,
    limit: u32,
    graph: Arc<Graph>,
    result: &mut Vec<GraphPath>,
) {
    let half_len = (limit + 1) / 2;
    let mut left_part_answer = vec![];
    let mut left_answer = vec![];
    cal_part(u, v, half_len, graph.clone(), &mut left_part_answer, &mut left_answer);
    let mut right_part_answer = vec![];
    let mut right_answer = vec![];
    cal_part(v, u, half_len, graph.clone(), &mut right_part_answer, &mut right_answer);

    let mut lu = vec![];
    let mut lv = vec![];
    let mut minlu = usize::max_value();
    let mut minlv = usize::max_value();
    for lp in left_part_answer {
        if lp.len() > 1 {
            if lp[lp.len() - 1] == u {                
                minlu = min(minlu, lp.len());
                lu.push(lp);
            } else if lp[lp.len() - 1] == v {
                minlv = min(minlv, lp.len());
                lv.push(lp); 
            }
        }
    }

    let mut ru = vec![];
    let mut rv = vec![];
    let mut minru = usize::max_value();
    let mut minrv = usize::max_value();
    for rp in right_part_answer {
        if rp.len() > 1 {
            if rp[rp.len() - 1] == u {
                minru = min(minru, rp.len());
                ru.push(rp);
            } else if rp[rp.len() - 1] == v {
                minrv = min(minrv, rp.len());
                rv.push(rp);
            }
        }
    }
    
    
}

