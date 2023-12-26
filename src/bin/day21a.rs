use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day21.txt");

fn main() {
    let mut positions = HashSet::new();
    let map = INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        positions.insert([x as i32, y as i32]);
                    }
                    c == '#'
                })
                .collect_vec()
        })
        .collect_vec();

    for _ in 0..64 {
        let mut new_positions = HashSet::new();
        for [x, y] in positions {
            for [dx, dy] in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= map[0].len() as i32 || ny >= map.len() as i32 {
                    continue;
                }
                if map[ny as usize][nx as usize] {
                    continue;
                }
                new_positions.insert([nx, ny]);
            }
        }
        positions = new_positions;
    }

    println!("{}", positions.len());
}
