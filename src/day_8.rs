use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use crate::common::Matrix;

pub fn part_1(data: File) -> usize {
    let matrix = Matrix::from_lines(BufReader::new(data).lines().flatten(), |x| x);

    let mut antennas: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, &x) in matrix.buffer.iter().enumerate() {
        if x != '.' {
            antennas.entry(x).and_modify(|x| { x.push(i) }).or_insert(vec![i]);
        }
    }
    
    let mut antinodes = HashSet::new();

    for (_, locs) in antennas.iter() {
        for i in 0..locs.len() {
            for j in i + 1..locs.len() {
                let a = matrix.index_to_point(locs[i]).unwrap();
                let b = matrix.index_to_point(locs[j]).unwrap();

                let d = (b.0 - a.0, b.1 - a.1);

                let c1 = (b.0 + d.0, b.1 + d.1);
                let c2 = (a.0 - d.0, a.1 - d.1);

                if matrix.point_to_index(c1).is_some() {
                    antinodes.insert(c1);
                }

                if matrix.point_to_index(c2).is_some() {
                    antinodes.insert(c2);
                }
            }
        }
    }

    return antinodes.len();
}

pub fn part_2(data: File) -> usize {
    let matrix = Matrix::from_lines(BufReader::new(data).lines().flatten(), |x| x);

    let mut antennas: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, &x) in matrix.buffer.iter().enumerate() {
        if x != '.' {
            antennas.entry(x).and_modify(|x| { x.push(i) }).or_insert(vec![i]);
        }
    }

    let mut antinodes = HashSet::new();

    for (_, locs) in antennas.iter() {
        for i in 0..locs.len() {
            for j in i + 1..locs.len() {
                let a = matrix.index_to_point(locs[i]).unwrap();
                let b = matrix.index_to_point(locs[j]).unwrap();

                let d = (b.0 - a.0, b.1 - a.1);

                let mut cp = a;
                while let Some(cpi) = matrix.point_to_index(cp) {
                    antinodes.insert(cpi);
                    cp.0 += d.0;
                    cp.1 += d.1;
                }

                let mut cp = (a.0 - d.0, a.1 - d.1);
                while let Some(cpi) = matrix.point_to_index(cp) {
                    antinodes.insert(cpi);
                    cp.0 -= d.0;
                    cp.1 -= d.1;
                }
            }
        }
    }

    return antinodes.len();
}