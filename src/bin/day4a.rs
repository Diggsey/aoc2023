use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day4.txt");

fn main() {
    let sum: i32 = INPUT
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(": ").unwrap();
            let (winning, actual) = rest.split_once(" | ").unwrap();
            let winning_numbers: HashSet<i32> = winning
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let count = actual
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .filter(|n| winning_numbers.contains(n))
                .count();
            if count > 0 {
                1 << (count - 1)
            } else {
                0
            }
        })
        .sum();
    println!("{}", sum);
}
