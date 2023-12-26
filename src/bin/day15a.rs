const INPUT: &str = include_str!("../../inputs/day15.txt");

fn compute_hash(s: &str) -> u32 {
    let mut hash = 0;
    for &b in s.as_bytes() {
        hash += b as u32;
        hash = (hash * 17) % 256;
    }
    hash
}

fn main() {
    let sum: u32 = INPUT.trim().split(',').map(compute_hash).sum();

    println!("{sum}");
}
