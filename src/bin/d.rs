use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use std::collections::VecDeque;
#[allow(unused_imports)]
use petgraph::graph::{Graph, UnGraph, node_index, NodeIndex};
#[allow(unused_imports)]
use petgraph::{Undirected, Directed};
#[allow(unused_imports)]
use petgraph::visit::{Dfs, Bfs, depth_first_search, Control, DfsEvent};


fn main() {
//     let source = AutoSource::from("6 9
// 3 4
// 5 1
// 2 4
// 5 3
// 4 5
// 1 5
// 5 2
// 4 5
// 5 5
// ");
    input!{
        // from source,
        n: usize,
        m: usize,
        edges: [(usize, usize); m]
    };


    let graph = UnGraph::<(),(), usize>::from_edges(&edges);

    let mut v = vec![-1; n+1];
    v[0] = 0;
    v[1] = 0;

    let mut deque = VecDeque::<usize>::new();
    deque.push_back(1);

    while let Some(idx) = deque.pop_front(){
        for node in graph.neighbors(node_index(idx)) {
            let next_idx = node.index();
            if v[next_idx] != -1 {
                continue
            }
            v[next_idx] = idx as i32;
            deque.push_back(next_idx);
        }
    }

    // println!("{:?}", v);

    if v.iter().filter(|&x| x == &-1).count() > 0 {
        println!("No");
    }
    else {
        println!("Yes");
        for i in 2..n+1 {
            println!("{}", v[i]);
        }
    }
}
