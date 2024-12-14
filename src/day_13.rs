use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::{assert_equal, Itertools};

fn extended_euclid(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_euclid(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

fn extended_euclid_pos(a: usize, b: usize) -> (usize, isize, isize) {
    let (d, x, y) = extended_euclid(a as isize, b as isize);
    if d > 0 {
        return (d as usize, x, y);
    } else {
        return ((-d) as usize, -x, -y);
    }
}

fn lcm_pos(a: usize, b: usize) -> usize {
    let (d, _, _) = extended_euclid_pos(a, b);
    return (a * b) / d;
}

// Given ILP:
//  return minimize 3 x.0 + 1 x.1
//  s.t. x.0, x.1 >= 0
//       a.0 x.0 + b.0 x.0 = c.0 (1)
//       a.1 x.1 + b.1 x.1 = c.1 (2)
fn solve(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> Option<usize> {
    // x.1 = (c.0 - x.0 a.0) / b.0
    // x.1 = (c.1 - x.0 a.1) / b.1

    // pre-process a.0, b.0 and a.1, b.1 to be co-prime
    let (d0, d0a, _) = extended_euclid_pos(a.0, b.0);
    let (d1, d1a, _) = extended_euclid_pos(a.1, b.1);

    if d0 != 1 || d1 != 1 {
        if c.0 % d0 == 0 && c.1 % d1 == 0 {
            return solve((a.0 / d0, a.1 / d1), (b.0 / d0, b.1 / d1), (c.0 / d0, c.1 / d1));
        } else {
            return None;
        }
    }

    // x.0 in z.0 + b.0 Z
    let z0 = (d0a * (c.0 as isize)).rem_euclid(b.0 as isize);
    // x.0 in z.1 + b.1 Z
    let z1 = (d1a * (c.1 as isize)).rem_euclid(b.1 as isize);

    let (db, _, d1b) = extended_euclid(b.0 as isize, b.1 as isize);

    if (z1 - z0) % db != 0 {
        return None;
    }

    // x.0 = z + k p, k >= 0
    let p = (b.0 * b.1) / (db.abs() as usize);
    let z = (z1 - (b.1 as isize) * d1b * ((z1 - z0) / db)).rem_euclid(p as isize) as usize;

    let m1 = (b.1 * c.0) as isize - (b.0 * c.1) as isize;
    let m2 = (b.1 * a.0) as isize - (b.0 * a.1) as isize;

    if m1 % m2 != 0 {
        return None;
    }

    let m = m1 / m2 - z as isize;

    if m % (p as isize) != 0 {
        return None;
    }

    let k = m / (p as isize);

    let x0 = z as isize + k * p as isize;
    let x1 = (c.0 as isize - a.0 as isize * x0) / b.0 as isize;

    assert_eq!(a.0 as isize * x0 + b.0 as isize * x1, c.0 as isize);
    assert_eq!(a.1 as isize * x0 + b.1 as isize * x1, c.1 as isize);

    if x0 < 0 || x1 < 0 {
        return None;
    }

    return Some((3 * x0 + x1) as usize);
}

fn solve_data(data: File, prize_offset: usize) -> usize {
    let mut total = 0;

    for (l0, l1, l2) in BufReader::new(data).lines().flatten()
        .filter(|l| !l.is_empty()).tuples() {

        let a: (usize, usize) = l0.as_str().split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|val| val.split('+').nth(1).unwrap().parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();

        let b: (usize, usize) = l1.as_str().split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|val| val.split('+').nth(1).unwrap().parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();

        let c: (usize, usize) = l2.as_str().split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|val| val.split('=').nth(1).unwrap().parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();

        total += solve(a, b, (c.0 + prize_offset, c.1 + prize_offset)).unwrap_or(0);
    }

    return total;
}

pub fn part_1(data: File) -> usize {
    return solve_data(data, 0);
}

pub fn part_2(data: File) -> usize {
    return solve_data(data, 10000000000000);
}