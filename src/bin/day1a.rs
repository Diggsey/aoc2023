const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let sum: i32 = INPUT
        .lines()
        .map(|line| {
            let mut it = line.chars().filter(|c| c.is_ascii_digit());
            let first = it.next().unwrap();
            let last = it.next_back().unwrap_or(first);
            format!("{first}{last}").parse::<i32>().unwrap()
        })
        .sum();
    println!("{}", sum);
}
