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
    let mut seq_value = vec![0; 20usize.pow(4)];
    let mut found = vec![false; 20usize.pow(4)];

    for line in BufReader::new(data).lines().flatten() {
        let x = line.parse::<usize>().unwrap();

        found.as_mut_slice().fill(false);
        
        let mut change = 0;

        let mut curr = x;
        for _ in 0..3 {
            let mut next = evolve(curr);
            change = (change * 20 + (10 + next % 10 - curr % 10)) % (20usize.pow(4));
            curr = next;
        }
        
        for _ in 0..2000 - 3 {
            let mut next = evolve(curr);
            change = (change * 20 + (10 + next % 10 - curr % 10)) % (20usize.pow(4));

            if !found[change] {
                found[change] = true;
                seq_value[change] += next % 10;
            }
            
            curr = next;
        }
    }
    
    return seq_value.into_iter().max().unwrap_or(0);
}