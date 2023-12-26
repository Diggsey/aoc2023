use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day23.txt");

#[derive(Copy, Clone, Debug)]
struct Edge {
    to: (usize, usize),
    dist: usize,
}

#[derive(Default, Debug)]
struct Vertex {
    linked: Vec<Edge>,
}

struct State {
    vertices: HashMap<(usize, usize), Vertex>,
    visited: RefCell<HashSet<(usize, usize)>>,
    size: (usize, usize),
    longest_dist: Cell<usize>,
}
impl State {
    fn walk(&self, coord: (usize, usize), dist: usize) {
        if coord == (self.size.0 - 2, self.size.1 - 1) {
            if dist > self.longest_dist.get() {
                self.longest_dist.set(dist);
            }
            return;
        }
        self.visited.borrow_mut().insert(coord);
        for linked in &self.vertices[&coord].linked {
            if !self.visited.borrow().contains(&linked.to) {
                self.walk(linked.to, dist + linked.dist);
            }
        }
        self.visited.borrow_mut().remove(&coord);
    }
}

fn main() {
    let map = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let w = map[0].len();
    let h = map.len();
    let mut vertices = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if !map[y][x] {
                let mut v = Vertex::default();
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if nx < w && ny < h && !map[ny][nx] {
                        v.linked.push(Edge {
                            to: (nx, ny),
                            dist: 1,
                        });
                    }
                }
                vertices.insert((x, y), v);
            }
        }
    }

    let coords = vertices.keys().copied().collect_vec();
    for coord in &coords {
        if vertices[coord].linked.len() == 2 {
            let v = vertices.remove(coord).unwrap();
            for (a, b) in v.linked.into_iter().circular_tuple_windows() {
                for c in &mut vertices.get_mut(&a.to).unwrap().linked {
                    if c.to == *coord {
                        c.to = b.to;
                        c.dist += b.dist;
                    }
                }
            }
        }
    }

    let state = State {
        vertices,
        visited: Default::default(),
        size: (w, h),
        longest_dist: Default::default(),
    };

    state.walk((1, 0), 0);

    println!("{}", state.longest_dist.get());
}
