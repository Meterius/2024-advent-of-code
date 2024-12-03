use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use itertools::Itertools;

fn is_safe(mut numbers: impl Iterator<Item=usize>) -> bool {
    if let Some((first, second)) = numbers.next_tuple() {
        let diff = second as isize - first as isize;
        if diff < -3 || diff > 3 || diff == 0 { return false; }
        return is_safe_directed(once(second).chain(numbers), diff > 0, 0);
    } else {
        return true;
    }
}

fn is_safe_directed(
    mut numbers: impl Iterator<Item=usize>,
    upwards: bool,
    max_mistakes: usize,
) -> bool {
    numbers.tuple_windows()
        .map(|(a, b)| (upwards && a < b && b <= a + 3) || (!upwards && b < a && a <= b + 3))
        .filter(|safe| !safe)
        .nth(max_mistakes)
        .is_none()
}

fn is_safe_except_one(mut numbers: impl Iterator<Item=usize>) -> bool {
    let numbers: Vec<usize> = numbers.collect();
    return (0..numbers.len()).any(
        |j| is_safe(
            numbers.iter().enumerate().filter(|(i, &x)| *i != j).map(|(_, &x)| x)
        )
    )
}

pub fn part_1(fs: File) -> usize {
    return BufReader::new(fs)
        .lines().flatten()
        .map(|line| is_safe(line.split_ascii_whitespace().map(|val| val.parse::<usize>().unwrap())))
        .fold(0, |prev, safe| if safe { prev + 1 } else { prev });
}

pub fn part_2(fs: File) -> usize {
    return BufReader::new(fs)
        .lines().flatten()
        .map(|line| is_safe_except_one(line.split_ascii_whitespace().map(|val| val.parse::<usize>().unwrap())))
        .fold(0, |prev, safe| if safe { prev + 1 } else { prev });
}