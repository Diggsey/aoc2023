use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day23.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn advance(dir: &mut Option<Direction>) -> Option<Direction> {
    let old = *dir;
    *dir = match dir {
        Some(Direction::N) => Some(Direction::E),
        Some(Direction::E) => Some(Direction::S),
        Some(Direction::S) => Some(Direction::W),
        Some(Direction::W) => None,
        None => None,
    };
    old
}

impl Direction {
    fn apply(&self, coord: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::N => (coord.0, coord.1 - 1),
            Direction::E => (coord.0 + 1, coord.1),
            Direction::S => (coord.0, coord.1 + 1),
            Direction::W => (coord.0 - 1, coord.1),
        }
    }
}

enum Cell {
    Empty,
    Wall,
    Slope(Direction),
}

struct State {
    pos: (usize, usize),
    next_dir: Option<Direction>,
}

fn main() {
    let map = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    '^' => Cell::Slope(Direction::N),
                    '>' => Cell::Slope(Direction::E),
                    'v' => Cell::Slope(Direction::S),
                    '<' => Cell::Slope(Direction::W),
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut visited_cells = HashSet::new();
    visited_cells.insert((1, 0));
    let mut stack = Vec::new();
    stack.push(State {
        pos: (1, 1),
        next_dir: Some(Direction::N),
    });
    let mut max_walk = 0;
    loop {
        let Some(state) = stack.last_mut() else { break };
        if let Some(dir) = advance(&mut state.next_dir) {
            let pos = dir.apply(state.pos);
            let can_pass = match map[pos.1][pos.0] {
                Cell::Empty => true,
                Cell::Wall => false,
                Cell::Slope(d) => d == dir,
            } && !visited_cells.contains(&pos);
            if can_pass {
                if pos.1 == map.len() - 1 {
                    if stack.len() + 1 > max_walk {
                        max_walk = stack.len() + 1;
                    }
                } else {
                    stack.push(State {
                        pos,
                        next_dir: Some(Direction::N),
                    });
                    visited_cells.insert(pos);
                }
            }
        } else {
            visited_cells.remove(&state.pos);
            stack.pop();
        }
    }
    println!("{max_walk}");
}
