use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::Vacant;
use priority_queue::PriorityQueue;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
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

    fn directions() -> impl Iterator<Item=Direction> {
        return [Direction::West, Direction::East, Direction::North, Direction::South].into_iter();
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

fn dijkstra(
    field: &Matrix<bool>,
    start: impl Iterator<Item=((isize, isize), Direction)>,
    reversed: bool,
) -> HashMap::<((isize, isize), Direction), usize> {
    let mut found = HashMap::new();
    let mut queue = PriorityQueue::new();

    for loc in start {
        queue.push(loc, Reverse(0));
    }

    while let Some((loc, Reverse(dist))) = queue.pop() {
        if let Vacant(vac_entry) = found.entry(loc) {
            vac_entry.insert(dist);

            let (pos, dir) = loc;

            let next_pos = (
                pos.0 + match dir {
                    Direction::East => 1,
                    Direction::West => -1,
                    _ => 0
                } * (if reversed { -1 } else { 1 }),
                pos.1 + match dir {
                    Direction::North => -1,
                    Direction::South => 1,
                    _ => 0
                } * (if reversed { -1 } else { 1 }),
            );

            if field.get(next_pos).is_some_and(|v| !v) {
                queue.push_increase((next_pos, dir), Reverse(dist + 1));
            }

            for i in 1..=3 {
                let rot_dir = dir.rotated(i);
                queue.push_increase((pos, rot_dir), Reverse(dist + 1000 + (if i == 2 { 1000 } else { 0 })));
            }
        }
    }

    return found;
}

pub fn part_2(data: File) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let field = Matrix::from_lines(
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

    let start_dist = dijkstra(&field, once((start, Direction::East)), false);
    let end_dist = dijkstra(&field, Direction::directions().map(|dir| (end, dir)), true);

    let min_dist = Direction::directions()
        .into_iter()
        .map(|dir| start_dist.get(&(end, dir)).cloned().unwrap_or(usize::MAX))
        .min()
        .unwrap();

    let mut checked = HashSet::new();

    let mut total = 0;
    for (loc, for_dist) in start_dist {
        if end_dist.get(&loc).is_some_and(|&rev_dist| for_dist + rev_dist == min_dist) {
            if checked.insert(loc.0) {
                total += 1;
            }
        }
    }

    return total;
}