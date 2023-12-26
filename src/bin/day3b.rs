use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let number_re = Regex::new("[0-9]+").unwrap();
    let mut gears: HashMap<_, _> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.match_indices('*')
                .map(move |(x, _)| ((y as i32, x as i32), Vec::new()))
        })
        .collect();
    for (y, x0, x1, v) in INPUT.lines().enumerate().flat_map(|(y, line)| {
        number_re.find_iter(line).map(move |m| {
            (
                y as i32,
                m.start() as i32,
                m.end() as i32,
                m.as_str().parse::<i32>().unwrap(),
            )
        })
    }) {
        for p in (y - 1..=y + 1).flat_map(|cy| (x0 - 1..x1 + 1).map(move |cx| (cy, cx))) {
            if let Some(vec) = gears.get_mut(&p) {
                vec.push(v);
            }
        }
    }
    let sum: i32 = gears
        .values()
        .filter(|vec| vec.len() == 2)
        .map(|vec| vec.iter().copied().product::<i32>())
        .sum();
    println!("{}", sum);
}
