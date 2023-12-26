use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day12.txt");

#[derive(Debug, Hash, Eq, PartialEq)]
struct Args<'a> {
    group_size: usize,
    record: &'a [Option<bool>],
    groups: &'a [usize],
}

fn solve<'a>(cache: &mut HashMap<Args<'a>, u64>, args: Args<'a>) -> u64 {
    if let Some(result) = cache.get(&args) {
        return *result;
    }
    let Args {
        group_size,
        record,
        groups,
    } = args;
    let result = match (record, groups) {
        ([], []) if group_size == 0 => 1,
        ([], [group]) if *group == group_size => 1,
        ([Some(false), rest @ ..], _) if group_size == 0 => solve(
            cache,
            Args {
                group_size: 0,
                record: rest,
                groups,
            },
        ),
        ([Some(false), rest @ ..], [group, rem_groups @ ..]) if group_size == *group => solve(
            cache,
            Args {
                group_size: 0,
                record: rest,
                groups: rem_groups,
            },
        ),
        ([Some(true), rest @ ..], [group, ..]) if *group > group_size => solve(
            cache,
            Args {
                group_size: group_size + 1,
                record: rest,
                groups,
            },
        ),
        ([None, rest @ ..], _) if group_size == 0 => {
            solve(
                cache,
                Args {
                    group_size: group_size + 1,
                    record: rest,
                    groups,
                },
            ) + solve(
                cache,
                Args {
                    group_size: 0,
                    record: rest,
                    groups,
                },
            )
        }
        ([None, rest @ ..], [group, rem_groups @ ..]) if group_size == *group => {
            solve(
                cache,
                Args {
                    group_size: group_size + 1,
                    record: rest,
                    groups,
                },
            ) + solve(
                cache,
                Args {
                    group_size: 0,
                    record: rest,
                    groups: rem_groups,
                },
            )
        }
        ([None, rest @ ..], [group, ..]) if *group > group_size => solve(
            cache,
            Args {
                group_size: group_size + 1,
                record: rest,
                groups,
            },
        ),
        _ => 0,
    };
    cache.insert(args, result);
    result
}

fn main() {
    let sum: u64 = INPUT
        .lines()
        .map(|line| {
            let (record, groups) = line.split_once(' ').unwrap();
            let mut record: Vec<_> = record
                .chars()
                .map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    '?' => None,
                    _ => unreachable!(),
                })
                .collect();
            record.push(None);
            let groups: Vec<_> = groups
                .split(',')
                .map(|group| group.parse::<usize>().unwrap())
                .collect();
            let record_len = record.len();
            let record: Vec<_> = record
                .into_iter()
                .cycle()
                .take(record_len * 5 - 1)
                .collect();
            let groups_len = groups.len();
            let groups: Vec<_> = groups.into_iter().cycle().take(groups_len * 5).collect();
            solve(
                &mut HashMap::new(),
                Args {
                    group_size: 0,
                    record: &record,
                    groups: &groups,
                },
            )
        })
        .sum();

    println!("{sum}");
}
