use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

fn evolve(data: File, n: usize) -> usize {
    let mut stones = HashMap::new();

    BufReader::new(data)
        .lines()
        .flatten()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| { stones.entry(x).and_modify(|v| *v += 1).or_insert(1); });

    let mut stones_next = HashMap::new();

    for _ in 0..n {
        for (x, count) in stones.drain() {
            if x == 0 {
                stones_next.entry(1)
                    .and_modify(|v| *v += count).or_insert(count);
            } else {
                let digit_count = x.ilog10() + 1;

                if digit_count % 2 == 0 {
                    stones_next.entry(x / 10usize.pow(digit_count / 2))
                        .and_modify(|v| *v += count).or_insert(count);

                    stones_next.entry(x % 10usize.pow(digit_count / 2))
                        .and_modify(|v| *v += count).or_insert(count);
                } else {
                    stones_next.entry(x * 2024)
                        .and_modify(|v| *v += count).or_insert(count);
                }
            }
        }

        swap(&mut stones, &mut stones_next);
    }

    return stones.values().sum::<usize>();
}

pub fn part_1(data: File) -> usize {
    return evolve(data, 25);
}

pub fn part_2(data: File) -> usize {
    return evolve(data, 75);
}