use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day24.txt");

type Vec3 = (i128, i128, i128);

#[derive(Debug, Copy, Clone)]
struct Line {
    p: Vec3,
    v: Vec3,
}

fn parse_vec3(s: &str) -> Vec3 {
    s.split(", ")
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn intersection(a: &Line, b: &Line) -> Option<Vec3> {
    let ad = (a.v.0, a.v.1);
    let bd = (b.v.0, b.v.1);
    let den = ad.0 * bd.1 - ad.1 * bd.0;
    if den != 0 {
        let a0 = (a.p.0, a.p.1);
        let a1 = (a.p.0 + ad.0, a.p.1 + ad.1);
        let b0 = (b.p.0, b.p.1);
        let b1 = (b.p.0 + bd.0, b.p.1 + bd.1);

        let at = (b0.0 - a0.0) * bd.1 - (b0.1 - a0.1) * bd.0;
        let bt = (b0.0 - a0.0) * ad.1 - (b0.1 - a0.1) * ad.0;

        if at / den >= 0 && bt / den >= 0 {
            let af = a0.0 * a1.1 - a0.1 * a1.0;
            let bf = b0.0 * b1.1 - b0.1 * b1.0;
            let x = bf * ad.0 - af * bd.0;
            let y = bf * ad.1 - af * bd.1;
            Some((x / den, y / den, 0))
        } else {
            None
        }
    } else {
        None
    }
}

fn main() {
    let lines = INPUT
        .lines()
        .map(|line| {
            let (p, v) = line.split(" @ ").map(parse_vec3).collect_tuple().unwrap();
            Line { p, v }
        })
        .collect_vec();

    let intersection_region = 200000000000000..=400000000000000;
    let count = lines
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            intersection(a, b).is_some_and(|pos| {
                intersection_region.contains(&pos.0) && intersection_region.contains(&pos.1)
            })
        })
        .count();
    println!("{count}");
}
