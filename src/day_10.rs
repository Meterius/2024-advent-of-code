use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::common::Matrix;

pub fn part_1(data: File) -> usize {
    let matrix = Matrix::from_lines(
        BufReader::new(data).lines().flatten(),
        |c| c.to_digit(10).unwrap() as usize
    );

    let mut total = 0;

    for i in 0..matrix.buffer.len() {
        if matrix.buffer[i] == 0 {
            let mut front = HashSet::new();
            front.insert(matrix.index_to_point(i).unwrap());

            for j in 1..=9 {
                let mut next = HashSet::new();

                for p in front.drain() {
                    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
                        let np = (p.0 + d.0, p.1 + d.1);

                        if let Some(npi) = matrix.point_to_index(np) {
                            if matrix.buffer[npi] == j {
                                next.insert(np);
                            }
                        }
                    }
                }

                front = next;
            }

            total += front.len();
        }
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    let matrix = Matrix::from_lines(
        BufReader::new(data).lines().flatten(),
        |c| c.to_digit(10).unwrap() as usize
    );

    let mut total = 0;

    for i in 0..matrix.buffer.len() {
        if matrix.buffer[i] == 0 {
            let mut front = HashMap::new();
            front.insert(matrix.index_to_point(i).unwrap(), 1);

            for j in 1..=9 {
                let mut next = HashMap::new();

                for (p, p_count) in front.drain() {
                    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
                        let next_p = (p.0 + d.0, p.1 + d.1);

                        if let Some(npi) = matrix.point_to_index(next_p) {
                            if matrix.buffer[npi] == j {
                                next.entry(next_p).and_modify(|v| *v += p_count).or_insert(p_count);
                            }
                        }
                    }
                }

                front = next;
            }

            total += front.drain().map(|(_, v)| v).sum::<usize>();
        }
    }

    return total;
}