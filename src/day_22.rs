use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn evolve(x: usize) -> usize {
    let s1 = ((x * 64) ^ x) % 16777216;
    let s2 = ((s1 / 32) ^ s1) % 16777216;
    let s3 = ((s2 * 2048) ^ s2) % 16777216;
    return s3;
}

pub fn part_1(data: File) -> usize {
    let mut total = 0;

    for line in BufReader::new(data).lines().flatten() {
        let x = line.parse::<usize>().unwrap();
        let res = (0..2000).fold(x, |acc, _| evolve(acc));
        total += res;
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    let mut seq_value = HashMap::new();

    for line in BufReader::new(data).lines().flatten() {
        let x = line.parse::<usize>().unwrap();

        let mut found = HashSet::new();
        let mut changes = VecDeque::with_capacity(4);

        let mut curr = x;
        for _ in 0..2000 {
            let mut next = evolve(curr);

            if changes.len() == 4 {
                changes.pop_front();
            }
            changes.push_back((next % 10) as isize - (curr % 10) as isize);

            if changes.len() == 4 {
                let mut seq = [0; 4];
                seq.iter_mut().set_from(changes.iter().cloned());
                
                if found.insert(seq.clone()) {
                    seq_value.entry(seq)
                        .and_modify(|v| { *v += next % 10; })
                        .or_insert(next % 10);
                }
            }

            curr = next;
        }
    }
    
    return seq_value.values().cloned().max().unwrap_or(0);
}