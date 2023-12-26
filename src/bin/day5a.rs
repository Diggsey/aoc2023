use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn main() {
    let mut groups = INPUT.split("\n\n");
    let seed_group = groups.next().unwrap();
    let mut values: Vec<_> = seed_group
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    for group in groups {
        let mappings: Vec<(i64, i64, i64)> = group
            .lines()
            .skip(1)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        values = values
            .into_iter()
            .map(|value| {
                if let Some(m) = mappings.iter().find(|m| value >= m.1 && value < m.1 + m.2) {
                    value - m.1 + m.0
                } else {
                    value
                }
            })
            .collect();
    }
    let min = values.into_iter().min().unwrap();
    println!("{}", min);
}
