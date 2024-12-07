use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

const COLLECT_STATS: bool = false;

pub fn part_1(data: File) -> usize {
    let mut total = 0;

    let mut total_combs = 0;
    let mut total_redundants = 0;

    for line in BufReader::new(data).lines().flatten() {
        let (y, xs) = line.split_once(':').unwrap();

        let y: usize = y.parse().unwrap();

        let mut xs = xs.split_whitespace().map(|x| x.parse().unwrap());
        let (x1, xs) = (xs.next().unwrap(), xs.collect::<Vec<usize>>());

        let mut prev_targets = Vec::new();
        prev_targets.push(y);

        let mut next_targets = Vec::new();

        for x in xs.into_iter().rev() {
            for prev in prev_targets.drain(..).filter(|&prev| prev >= x1) {
                if prev >= x {
                    next_targets.push(prev - x);
                }

                if prev % x == 0 {
                    next_targets.push(prev / x);
                }
            }

            if COLLECT_STATS {
                total_combs += next_targets.len();
                let test: HashSet::<usize> = HashSet::from_iter(next_targets.iter().cloned());
                total_redundants += next_targets.len() - test.len();
            }

            swap(&mut prev_targets, &mut next_targets);
        }

        if prev_targets.contains(&x1) {
            total += y;
        }
    }

    // - required 919922 combinations had 89535 redundants with left-to-right with pruning via exceeding y
    // - required 23296 combinations had 740 redundants with right-to-left with pruning via going below first value (i.e. exceeding y) and
    //   requiring divisibility for multiplication

    if COLLECT_STATS {
        println!("Total Combintations: {total_combs}; Total Redundant: {total_redundants}");
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    let mut total = 0;

    let mut total_combs = 0;
    let mut total_redundants = 0;

    for line in BufReader::new(data).lines().flatten() {
        let (y, xs) = line.split_once(':').unwrap();

        let y: usize = y.parse().unwrap();

        let mut xs = xs.split_whitespace().map(|x| x.parse().unwrap());
        let (x1, xs) = (xs.next().unwrap(), xs.collect::<Vec<usize>>());

        let mut prev_targets = Vec::new();
        prev_targets.push(y);

        let mut next_targets = Vec::new();

        for x in xs.into_iter().rev() {
            for prev in prev_targets.drain(..).filter(|&prev| prev >= x1) {
                if prev >= x {
                    next_targets.push(prev - x);
                }

                if prev % x == 0 {
                    next_targets.push(prev / x);
                }

                let m = 10usize.pow(x.ilog10() + 1);
                if prev % m == x {
                    next_targets.push(prev / m);
                }
            }

            if COLLECT_STATS {
                total_combs += next_targets.len();
                let test: HashSet::<usize> = HashSet::from_iter(next_targets.iter().cloned());
                total_redundants += next_targets.len() - test.len();
            }

            swap(&mut prev_targets, &mut next_targets);
        }

        if prev_targets.contains(&x1) {
            total += y;
        }
    }

    // - required 34053963 combintations had 18823897 redundants with left-to-right with pruning via exceeding y
    // - required 35117 combintations had 1126 redundants with right-to-left with pruning via going below first value (i.e. exceeding y),
    //   requiring divisibility for multiplication, requiring digit match for concatenation

    if COLLECT_STATS {
        println!("Total Combintations: {total_combs}; Total Redundant: {total_redundants}");
    }

    return total;
}