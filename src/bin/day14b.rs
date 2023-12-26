use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day14.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    RoundRock,
    SquareRock,
}

fn shift_and_compute_weight(map: &mut Vec<Cell>, rows: usize, cols: usize) {
    let mut positions = vec![0; cols];
    for y in 0..rows {
        for x in 0..cols {
            match map[y * cols + x] {
                Cell::Empty => {}
                Cell::RoundRock => {
                    let y2 = &mut positions[x];
                    map[y * cols + x] = Cell::Empty;
                    map[(*y2) * cols + x] = Cell::RoundRock;
                    *y2 += 1;
                }
                Cell::SquareRock => {
                    positions[x] = y + 1;
                }
            }
        }
    }
}

fn rotate(map: &mut Vec<Cell>, rows: usize, cols: usize) {
    for y in 0..(rows / 2) {
        for x in 0..(cols / 2) {
            let original = map[y * cols + x];
            map[y * cols + x] = map[(rows - x - 1) * cols + y];
            map[(rows - x - 1) * cols + y] = map[(rows - y - 1) * cols + (cols - x - 1)];
            map[(rows - y - 1) * cols + (cols - x - 1)] = map[x * cols + (cols - y - 1)];
            map[x * cols + (cols - y - 1)] = original;
        }
    }
}

fn compute_weight(map: &Vec<Cell>, rows: usize, cols: usize) -> u32 {
    let mut weight = 0;
    for y in 0..rows {
        for x in 0..cols {
            match map[y * cols + x] {
                Cell::Empty => {}
                Cell::RoundRock => {
                    weight += (rows - y) as u32;
                }
                Cell::SquareRock => {}
            }
        }
    }
    weight
}

fn main() {
    let rows = INPUT.lines().count();
    let mut map: Vec<_> = INPUT
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '.' => Cell::Empty,
                'O' => Cell::RoundRock,
                '#' => Cell::SquareRock,
                _ => unreachable!(),
            })
        })
        .collect();
    let cols = map.len() / rows;

    let mut visited = HashMap::new();
    for i in 0u64.. {
        if let Some(prev) = visited.insert(map.clone(), i) {
            let cycle_len = i - prev;
            let remaining = 1000000000 - i;
            if remaining % cycle_len == 0 {
                let weight = compute_weight(&map, rows, cols);
                println!("{weight}");
                break;
            }
        }
        for _ in 0..4 {
            shift_and_compute_weight(&mut map, rows, cols);
            rotate(&mut map, rows, cols);
        }
    }
}
