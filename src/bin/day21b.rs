use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day21.txt");

fn compute_locations(
    cache: &mut HashMap<([i32; 2], usize), u64>,
    map: &Vec<Vec<bool>>,
    pos: [i32; 2],
    steps: usize,
) -> u64 {
    *cache.entry((pos, steps)).or_insert_with(|| {
        let mut positions = HashSet::new();
        positions.insert(pos);

        for _ in 0..steps {
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

        positions.len() as u64
    })
}

fn main() {
    let mut cache = HashMap::new();
    let map = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();
    let s = map.len() as i32;
    let steps = 26501365;
    let max_square_dist = (steps / s) + 1;
    let mut total: u64 = 0;

    let odd_total = compute_locations(&mut cache, &map, [s / 2, s / 2], (s * 2 + 1) as usize);
    let even_total = compute_locations(&mut cache, &map, [s / 2, s / 2], (s * 2) as usize);

    for y in -max_square_dist..=max_square_dist {
        let ydist = (y.abs() * s - s / 2).max(0);
        let xsteps = steps - ydist;
        let min_x_dist = xsteps / s;
        let max_x_dist = min_x_dist + 1;
        if min_x_dist > 0 {
            let count = if steps % 2 != (y.abs() + min_x_dist) % 2 {
                even_total * (min_x_dist as u64) + odd_total * (min_x_dist as u64 - 1)
            } else {
                odd_total * (min_x_dist as u64) + even_total * (min_x_dist as u64 - 1)
            };
            total += count;
        }
        let xvalues: HashSet<_> = (-max_x_dist..=(-min_x_dist))
            .chain(min_x_dist..=max_x_dist)
            .collect();
        for x in xvalues {
            let xdist = (x.abs() * s - s / 2).max(0);
            let remaining_steps = xsteps - xdist;
            if remaining_steps < 0 {
                continue;
            }
            let xstart = (s - 1) / 2 * (1 - x.signum());
            let ystart = (s - 1) / 2 * (1 - y.signum());

            let count =
                compute_locations(&mut cache, &map, [xstart, ystart], remaining_steps as usize);

            total += count;
        }
    }
    println!("{total}");
}
