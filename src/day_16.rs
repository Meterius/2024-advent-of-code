use std::cmp::Reverse;
use std::collections::HashSet;
use priority_queue::PriorityQueue;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::common::Matrix;

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
enum Direction {
    West = 0, North = 1, East = 2, South = 3
}

impl Direction {
    fn from(value: isize) -> Direction {
        match value.rem_euclid(4) {
            0 => Direction::West,
            1 => Direction::North,
            2 => Direction::East,
            3 => Direction::South,
            _ => unreachable!()
        }
    }

    fn rotated(&self, amount: isize) -> Direction {
        return Direction::from(self.clone() as isize + amount);
    }
}

pub fn part_1(data: File) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let matrix = Matrix::from_lines(
        BufReader::new(data).lines().flatten(),
        |v, p| match v {
            'E' => {
                end = (p.0 as isize, p.1 as isize);
                false
            },
            'S' => {
                start = (p.0 as isize, p.1 as isize);
                false
            },
            '#' => {
                true
            }
            _ => false,
        }
    );

    let mut found = HashSet::new();
    let mut queue = PriorityQueue::new();
    queue.push((start, Direction::East), Reverse(0));

    while let Some((loc, Reverse(dist))) = queue.pop() {
        if found.insert(loc) {
            let (pos, dir) = loc;

            if pos == end {
                return dist;
            }

            let next_pos = (
                pos.0 + match dir {
                    Direction::East => 1,
                    Direction::West => -1,
                    _ => 0
                },
                pos.1 + match dir {
                    Direction::North => -1,
                    Direction::South => 1,
                    _ => 0
                },
            );

            if matrix.get(next_pos).is_some_and(|v| !v) {
                queue.push_increase((next_pos, dir), Reverse(dist + 1));
            }

            for i in 1..=3 {
                let rot_dir = dir.rotated(i);
                queue.push_increase((pos, rot_dir), Reverse(dist + 1000 + (if i == 2 { 1000 } else { 0 })));
            }
        }
    }

    unreachable!();
}

pub fn part_2(data: File) -> usize {
    return 0;
}