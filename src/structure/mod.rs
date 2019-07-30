pub mod graph;
pub use graph::Graph;

use std::collections::HashMap;
pub type AdjacentList=HashMap<i64, Vec<i64>>;
pub type GraphPath=Vec<i64>;
