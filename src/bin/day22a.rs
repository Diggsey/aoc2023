use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day22.txt");

fn parse_coord(s: &str) -> [i32; 3] {
    let (x, y, z) = s
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect_tuple()
        .unwrap();
    [x, y, z]
}

fn main() {
    let mut crucial_bricks = HashSet::new();
    let mut heights = HashMap::<(i32, i32), (usize, i32)>::new();
    let mut total_bricks = 0;
    for (i, (a, b)) in INPUT
        .lines()
        .map(|line| line.split('~').map(parse_coord).collect_tuple().unwrap())
        .enumerate()
        .sorted_by_key(|(_, (a, _))| a[2])
    {
        total_bricks += 1;
        let xy_coords = (a[0]..=b[0]).cartesian_product(a[1]..=b[1]).collect_vec();
        let supporting_blocks = xy_coords
            .iter()
            .flat_map(|coord| heights.get(coord).copied())
            .sorted()
            .dedup()
            .max_set_by_key(|&(_, z)| z);

        let max_z = if let Some(&(j, z)) = supporting_blocks.first() {
            if supporting_blocks.len() == 1 {
                crucial_bricks.insert(j);
            }
            z
        } else {
            0
        };

        let h = b[2] + 1 - a[2];
        for coord in xy_coords {
            heights.insert(coord, (i, h + max_z));
        }
    }

    let safe_to_remove_bricks = total_bricks - crucial_bricks.len();
    println!("{safe_to_remove_bricks}");
}
