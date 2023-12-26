use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    mem,
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day10.txt");

type Map = HashMap<[i32; 2], [[i32; 2]; 2]>;

fn walk(map: &Map, pos: [i32; 2], prev_pos: [i32; 2]) -> [i32; 2] {
    let [a, b] = map[&pos];
    if a == prev_pos {
        b
    } else {
        a
    }
}

fn main() {
    let start = &Cell::new([0, 0]);
    let map: Map = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let y = y as i32;
            line.chars().enumerate().map(move |(x, c)| {
                let x = x as i32;
                (
                    [x, y],
                    match c {
                        'S' => {
                            start.set([x, y]);
                            [[x - 1, y], [x, y + 1]]
                        }
                        '|' => [[x, y - 1], [x, y + 1]],
                        '-' => [[x - 1, y], [x + 1, y]],
                        'L' => [[x, y - 1], [x + 1, y]],
                        'J' => [[x - 1, y], [x, y - 1]],
                        '7' => [[x - 1, y], [x, y + 1]],
                        'F' => [[x + 1, y], [x, y + 1]],
                        _ => [[x, y], [x, y]],
                    },
                )
            })
        })
        .collect();

    let mut pipes = HashSet::<[i32; 2]>::new();
    let start = start.get();
    pipes.insert(start);
    let mut prev_a = start;
    let mut a = map[&prev_a][0];
    loop {
        pipes.insert(a);
        if a == start {
            break;
        }
        prev_a = walk(&map, a, prev_a);
        mem::swap(&mut prev_a, &mut a);
    }

    let verticals = pipes
        .iter()
        .filter(|&pipe| {
            let m = map[pipe];
            m[0][1] == pipe[1] - 1 || m[1][1] == pipe[1] - 1
        })
        .sorted_by_key(|&pipe| pipe[0])
        .into_group_map_by(|&pipe| pipe[1]);

    let sum: usize = verticals
        .into_iter()
        .map(|(y, scan)| {
            scan.into_iter()
                .tuples()
                .map(|(a, b)| (a[0]..b[0]).filter(|&x| !pipes.contains(&[x, y])).count())
                .sum::<usize>()
        })
        .sum();
    println!("{sum}");
}
