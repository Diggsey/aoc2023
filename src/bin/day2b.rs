use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let sum: i32 = INPUT
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Game ").unwrap();
            let (_game_no, sets) = line.split_once(": ").unwrap();
            let power: i32 = sets
                .split("; ")
                .flat_map(|subset| {
                    subset
                        .split(", ")
                        .map(|sample| {
                            let (count, color) = sample.split_once(' ').unwrap();
                            (color, count.parse::<i32>().unwrap())
                        })
                        .into_grouping_map()
                        .sum()
                })
                .into_grouping_map()
                .max()
                .into_values()
                .product();
            power
        })
        .sum();
    println!("{}", sum);
}
