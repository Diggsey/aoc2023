use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("../../inputs/day17.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
    dist: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct OrderedState {
    loss: u32,
    state: State,
}

impl PartialOrd for OrderedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.loss.partial_cmp(&self.loss)
    }
}

impl Ord for OrderedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.loss.cmp(&self.loss)
    }
}

fn main() {
    let grid: Vec<Vec<u32>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut expanded_states = HashSet::new();
    let mut states_to_process = BinaryHeap::new();
    states_to_process.push(OrderedState {
        loss: 0,
        state: State {
            x: 0,
            y: 0,
            dir: Direction::East,
            dist: 0,
        },
    });

    while let Some(OrderedState { loss, state }) = states_to_process.pop() {
        if state.x == grid[0].len() - 1 && state.y == grid.len() - 1 && state.dist >= 4 {
            println!("{loss}");
            break;
        }
        if expanded_states.insert(state) {
            for dir in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                if dir == state.dir.opposite()
                    || (dir == state.dir && state.dist >= 10)
                    || (dir != state.dir && (state.dist > 0 && state.dist < 4))
                {
                    continue;
                }
                let (x, y) = match dir {
                    Direction::North => (state.x, state.y.wrapping_sub(1)),
                    Direction::East => (state.x + 1, state.y),
                    Direction::South => (state.x, state.y + 1),
                    Direction::West => (state.x.wrapping_sub(1), state.y),
                };
                if x >= grid[0].len() || y >= grid.len() {
                    continue;
                }
                let new_loss = loss + grid[y][x];
                states_to_process.push(OrderedState {
                    loss: new_loss,
                    state: State {
                        x,
                        y,
                        dir,
                        dist: if dir == state.dir { state.dist + 1 } else { 1 },
                    },
                });
            }
        }
    }
}
