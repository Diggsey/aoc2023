use std::{cell::Cell, collections::HashMap, mem};

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

    let start = start.get();
    let [mut prev_a, mut prev_b] = [start, start];
    let [mut a, mut b] = map[&start];
    for i in 1.. {
        if a == b {
            println!("{i}");
            break;
        }
        prev_a = walk(&map, a, prev_a);
        prev_b = walk(&map, b, prev_b);
        mem::swap(&mut prev_a, &mut a);
        mem::swap(&mut prev_b, &mut b);
    }
}
