use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};
use crate::common::Matrix;

fn solve(data: File, cheat_dist: usize, cheat_req: usize) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let field = Matrix::from_lines(
        BufReader::new(data).lines().flatten(),
        |c, pos| match c {
            '#' => true,
            'S' => { start = (pos.0 as isize, pos.1 as isize); false },
            'E' => { end = (pos.0 as isize, pos.1 as isize); false },
            _ => false,
        },
    );

    let mut track = HashMap::<(isize, isize), usize>::new();
    track.insert(end, 0);

    let mut queue = VecDeque::new();
    queue.push_front(end.clone());

    while let Some(pos) = queue.pop_front() {
        for (next, x) in field.neighborhood_four_way(pos) {
            if !x && !track.contains_key(&next) {
                track.insert(next, track[&pos] + 1);
                queue.push_back(next);
            }
        }
    }

    let mut total = 0;

    for (&pos, &rank) in track.iter() {
        for (cheat_end, &cheat_end_blocked) in field.neighborhood_manhattan(pos, cheat_dist) {
            if !cheat_end_blocked {
                let cheat_rank = track[&cheat_end] + pos.0.abs_diff(cheat_end.0) + pos.1.abs_diff(cheat_end.1);
                total += if cheat_rank + cheat_req <= rank { 1 } else { 0 };
            }
        }
    }

    return total;
}

pub fn part_1(data: File) -> usize {
    return solve(data, 2, 100);
}

pub fn part_2(data: File) -> usize {
    return solve(data, 20, 100);
}