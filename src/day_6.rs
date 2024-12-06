use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{once};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = 1, Right = 2, Up = 4, Down = 8
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            &Direction::Left => Direction::Up,
            &Direction::Right => Direction::Down,
            &Direction::Up => Direction::Right,
            &Direction::Down => Direction::Left,
        }
    }
}

pub fn part_1(data: File) -> usize {
    let mut w = 0;
    let mut h = 0;

    let mut field = Vec::new();

    let mut pos = (0, 0);
    let mut dir = Direction::Down;

    for (i, line) in BufReader::new(data).lines().flatten().enumerate() {
        if w == 0 {
            w = line.len() as isize;
        }
        h += 1;

        field.extend(line.chars().map(|x| match x {
            '#' => true,
            _ => false
        }));

        line.chars().enumerate().for_each(|(j, x)| match x {
            '^' => { dir = Direction::Up; pos = (i as isize, j as isize); },
            '<' => { dir = Direction::Left; pos = (i as isize, j as isize); },
            '>' => { dir = Direction::Right; pos = (i as isize, j as isize); },
            'v' => { dir = Direction::Down; pos = (i as isize, j as isize); },
            _ => {}
        });
    }

    let obstructed = |i: isize, j: isize| if i < 0 || i >= h || j < 0 || j >= w { 2 }
        else if field[(i * w + j) as usize] { 1 } else { 0 };

    let mut visited = Vec::with_capacity(field.len());
    (0..field.len()).for_each(|_| { visited.push(0); });

    let mut total = 0;

    'outer: while visited[(pos.0 * w + pos.1) as usize] & (dir as usize) == 0 {
        if visited[(pos.0 * w + pos.1) as usize] == 0 {
            total += 1;
        }

        visited[(pos.0 * w + pos.1) as usize] |= dir as usize;

        let next_pos = match dir {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        };
        
        match obstructed(next_pos.0, next_pos.1) {
            2 => break 'outer,
            1 => { dir = dir.rotate_right(); },
            0 => { pos = next_pos; },
            _ => {},
        };
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    return 0;
}