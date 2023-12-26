const INPUT: &str = include_str!("../../inputs/day14.txt");

enum Cell {
    Empty,
    RoundRock,
    SquareRock,
}

fn compute_weight(map: &Vec<Vec<Cell>>) -> u32 {
    let cols = map[0].len();
    let rows = map.len();
    let mut weight = 0;
    let mut positions = vec![0; cols];
    for y in 0..rows {
        for x in 0..cols {
            match map[y][x] {
                Cell::Empty => {}
                Cell::RoundRock => {
                    weight += (rows - positions[x]) as u32;
                    positions[x] += 1;
                }
                Cell::SquareRock => {
                    positions[x] = y + 1;
                }
            }
        }
    }
    weight
}

fn main() {
    let map: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    'O' => Cell::RoundRock,
                    '#' => Cell::SquareRock,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let weight = compute_weight(&map);

    println!("{weight}");
}
