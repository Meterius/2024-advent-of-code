use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn solve(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> Option<usize> {
    let m1 = (b.0 * c.1) as isize - (b.1 * c.0) as isize;
    let m2 = (b.0 * a.1) as isize - (b.1 * a.0) as isize;
    
    assert_ne!(m2, 0);
    
    if m1 % m2 != 0 {
        return None;
    }
    
    let x0 = m1 / m2;
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