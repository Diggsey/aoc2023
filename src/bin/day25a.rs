// This solution is very slow but it's Christmas and I CBA implementing
// a more complicated algorithm... Run it in release mode!
use std::collections::HashMap;

use itertools::Itertools;
use rand::{seq::IteratorRandom, thread_rng};

const INPUT: &str = include_str!("../../inputs/day25.txt");

#[derive(Debug, Clone)]
struct Edge {
    size: usize,
}

impl Default for Edge {
    fn default() -> Self {
        Self { size: 1 }
    }
}

#[derive(Debug, Clone)]
struct Node {
    size: usize,
    adjacent: HashMap<&'static str, Edge>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            size: 1,
            adjacent: HashMap::default(),
        }
    }
}

type Graph = HashMap<&'static str, Node>;

fn randomly_contract(graph: &mut Graph) {
    let (a, b) = graph
        .iter()
        .flat_map(|(&k0, node)| node.adjacent.keys().map(move |&k1| (k0, k1)))
        .choose(&mut thread_rng())
        .unwrap();

    let mut a_node = graph.remove(&a).unwrap();
    let b_node = graph.remove(&b).unwrap();

    a_node.size += b_node.size;
    a_node.adjacent.remove(&b);
    for (k, e) in b_node.adjacent {
        if let Some(c) = graph.get_mut(k) {
            c.adjacent.remove(&b);
            c.adjacent.entry(&a).or_insert(Edge { size: 0 }).size += e.size;
            a_node.adjacent.entry(&k).or_insert(Edge { size: 0 }).size += e.size;
        }
    }

    graph.insert(a, a_node);
}

fn try_contraction(mut graph: Graph) -> (Node, Node) {
    while graph.len() > 2 {
        randomly_contract(&mut graph);
    }
    graph.into_values().collect_tuple().unwrap()
}

fn main() {
    let mut graph = Graph::new();
    for line in INPUT.lines() {
        let (a, bs) = line.split_once(": ").unwrap();
        for b in bs.split(' ') {
            graph.entry(a).or_default().adjacent.entry(b).or_default();
            graph.entry(b).or_default().adjacent.entry(a).or_default();
        }
    }

    loop {
        let (a, b) = try_contraction(graph.clone());
        if let Some(edge) = a.adjacent.values().next() {
            if edge.size == 3 {
                let product = a.size * b.size;
                println!("{product}");
                break;
            }
        }
    }
}
