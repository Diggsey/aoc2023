use std::collections::HashMap;

use num::integer::lcm;

const INPUT: &str = include_str!("../../inputs/day8.txt");

fn main() {
    let (directions, rest) = INPUT.split_once("\n\n").unwrap();
    let directions = directions.chars().map(|c| c == 'R').cycle();
    let map: HashMap<_, _> = rest
        .lines()
        .map(|line| {
            let from = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (from, (left, right))
        })
        .collect();

    let starting_nodes: Vec<_> = map.keys().copied().filter(|k| k.ends_with("A")).collect();
    let mut cycle_len = 1;
    for mut current in starting_nodes {
        for (i, direction) in directions.clone().enumerate() {
            if current.ends_with("Z") {
                cycle_len = lcm(cycle_len, i as u64);
                break;
            }
            let (a, b) = map[current];
            current = if direction { b } else { a };
        }
    }
    println!("{cycle_len}");
}
