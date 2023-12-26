use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day18.txt");

struct Op {
    dir: &'static str,
    dist: i64,
}

#[derive(Clone, Debug)]
struct Block {
    extent: i64,
    verticals: BTreeSet<i64>,
}

fn main() {
    let ops: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let (_dir, _dist, color) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let dist: i64 = i64::from_str_radix(&color[2..7], 16).unwrap();
            let dir = &color[7..8];
            Op { dir, dist }
        })
        .collect();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut trench = BTreeMap::<i64, Block>::new();
    let mut was_left_turn = false;
    for (prev_op, next_op) in ops.iter().circular_tuple_windows() {
        let (is_right_turn, is_left_turn) = match (prev_op.dir, next_op.dir) {
            ("3", "0") | ("0", "1") | ("1", "2") | ("2", "3") => (true, false),
            ("0", "3") | ("1", "0") | ("2", "1") | ("3", "2") => (false, true),
            _ => (false, false),
        };
        let mut dist = prev_op.dist;
        if is_right_turn {
            dist += 1;
        }
        if was_left_turn {
            dist -= 1;
        }
        was_left_turn = is_left_turn;
        let prev_y = y;
        match prev_op.dir {
            "3" => y -= dist,
            "1" => y += dist,
            "2" => x -= dist,
            "0" => x += dist,
            _ => unreachable!(),
        }
        if y != prev_y {
            let min_y = y.min(prev_y);
            let mut max_y = y.max(prev_y);
            let mut new_blocks = Vec::new();
            for (&cur_y, block) in trench.range_mut(..max_y).rev() {
                if block.extent <= min_y {
                    break;
                }
                if block.extent > max_y {
                    new_blocks.push((max_y, block.clone()));
                    block.extent = max_y;
                } else if max_y > block.extent {
                    let mut verticals = BTreeSet::new();
                    verticals.insert(x);
                    new_blocks.push((
                        block.extent,
                        Block {
                            extent: max_y,
                            verticals,
                        },
                    ));
                    max_y = block.extent;
                }

                if cur_y < min_y {
                    let mut verticals = block.verticals.clone();
                    verticals.insert(x);
                    new_blocks.push((
                        min_y,
                        Block {
                            extent: max_y,
                            verticals,
                        },
                    ));
                    block.extent = min_y;
                    max_y = min_y;
                    break;
                } else {
                    block.verticals.insert(x);
                    max_y = cur_y;
                }
            }
            if max_y > min_y {
                let mut verticals = BTreeSet::new();
                verticals.insert(x);
                new_blocks.push((
                    min_y,
                    Block {
                        extent: max_y,
                        verticals,
                    },
                ));
            }
            trench.extend(new_blocks);
        }
    }
    let area: i64 = trench
        .iter()
        .map(|(&y, block)| {
            block
                .verticals
                .iter()
                .tuples()
                .map(|(&x0, &x1)| x1 - x0)
                .sum::<i64>()
                * (block.extent - y)
        })
        .sum::<i64>();

    println!("{area}");
}
