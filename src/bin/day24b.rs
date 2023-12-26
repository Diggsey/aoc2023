use std::ops::Sub;

use itertools::Itertools;
use num::integer::gcd;

const INPUT: &str = include_str!("../../inputs/day24.txt");

type ElemType = i128;
type Vector3 = nalgebra::Vector3<ElemType>;

#[derive(Debug, Copy, Clone)]
struct Line {
    p: Vector3,
    v: Vector3,
}

impl Sub for &Line {
    type Output = Line;

    fn sub(self, rhs: Self) -> Self::Output {
        Line {
            p: self.p - rhs.p,
            v: self.v - rhs.v,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Plane {
    n: Vector3,
    d: ElemType,
}

impl Plane {
    fn intersection(a: &Self, b: &Self, c: &Self) -> Vector3 {
        let denom = a.n.dot(&b.n.cross(&c.n));
        (b.n.cross(&c.n) * a.d + c.n.cross(&a.n) * b.d + a.n.cross(&b.n) * c.d) / denom
    }
    fn reduce(&mut self) {
        // The numbers get very big so reduce them down by dividing
        // top and bottom by the GCD of both
        let gcd = gcd(gcd(gcd(self.d, self.n.x), self.n.y), self.n.z);
        self.n /= gcd;
        self.d /= gcd;
    }
}

fn parse_vec3(s: &str) -> Vector3 {
    Vector3::from_iterator(s.split(", ").map(|s| s.parse().unwrap()))
}

// Given two hailstone rays, find the set of velocities for a rock that
// would allow hitting both hailstones.
fn find_rock_velocities(a: &Line, b: &Line) -> Plane {
    // Change frame of reference so that hailstone 'b' is stationary at the
    // origin, and only 'a' is moving relative to 'b'.
    let a_rel = a - b;

    // We have three points of interest: the the position of hailstone 'b'
    // (the origin) the relative position of hailstone 'a', and the relative
    // position of hailstone 'a' after it's moved one step. For a rock to hit both
    // hailstones it can only be moving in the plane formed by those three
    // points.
    // The normal of this plane is perpendicular to any two vectors between
    // those points:
    let plane_normal = a_rel.p.cross(&a_rel.v);

    // We need to change coordinates back to absolute coordinates. The plane
    // will have the same orientation but won't necessarily go through the
    // origin anymore.
    // We can calculate the plane constant by picking any point we know is
    // on the plane and evaluating the plane constant at that point.

    // A rock velocity which starts at 'b' and hits 'a' after one tick is
    // easy to calculate and definitely on the plane:
    let sample_v = a_rel.p + a.v;
    let plane_constant = sample_v.dot(&plane_normal);

    let mut plane = Plane {
        n: plane_normal,
        d: plane_constant,
    };

    // Make the numbers more manageable
    plane.reduce();

    return plane;
}

fn main() {
    let lines = INPUT
        .lines()
        .map(|line| {
            let (p, v) = line.split(" @ ").map(parse_vec3).collect_tuple().unwrap();
            Line { p, v }
        })
        .collect_vec();

    // First compute the rock velocity

    // The rock velocity has three unknowns (since it's 3D), but each pair of
    // independent hailstone rays eliminates a single unknown: ie. the velocity solutions
    // for a single pair lie on a plane.
    // Use the three combinations of the first three rays to form three planes.
    let a = find_rock_velocities(&lines[0], &lines[1]);
    let b = find_rock_velocities(&lines[0], &lines[2]);
    let c = find_rock_velocities(&lines[1], &lines[2]);

    // Compute the intersection of the three planes to find the single velocity
    // which is a solution to all three equations.
    let rock_vel = Plane::intersection(&a, &b, &c);

    // Now we know the rock velocity, we can work backwards to find the rock's starting
    // position. We do this by pretending that the rock is stationary and the velocity
    // of all the hailstones has been adjusted by `-rock_vel`. Then we find out where
    // all the adjusted lines intersect (we only need two for this). This intersection
    // position must be where the rock started.
    let lr = Line {
        p: Vector3::zeros(),
        v: rock_vel,
    };
    let la = &lines[0] - &lr;
    let lb = &lines[1] - &lr;

    // Line intersection algorithm
    // Find out how far along 'la' we hit 'lb' by taking the difference between
    // points on the two lines (lb.p - la.p) and dividing by how quickly 'la'
    // approaches 'lb'. We can effectively "cancel out" the impact of 'lb.v'
    // by taking its cross product with both sides since `lb.v x lb.v = 0`.
    let num = (lb.p - la.p).cross(&lb.v);
    let den = la.v.cross(&lb.v);

    // The two vectors should be parallel, so it shouldn't matter which
    // non-zero component we take to do the division.
    let t0 = num.x / den.x;

    // Finally calculate the rock position
    let rock_pos = la.p + la.v * t0;

    let sum = rock_pos.x + rock_pos.y + rock_pos.z;
    println!("{sum}");
}
