use std::collections::HashSet;

use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let number_re = Regex::new("[0-9]+").unwrap();
    let symbol_re = Regex::new("[^0-9.]").unwrap();
    let symbol_positions: HashSet<_> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            symbol_re
                .find_iter(line)
                .map(move |m| (y as i32, m.start() as i32))
        })
        .collect();
    let sum: i32 = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            number_re.find_iter(line).map(move |m| {
                (
                    y as i32,
                    m.start() as i32,
                    m.end() as i32,
                    m.as_str().parse::<i32>().unwrap(),
                )
            })
        })
        .filter(|&(y, x0, x1, _)| {
            (y - 1..=y + 1)
                .flat_map(|cy| (x0 - 1..x1 + 1).map(move |cx| (cy, cx)))
                .any(|p| symbol_positions.contains(&p))
        })
        .map(|(_, _, _, v)| v)
        .sum();
    println!("{}", sum);
}
