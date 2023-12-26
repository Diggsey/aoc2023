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
    let mut boxes = vec![Vec::<(&str, u64)>::new(); 256];
    for instruction in INPUT.trim().split(',') {
        if let Some((label, focal_length)) = instruction.split_once('=') {
            let focal_length = focal_length.parse().unwrap();
            let hash = compute_hash(label);
            let box_ = &mut boxes[hash as usize];
            if let Some(slot) = box_.iter_mut().find(|(l, _)| *l == label) {
                slot.1 = focal_length;
            } else {
                box_.push((label, focal_length));
            }
        } else {
            let label = instruction.strip_suffix('-').unwrap();
            let hash = compute_hash(label);
            let box_ = &mut boxes[hash as usize];
            if let Some(idx) = box_.iter().position(|(l, _)| *l == label) {
                box_.remove(idx);
            }
        }
    }
    let sum: u64 = boxes
        .iter()
        .enumerate()
        .map(|(i, box_)| {
            box_.iter()
                .enumerate()
                .map(|(j, slot)| (j as u64 + 1) * slot.1)
                .sum::<u64>()
                * (i as u64 + 1)
        })
        .sum();

    println!("{sum}");
}
