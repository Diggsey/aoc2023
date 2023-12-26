use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day6.txt");

fn main() {
    let mut it = INPUT.lines();
    let t = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<i64>()
        .unwrap();
    let d = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<i64>()
        .unwrap();
    let mp = (t as f64) / 2.0;
    let offset = ((t as f64).powi(2) / 4.0 - ((d + 1) as f64)).sqrt();
    let from = (mp - offset).ceil() as i64;
    let to = (mp + offset).floor() as i64;

    println!("{}", to + 1 - from);
}
