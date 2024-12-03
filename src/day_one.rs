use std::{fs, io};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use crate::common::OrderedCounter;

pub fn part_1(data: fs::File) -> usize {
    let mut left = OrderedCounter::<usize>::new();
    let mut right = OrderedCounter::<usize>::new();

    for line in io::BufReader::new(data).lines().flatten() {
        let mut numbers = line.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap());
        left.increment(numbers.next().unwrap());
        right.increment(numbers.next().unwrap());
    }

    let mut total = 0;
    while let Some(left_val) = left.pop_first() {
        total += left_val.abs_diff(right.pop_first().unwrap());
    }

    return total;
}

pub fn part_2(data: fs::File) -> usize {
    let mut left = HashSet::<usize>::new();
    let mut right = HashMap::<usize, usize>::new();

    for line in io::BufReader::new(data).lines().flatten() {
        let mut numbers = line.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap());
        left.insert(numbers.next().unwrap());
        right.entry(numbers.next().unwrap()).and_modify(|v| { *v += 1 }).or_insert(1);
    }

    return left.into_iter().map(|x| x * right.get(&x).cloned().unwrap_or(0)).sum();
}