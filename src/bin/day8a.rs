use std::collections::HashMap;

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

    let mut current = "AAA";
    for (i, direction) in directions.enumerate() {
        if current == "ZZZ" {
            println!("{i}");
            break;
        }
        let (a, b) = map[current];
        current = if direction { b } else { a };
    }
}
