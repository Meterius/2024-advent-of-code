use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

pub fn part_1(data: File) -> usize {
    let mut total = 0;

    for line in BufReader::new(data).lines().flatten() {
        let (y, xs) = line.split_once(':').unwrap();

        let y: usize = y.parse().unwrap();
        let xs: Vec<usize> = xs.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut prev_targets = HashSet::new();
        prev_targets.insert(0);

        let mut next_targets = HashSet::new();

        for x in xs.into_iter() {
            for prev in prev_targets.drain() {
                let (a, b) = (prev + x, prev * x);

                if a <= y {
                    next_targets.insert(a);
                }

                if b <= y {
                    next_targets.insert(b);
                }
            }

            swap(&mut prev_targets, &mut next_targets);
        }

        if prev_targets.contains(&y) {
            total += y;
        }
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    let mut total = 0;

    for line in BufReader::new(data).lines().flatten() {
        let (y, xs) = line.split_once(':').unwrap();

        let y: usize = y.parse().unwrap();
        let xs: Vec<usize> = xs.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut prev_targets = HashSet::new();
        prev_targets.insert(0);

        let mut next_targets = HashSet::new();

        for x in xs.into_iter() {
            for prev in prev_targets.drain() {
                let (a, b, c) = (prev + x, prev * x, prev * 10usize.pow(x.ilog10() + 1) + x);

                if a <= y {
                    next_targets.insert(a);
                }

                if b <= y {
                    next_targets.insert(b);
                }

                if c <= y {
                    next_targets.insert(c);
                }
            }

            swap(&mut prev_targets, &mut next_targets);
        }

        if prev_targets.contains(&y) {
            total += y;
        }
    }

    return total;
}