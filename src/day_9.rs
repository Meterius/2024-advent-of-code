use std::array::from_fn;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BTreeSet};

pub fn part_1(data: File) -> usize {
    let line = BufReader::new(data).lines().flatten().next().unwrap();
    let mut disk = Vec::with_capacity(line.len() * 9);

    for (i, c) in line.chars().enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            disk.push(if i % 2 == 0 { 1 + i / 2 } else { 0 });
        }
    }

    let mut i = 0;
    for j in (0..disk.len()).rev() {
        if disk[j] == 0 {
            continue;
        }

        while i < j && disk[i] != 0 {
            i += 1;
        }

        if i >= j {
            break;
        }

        disk.swap(i, j);

        i += 1;
    }

    return disk.iter()
        .take_while(|&&x| x != 0)
        .enumerate()
        .map(|(i, &x)| i * (x - 1))
        .sum::<usize>();
}

pub fn part_2(data: File) -> usize {
    let line = BufReader::new(data).lines().flatten().next().unwrap();

    let mut free: [_; 9] = from_fn(|_| BTreeSet::<usize>::new());
    let mut files = Vec::with_capacity(line.len() / 2);

    let mut disk_len = 0;
    for (i, c) in line.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;
        
        if size > 0 {
            if i % 2 != 0 {
                free[size - 1].insert(disk_len);
            } else {
                files.push((disk_len, size));
            }
        }

        disk_len += size;
    }

    let mut checksum = 0;
    
    for (id, (entry_j, entry_size)) in files.into_iter().enumerate().rev() {
        let free_entry = ((entry_size - 1)..9)
            .map(|s| free[s].first().cloned().map(|i| (s + 1, i)))
            .flatten()
            .min_by_key(|(_, i)| *i);

        if let Some((free_size, free_i)) = free_entry {
            if free_i < entry_j {
                free[free_size - 1].remove(&free_i);

                checksum += id * (free_i * entry_size + (entry_size * (entry_size - 1)) / 2);

                if free_size > entry_size {
                    free[free_size - entry_size - 1].insert(free_i + entry_size);
                }
            } else {
                checksum += id * (entry_j * entry_size + (entry_size * (entry_size - 1)) / 2);
            }
        } else {
            checksum += id * (entry_j * entry_size + (entry_size * (entry_size - 1)) / 2);
        }
    }
    
    return checksum;
}
