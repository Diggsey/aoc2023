use std::collections::BTreeSet;

const INPUT: &str = include_str!("../../inputs/day11.txt");

fn main() {
    let map: Vec<_> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let y = y as i64;
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| {
                    let x = x as i64;
                    [x, y]
                })
        })
        .collect();

    let populated_columns: BTreeSet<_> = map.iter().map(|x| x[0]).collect();
    let populated_rows: BTreeSet<_> = map.iter().map(|x| x[1]).collect();

    let mut dist = 0;
    for i in 0..map.len() - 1 {
        for j in (i + 1)..map.len() {
            let a = map[i];
            let b = map[j];
            let xmin = a[0].min(b[0]);
            let xmax = a[0].max(b[0]);
            let ymin = a[1].min(b[1]);
            let ymax = a[1].max(b[1]);

            let base_dist = xmax + ymax - xmin - ymin;
            let non_expanding_cols = populated_columns.range(xmin..xmax).count() as i64;
            let non_expanding_rows = populated_rows.range(ymin..ymax).count() as i64;
            dist += base_dist * 1000000 - non_expanding_cols * 999999 - non_expanding_rows * 999999;
        }
    }
    println!("{dist}");
}
