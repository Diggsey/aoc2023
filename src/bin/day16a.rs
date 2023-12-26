use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../inputs/day16.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    MirrorU,
    MirrorD,
    SplitterV,
    SplitterH,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let grid: Vec<Vec<Cell>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '/' => Cell::MirrorU,
                    '\\' => Cell::MirrorD,
                    '|' => Cell::SplitterV,
                    '-' => Cell::SplitterH,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let mut expanded_beams = HashSet::new();
    let mut beams_to_process = VecDeque::new();
    beams_to_process.push_back((0, 0, Direction::East));

    while let Some((x, y, dir)) = beams_to_process.pop_front() {
        if x >= grid[0].len() || y >= grid.len() {
            continue;
        }
        if expanded_beams.insert((x, y, dir)) {
            let dirs = match (grid[y][x], dir) {
                (Cell::Empty, _)
                | (Cell::SplitterV, Direction::North)
                | (Cell::SplitterV, Direction::South)
                | (Cell::SplitterH, Direction::East)
                | (Cell::SplitterH, Direction::West) => vec![dir],
                (Cell::MirrorU, Direction::North) => vec![Direction::East],
                (Cell::MirrorU, Direction::East) => vec![Direction::North],
                (Cell::MirrorU, Direction::South) => vec![Direction::West],
                (Cell::MirrorU, Direction::West) => vec![Direction::South],
                (Cell::MirrorD, Direction::North) => vec![Direction::West],
                (Cell::MirrorD, Direction::East) => vec![Direction::South],
                (Cell::MirrorD, Direction::South) => vec![Direction::East],
                (Cell::MirrorD, Direction::West) => vec![Direction::North],
                (Cell::SplitterV, Direction::East) | (Cell::SplitterV, Direction::West) => {
                    vec![Direction::North, Direction::South]
                }
                (Cell::SplitterH, Direction::North) | (Cell::SplitterH, Direction::South) => {
                    vec![Direction::East, Direction::West]
                }
            };
            for dir in dirs {
                match dir {
                    Direction::North => beams_to_process.push_back((x, y.wrapping_sub(1), dir)),
                    Direction::East => beams_to_process.push_back((x + 1, y, dir)),
                    Direction::South => beams_to_process.push_back((x, y + 1, dir)),
                    Direction::West => beams_to_process.push_back((x.wrapping_sub(1), y, dir)),
                }
            }
        }
    }

    let energised_cells: HashSet<_> = expanded_beams.into_iter().map(|(x, y, _)| (x, y)).collect();

    println!("{}", energised_cells.len());
}
