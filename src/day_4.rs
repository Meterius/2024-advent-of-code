use std::fs::File;
use std::io::{BufRead, BufReader};

const N: usize = 10;

pub fn part_1(data: File) -> usize {
    let mut total = 0;

    const pattern: &str = "XMAS";

    let mut hor_forward = 0;
    let mut hor_backwards = 0;

    let mut prev_diag_forward = [0; N];
    let mut diag_forward = [0; N];

    let mut prev_diag_backward = [0; N];
    let mut diag_backward = [0; N];

    let mut vert_forward = [0; N];
    let mut vert_backward = [0; N];

    for line in BufReader::new(data).lines().flatten() {
        for (i, x) in line.chars().enumerate() {
            let r = if i == 0 { 0 } else { prev_diag_forward[i - 1] };
            if pattern.chars().nth(r).is_some_and(|y| y == x) {
                if r == pattern.len() - 1 {
                    total += 1;
                    diag_forward[i] = 0;
                } else {
                    diag_forward[i] = r + 1;
                }
            } else {
                diag_forward[i] = if pattern.chars().next().is_some_and(|y| y == x) { 1 } else { 0 };
            }

            let r = if i == 0 { 0 } else { prev_diag_backward[i - 1] };
            if pattern.chars().nth_back(r).is_some_and(|y| y == x) {
                if r == pattern.len() - 1 {
                    total += 1;
                    diag_backward[i] = 0;
                } else {
                    diag_backward[i] = r + 1;
                }
            } else {
                diag_backward[i] = if pattern.chars().next_back().is_some_and(|y| y == x) { 1 } else { 0 };
            }

            let r = vert_forward[i];
            if pattern.chars().nth(r).is_some_and(|y| y == x) {
                if r == pattern.len() - 1 {
                    total += 1;
                    vert_forward[i] = 0;
                } else {
                    vert_forward[i] = r + 1;
                }
            } else {
                vert_forward[i] = if pattern.chars().next().is_some_and(|y| y == x) { 1 } else { 0 };
            }

            let r = vert_backward[i];
            if pattern.chars().nth_back(r).is_some_and(|y| y == x) {
                if r == pattern.len() - 1 {
                    total += 1;
                    vert_backward[i] = 0;
                } else {
                    vert_backward[i] = r + 1;
                }
            } else {
                vert_backward[i] = if pattern.chars().next_back().is_some_and(|y| y == x) { 1 } else { 0 };
            }

            let r = hor_forward;
            if pattern.chars().nth(r).is_some_and(|y| y == x) {
                if r == pattern.len() - 1 {
                    total += 1;
                    hor_forward = 0;
                } else {
                    hor_forward = r + 1;
                }
            } else {
                hor_forward = if pattern.chars().next().is_some_and(|y| y == x) { 1 } else { 0 };
            }

            let r = hor_backwards;
            if pattern.chars().nth_back(r).is_some_and(|y| y == x) {
                if r == pattern.len() - 1 {
                    total += 1;
                    hor_backwards = 0;
                } else {
                    hor_backwards = r + 1;
                }
            } else {
                hor_backwards = if pattern.chars().next_back().is_some_and(|y| y == x) { 1 } else { 0 };
            }
        }

        println!("{} {}", line, total);

        prev_diag_forward = diag_forward;
        prev_diag_backward = diag_backward;

        hor_forward = 0;
        hor_backwards = 0;
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    0
}