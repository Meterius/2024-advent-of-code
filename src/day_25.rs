use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WIDTH: usize = 5;
const HEIGHT: usize = 7;

pub fn part_1(data: File) -> usize {
    let mut lines = BufReader::new(data).lines().flatten();

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    while let Some(first_line) = lines.next() {
        assert_eq!(first_line.len(), WIDTH);

        let mut is_key = first_line.chars().next().unwrap() == '#';

        let mut item = [0; WIDTH];
        let mut height = 1;

        while let Some(line) = lines.next() {
            if line.is_empty() { break; }

            assert!(height <= HEIGHT);

            if is_key {
                for (i, c) in line.chars().enumerate() {
                    if c == '.' && item[i] == 0 {
                        item[i] = height;
                    }
                }
            } else {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' && item[i] == 0 {
                        item[i] = HEIGHT - height;
                    }
                }
            }

            height += 1;
        }

        assert!(item.iter().all(|&x| x != 0));

        if is_key {
            keys.push(item);
        } else {
            locks.push(item);
        }
    };

    let mut total = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.iter().zip(lock.iter()).all(|(&i, &j)| i + j <= HEIGHT) {
                total += 1;
            }
        }
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    return 0;
}