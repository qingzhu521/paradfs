use std::collections::HashMap;
use crate::structure::AdjacentList;

#[derive(PartialEq, Debug)]
pub struct Graph {
    pub adj: AdjacentList,
    pub rev_adj: AdjacentList,
}

unsafe impl Send for Graph {}

unsafe impl Sync for Graph {}

impl Graph {
    pub fn new(adj: AdjacentList, rev_adj: AdjacentList) -> Self {
        Graph {
            adj,
            rev_adj,
        }
    }

    pub fn v(&self) -> Vec<i64> {
        let mut ret = Vec::new();
        for (v, _) in self.adj.iter() {
            ret.push(*v);
        }
        ret
    }

    /// 从id往外探一度
    pub fn out_v(&self, id: i64) -> Option<&Vec<i64>> {
        self.adj.get(&id)
    }

    /// 从id逆向探一度
    pub fn in_v(&self, id: i64) -> Option<&Vec<i64>> {
        self.rev_adj.get(&id)
    }

    pub fn cal_degree(&self, id: i64) -> u64 {
        let mut ret = 0;
        if let Some(x) = self.out_v(id) {
            ret += x.len();
        }
        if let Some(x) = self.in_v(id) {
            ret += x.len();
        }
        ret as u64
    }


    pub fn test() -> Self {
        let mut graph = Graph {
            adj: HashMap::new(),
            rev_adj: HashMap::new(),
        };
        graph.add_undirected_edge(1, 2);
        graph.add_directed_edge(1, 3);

        graph
    }

    pub fn add_undirected_edge(&mut self, id1: i64, id2: i64) {
        self.add_directed_edge(id1, id2);
        self.add_directed_edge(id2, id1);
    }

    pub fn add_directed_edge(&mut self, id1: i64, id2: i64) {
        self.adj.entry(id1).or_insert(Vec::new()).push(id2);
        self.rev_adj.entry(id2).or_insert(Vec::new()).push(id1);
    }

    pub fn test_large() -> Self {
        let mut graph = Graph {
            adj: HashMap::new(),
            rev_adj: HashMap::new(),
        };

        for x in 1..=3 {
            for i in (x*1000)..(x+1)*1000 {
                graph.add_undirected_edge(x, i);
            }
            for i in (x*11000)..(x+1)*11000 {
                graph.add_directed_edge(x, i);
            }
        }

        graph.add_directed_edge(1, 5);
        graph.add_directed_edge(5, 2);
        graph.add_directed_edge(2, 3);
        graph.add_directed_edge(2, 6);
        graph.add_directed_edge(6, 7);
        graph.add_directed_edge(7, 3);
        graph.add_directed_edge(8, 3);
        graph.add_directed_edge(9, 3);
        graph.add_directed_edge(2, 8);

        graph.add_undirected_edge(1, 4);
        graph.add_undirected_edge(3, 4);
        graph.add_undirected_edge(8, 9);


        graph
    }

    pub fn empty() -> Self {
        Graph {
            adj: HashMap::new(),
            rev_adj: HashMap::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_degree() {
        let graph = Graph::test();
        assert_eq!(graph.cal_degree(1), 3);
        assert_eq!(graph.cal_degree(2), 2);
        assert_eq!(graph.cal_degree(3), 1);

    }
}