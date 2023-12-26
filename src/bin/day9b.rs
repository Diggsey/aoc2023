use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day9.txt");

fn main() {
    let sum: i64 = INPUT
        .lines()
        .map(|line| {
            let mut values: Vec<_> = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let mut prediction = 0;
            let mut sign = -1;
            while !values.iter().all(|v| *v == 0) {
                prediction = *values.first().unwrap() - prediction;
                sign *= -1;
                values = values
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
            }
            sign * prediction
        })
        .sum();
    println!("{sum}");
}
