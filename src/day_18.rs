use std::cmp::Reverse;
use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use priority_queue::PriorityQueue;

pub fn part_1(data: File) -> usize {
    const W: usize = 71;
    const H: usize = 71;

    let mut field: [[bool; H]; W] = [[false; H]; W];

    for line in BufReader::new(data).lines().flatten().take(1024) {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        field[x][y] = true;
    }

    let start = (0, 0);
    let end = (W as isize - 1, H as isize - 1);

    let mut found = HashSet::new();
    let mut queue = PriorityQueue::new();
    queue.push(start, Reverse(0));

    while let Some((pos, dist)) = queue.pop() {
        if found.insert(pos) {
            if pos == end {
                return dist.0;
            }

            for d in [(-1, 0), (1, 0), (0, 1), (0, -1)].into_iter() {
                let next_pos = (pos.0 + d.0, pos.1 + d.1);

                if 0 <= next_pos.0 && next_pos.0 < W as isize && 0 <= next_pos.1 && next_pos.1 < H as isize {
                    if !field[next_pos.0 as usize][next_pos.1 as usize] && !found.contains(&next_pos) {
                        queue.push_increase(next_pos, Reverse(dist.0 + 1));
                    }
                }
            }
        }
    }

    panic!("Unreachable");
}

pub fn part_2(data: File) -> String {
    const W: usize = 71;
    const H: usize = 71;

    fn reachable(field: &[[bool; H]; W]) -> bool {
        let start = (0, 0);
        let end = (W as isize - 1, H as isize - 1);

        let mut found = HashSet::new();
        let mut queue = Vec::new();
        queue.push(start);

        while let Some(pos) = queue.pop() {
            if pos == end {
                return true;
            }

            for d in [(-1, 0), (1, 0), (0, 1), (0, -1)].into_iter() {
                let next_pos = (pos.0 + d.0, pos.1 + d.1);

                if 0 <= next_pos.0 && next_pos.0 < W as isize && 0 <= next_pos.1 && next_pos.1 < H as isize {
                    if !field[next_pos.0 as usize][next_pos.1 as usize] && found.insert(next_pos) {
                        queue.push(next_pos);
                    }
                }
            }
        }

        return false;
    }

    let mut blocks = Vec::new();
    for line in BufReader::new(data).lines().flatten() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        blocks.push((x, y));
    }

    let mut field: [[bool; H]; W] = [[false; H]; W];
    let mut field_query: [[bool; H]; W] = [[false; H]; W];

    let mut lower_bound = 0;
    let mut upper_bound = blocks.len() + 1;

    while lower_bound + 1 != upper_bound {
        let step = (upper_bound - lower_bound) / 2;

        for (x, y) in &blocks[lower_bound..lower_bound + step] {
            field_query[*x][*y] = true;
        }

        if reachable(&field_query) {
            lower_bound += step;
            field = field_query.clone();
        } else {
            upper_bound = lower_bound + step;
            field_query = field.clone();
        }
    }

    let (x, y) = blocks[lower_bound];
    return format!("{x},{y}");
}
