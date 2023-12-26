use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day18.txt");

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut trench = HashSet::new();
    trench.insert([x, y]);
    for line in INPUT.lines() {
        let (dir, dist, _color) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let dist: usize = dist.parse().unwrap();
        for _ in 0..dist {
            match dir {
                "R" => x += 1,
                "L" => x -= 1,
                "D" => y += 1,
                "U" => y -= 1,
                _ => unreachable!(),
            }
            trench.insert([x, y]);
        }
    }
    let (mut xmin, mut xmax) = trench
        .iter()
        .map(|&[x, _]| x)
        .minmax()
        .into_option()
        .unwrap();
    let (mut ymin, mut ymax) = trench
        .iter()
        .map(|&[_, y]| y)
        .minmax()
        .into_option()
        .unwrap();
    xmin -= 1;
    ymin -= 1;
    xmax += 2;
    ymax += 2;

    let mut cells_to_process = VecDeque::new();
    let mut processed_cells = HashSet::new();
    cells_to_process.push_back([xmin, ymin]);
    while let Some([x, y]) = cells_to_process.pop_front() {
        if x < xmin || x >= xmax || y < ymin || y >= ymax || trench.contains(&[x, y]) {
            continue;
        }
        if processed_cells.insert([x, y]) {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    cells_to_process.push_back([x + dx, y + dy]);
                }
            }
        }
    }
    let interior = (xmax - xmin) * (ymax - ymin) - processed_cells.len() as i32;
    println!("{interior}");
}
