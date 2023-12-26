use std::collections::{HashMap, HashSet, VecDeque};

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

fn compute_fall_count(supports: &HashMap<usize, HashSet<usize>>, i: usize) -> usize {
    let mut support_counts = HashMap::<usize, usize>::new();
    for js in supports.values() {
        for &j in js {
            *support_counts.entry(j).or_default() += 1;
        }
    }
    let mut count = 0;
    let mut bricks_to_remove = VecDeque::new();
    bricks_to_remove.push_back(i);
    while let Some(brick) = bricks_to_remove.pop_front() {
        count += 1;
        if let Some(support) = supports.get(&brick) {
            for &j in support {
                let support_count = support_counts.get_mut(&j).unwrap();
                *support_count -= 1;
                if *support_count == 0 {
                    bricks_to_remove.push_back(j);
                }
            }
        }
    }
    count - 1
}

fn main() {
    let mut supports = HashMap::<usize, HashSet<_>>::new();
    let mut heights = HashMap::<(i32, i32), (usize, i32)>::new();
    for (i, (a, b)) in INPUT
        .lines()
        .map(|line| line.split('~').map(parse_coord).collect_tuple().unwrap())
        .enumerate()
        .sorted_by_key(|(_, (a, _))| a[2])
    {
        let xy_coords = (a[0]..=b[0]).cartesian_product(a[1]..=b[1]).collect_vec();
        let supporting_blocks = xy_coords
            .iter()
            .flat_map(|coord| heights.get(coord).copied())
            .max_set_by_key(|&(_, z)| z);

        let mut max_z = 0;
        for (j, z) in supporting_blocks {
            max_z = z;
            supports.entry(j).or_default().insert(i);
        }

        let h = b[2] + 1 - a[2];
        for coord in xy_coords {
            heights.insert(coord, (i, h + max_z));
        }
    }

    let sum: usize = supports
        .keys()
        .copied()
        .map(|i| compute_fall_count(&supports, i))
        .sum();
    println!("{sum}");
}
