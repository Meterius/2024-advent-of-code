use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use itertools::Itertools;

pub fn part_1(data: File) -> usize {
    let mut total = 0;

    let mut lines = BufReader::new(data).lines().flatten();

    let mut forbidden = HashSet::new();

    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        let (x, y) = line.as_str()
            .split("|").map(|val| u32::from_str(val).unwrap())
            .next_tuple().unwrap();

        forbidden.insert((y, x));
    }

    for line in lines {
        let xs = line
            .split(",")
            .map(|val| u32::from_str(val).unwrap())
            .collect_vec();

        let med = xs[xs.len() / 2];

        if xs.into_iter().tuple_windows().all(|(x, y)| !forbidden.contains(&(x, y))) {
            total += med as usize;
        }
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    return 0;
}