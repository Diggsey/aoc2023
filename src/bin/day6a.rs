const INPUT: &str = include_str!("../../inputs/day6.txt");

fn main() {
    let mut it = INPUT.lines();
    let times = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap());
    let distances = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap());
    let product: i32 = times
        .zip(distances)
        .map(|(t, d)| {
            let mp = (t as f64) / 2.0;
            let offset = ((t as f64).powi(2) / 4.0 - ((d + 1) as f64)).sqrt();
            let from = (mp - offset).ceil() as i32;
            let to = (mp + offset).floor() as i32;
            to + 1 - from
        })
        .product();
    println!("{}", product);
}
