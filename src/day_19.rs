use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(data: File) -> usize {
    let mut lines = BufReader::new(data).lines().flatten();
    
    let patterns = lines.next().unwrap().split(", ")
        .map(|s| String::from(s)).collect::<Vec<_>>();
    
    let mut total = 0;
    for line in lines.skip(1) {
        let towel = line.as_str();
        
        let mut prefix_valid = vec![false; towel.len()];
        prefix_valid[0] = true;
        
        'check: for i in 0..towel.len() {
            if prefix_valid[i] {
                for pattern in patterns.iter().filter(|p| i + p.len() <= towel.len()) {
                    if pattern[..] == towel[i..i+pattern.len()] {
                        if i + pattern.len() == towel.len() {
                            total += 1;
                            break 'check;
                        } else {
                            prefix_valid[i+pattern.len()] = true;
                        }
                    }
                }
            }
        }
    }
    
    return total;
}

pub fn part_2(data: File) -> usize {
    let mut lines = BufReader::new(data).lines().flatten();

    let patterns = lines.next().unwrap().split(", ")
        .map(|s| String::from(s)).collect::<Vec<_>>();

    let mut total = 0;
    for line in lines.skip(1) {
        let towel = line.as_str();

        let mut prefix_paths = vec![0; towel.len() + 1];
        prefix_paths[0] = 1;

        for i in 0..towel.len() {
            if prefix_paths[i] > 0 {
                for pattern in patterns.iter().filter(|p| i + p.len() <= towel.len()) {
                    if pattern[..] == towel[i..i+pattern.len()] {
                        prefix_paths[i+pattern.len()] += prefix_paths[i];
                    }
                }
            }
        }
        
        total += prefix_paths[towel.len()];
    }

    return total;
}