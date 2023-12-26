const INPUT: &str = include_str!("../../inputs/day12.txt");

fn solve(group_size: usize, record: &[Option<bool>], groups: &[usize]) -> usize {
    match (record, groups) {
        ([], []) if group_size == 0 => 1,
        ([], [group]) if *group == group_size => 1,
        ([Some(false), rest @ ..], _) if group_size == 0 => solve(0, rest, groups),
        ([Some(false), rest @ ..], [group, rem_groups @ ..]) if group_size == *group => {
            solve(0, rest, rem_groups)
        }
        ([Some(true), rest @ ..], _) => solve(group_size + 1, rest, groups),
        ([None, rest @ ..], _) if group_size == 0 => {
            solve(group_size + 1, rest, groups) + solve(0, rest, groups)
        }
        ([None, rest @ ..], [group, rem_groups @ ..]) if group_size == *group => {
            solve(group_size + 1, rest, groups) + solve(0, rest, rem_groups)
        }
        ([None, rest @ ..], _) => solve(group_size + 1, rest, groups),
        _ => 0,
    }
}

fn main() {
    let sum: usize = INPUT
        .lines()
        .map(|line| {
            let (record, groups) = line.split_once(' ').unwrap();
            let record: Vec<_> = record
                .chars()
                .map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    '?' => None,
                    _ => unreachable!(),
                })
                .collect();
            let groups: Vec<_> = groups
                .split(',')
                .map(|group| group.parse::<usize>().unwrap())
                .collect();
            solve(0, &record, &groups)
        })
        .sum();

    println!("{sum}");
}
