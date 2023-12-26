const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let sum: i32 = INPUT
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Game ").unwrap();
            let (game_no, sets) = line.split_once(": ").unwrap();
            let game_no = game_no.parse::<i32>().unwrap();
            let is_possible = sets
                .split("; ")
                .map(|subset| {
                    subset
                        .split(", ")
                        .map(|sample| {
                            let (count, color) = sample.split_once(' ').unwrap();
                            (color, count.parse::<i32>().unwrap())
                        })
                        .collect::<Vec<_>>()
                })
                .all(|subset| {
                    subset.iter().all(|&(color, count)| {
                        count
                            <= match color {
                                "red" => 12,
                                "green" => 13,
                                "blue" => 14,
                                _ => unreachable!(),
                            }
                    })
                });
            if is_possible {
                game_no
            } else {
                0
            }
        })
        .sum();
    println!("{}", sum);
}
