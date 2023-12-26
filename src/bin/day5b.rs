use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn main() {
    let mut groups = INPUT.split("\n\n");
    let seed_group = groups.next().unwrap();
    let mut values: Vec<(i64, i64)> = seed_group
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .tuples()
        .collect();
    for group in groups {
        let mappings: Vec<_> = group
            .lines()
            .skip(1)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64, i64)>()
                    .unwrap()
            })
            .sorted_by_key(|m| m.1)
            .collect();

        values = values
            .into_iter()
            .flat_map(|value| {
                let mut result = Vec::new();
                let mut current = value.0;
                let end = value.0 + value.1;
                for m in &mappings {
                    if m.1 > current {
                        result.push((current, m.1 - current));
                        current = m.1;
                    }
                    if current >= end {
                        break;
                    }
                    let to = end.min(m.1 + m.2);
                    if to > current {
                        result.push((current - m.1 + m.0, to - current));
                        current = to;
                    }
                    if current >= end {
                        break;
                    }
                }
                result
            })
            .collect();
    }
    let min = values.into_iter().map(|x| x.0).min().unwrap();
    println!("{}", min);
}
