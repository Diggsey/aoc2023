use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../inputs/day4.txt");

fn main() {
    let mut sum = 0;
    let mut following = VecDeque::new();
    following.resize(INPUT.lines().count(), 1);
    for line in INPUT.lines() {
        let num_current = following.pop_front().unwrap();
        sum += num_current;
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
        for i in 0..count {
            following[i] += num_current;
        }
    }
    println!("{}", sum);
}
