use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::iter::{once};
use bitflags::bitflags;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = 1<<0, Right = 1<<1, Up = 1<<2, Down = 1<<3
}

impl Direction {
    fn rotated_clockwise(&self) -> Direction {
        match self {
            &Direction::Left => Direction::Up,
            &Direction::Right => Direction::Down,
            &Direction::Up => Direction::Right,
            &Direction::Down => Direction::Left,
        }
    }
}

enum GuardFieldValue {
    Empty,
    Obstacle,
    OutOfBounds
}

#[derive(Debug, Clone)]
struct GuardField {
    field: Vec<bool>,

    width: usize,
    height: usize,
}

impl GuardField {
    fn new(field: Vec<bool>, width: usize, height: usize) -> GuardField {
        GuardField { field, width, height }
    }

    fn to_index(&self, (x, y): (isize, isize)) -> Option<usize> {
        if x < 0 || x >= (self.width as isize) || y < 0 || y >= (self.height as isize) {
            return None;
        } else {
            return Some(self.width * y as usize + x as usize);
        }
    }

    fn get(&self, (x, y): (isize, isize)) -> GuardFieldValue {
        if let Some(idx) = self.to_index((x, y)) {
            return if self.field[idx] { GuardFieldValue::Obstacle } else { GuardFieldValue::Empty };
        } else {
            return GuardFieldValue::OutOfBounds;
        }
    }

    fn set(&mut self, (x, y): (isize, isize), obstacle: bool) {
        let idx = self.to_index((x, y)).unwrap();
        self.field[idx] = obstacle;
    }

    fn get_next_position((x, y): (isize, isize), dir: Direction) -> (isize, isize) {
        match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

struct GuardWalk<'a> {
    field: &'a mut GuardField,
    visited: Vec<u8>,

    position: (isize, isize),
    direction: Direction,
}

enum GuardWalkStepOutcome {
    Step(bool),
    Rotate,
    Loop,
    OutOfBounds,
}

impl<'a> GuardWalk<'a> {
    fn new(field: &'a mut GuardField, position: (isize, isize), direction: Direction) -> GuardWalk<'a> {
        let visited = vec![0; field.width * field.height];
        GuardWalk { field, position, direction, visited }
    }

    fn step_until_end(&mut self) -> bool {
        loop {
            match self.step() {
                GuardWalkStepOutcome::Loop => { return false; },
                GuardWalkStepOutcome::Rotate => {},
                GuardWalkStepOutcome::OutOfBounds => { return true; },
                GuardWalkStepOutcome::Step(_) => {},
            }
        };
    }

    fn step(&mut self) -> GuardWalkStepOutcome {
        let next_pos = GuardField::get_next_position(self.position, self.direction);

        match self.field.get(next_pos) {
            GuardFieldValue::Empty => {
                let next_idx = self.field.to_index(next_pos).unwrap();

                if self.visited[next_idx] & self.direction as u8 != 0 {
                    return GuardWalkStepOutcome::Loop;
                }

                self.position = next_pos;
                self.visited[next_idx] |= self.direction as u8;

                return GuardWalkStepOutcome::Step(self.visited[next_idx] == self.direction as u8);
            },
            GuardFieldValue::Obstacle => {
                self.direction = self.direction.rotated_clockwise();
                self.visited[self.field.to_index(self.position).unwrap()] |= self.direction as u8;

                return GuardWalkStepOutcome::Rotate;
            },
            GuardFieldValue::OutOfBounds => {
                return GuardWalkStepOutcome::OutOfBounds;
            },
        };
    }
    
    fn restart(&mut self, pos: (isize, isize), dir: Direction, visited_cache: &[u8]) {
        self.visited.as_mut_slice().copy_from_slice(visited_cache);
        self.position = pos;
        self.direction = dir;
        self.visited[self.field.to_index(self.position).unwrap()] = 0;
    }

    fn to_2d_string(&self) -> String {
        let mut result = String::new();

        for y in 0..self.field.height {
            for x in 0..self.field.width {
                result.push(match self.field.get((x as isize, y as isize)) {
                    GuardFieldValue::Empty => if self.visited[self.field.to_index((x as isize, y as isize)).unwrap()] == 0 {
                        '_'
                    } else {
                        'X'
                    },
                    GuardFieldValue::Obstacle => '#',
                    GuardFieldValue::OutOfBounds => '-',
                });
            }

            result.push('\n');
        }

        return result;
    }

    fn total_visited(&self) -> usize {
        return self.visited.iter().fold(0, |prev, &x| if x != 0 { prev + 1 } else { prev });
    }
}

fn lines_to_field_walk(lines: impl Iterator<Item=String>) -> (GuardField, (isize, isize), Direction) {
    let mut w = 0;
    let mut h = 0;

    let mut field = Vec::new();

    let mut pos = (0, 0);
    let mut dir = Direction::Down;

    for (i, line) in lines.enumerate() {
        if w == 0 {
            w = line.len();
        }
        h += 1;

        field.extend(line.chars().map(|x| match x {
            '#' => true,
            _ => false
        }));

        line.chars().enumerate().for_each(|(j, x)| match x {
            '^' => { dir = Direction::Up; pos = (j as isize, i as isize); },
            '<' => { dir = Direction::Left; pos = (j as isize, i as isize); },
            '>' => { dir = Direction::Right; pos = (j as isize, i as isize); },
            'v' => { dir = Direction::Down; pos = (j as isize, i as isize); },
            _ => {}
        });
    }

    return (GuardField::new(
        field, w, h,
    ), pos, dir);
}

pub fn part_1(data: File) -> usize {
    let (mut field, pos, dir) = lines_to_field_walk(BufReader::new(data).lines().flatten());

    let mut walk = GuardWalk::new(&mut field, pos, dir);
    walk.step_until_end();

    return walk.total_visited();
}

fn walk(
    field: &Vec<bool>,
    visited: &mut Vec<u8>,
    mut pos: (isize, isize),
    mut dir: Direction,
    w: isize,
    h: isize
) -> bool {
    while visited[(pos.0 * w + pos.1) as usize] & (dir as u8) == 0 {
        visited[(pos.0 * w + pos.1) as usize] |= dir as u8;

        let next_pos = match dir {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        };

        if next_pos.0 < 0 || next_pos.0 >= h || next_pos.1 < 0 || next_pos.1 >= w {
            return false;
        } else if field[(next_pos.0 * w + next_pos.1) as usize] {
            dir = dir.rotated_clockwise();
        } else {
            pos = next_pos;
        }
    }

    return true;
}

pub fn part_2(data: File) -> usize {
    let (mut field, pos, dir) = lines_to_field_walk(BufReader::new(data).lines().flatten());
    let mut test_field = field.clone();

    let mut walk = GuardWalk::new(&mut field, pos, dir);
    let mut test_walk = GuardWalk::new(&mut test_field, pos, dir);
    

    let mut total = 0;
    let mut total_not = 0;
    'outer: loop {
        let pos = walk.position;
        let dir = walk.direction;
        
        match walk.step() {
            GuardWalkStepOutcome::Loop => { break 'outer; },
            GuardWalkStepOutcome::Rotate => {},
            GuardWalkStepOutcome::Step(unvisited) => {
                if unvisited {
                    test_walk.field.set(walk.position, true);
                    test_walk.restart(pos, dir, walk.visited.as_slice());
                    
                    if !test_walk.step_until_end() {
                        total += 1;
                    } else {
                        total_not += 1;
                    }

                    test_walk.field.set(walk.position, false);
                }
            },
            GuardWalkStepOutcome::OutOfBounds => { break 'outer; }
        }
    };
    
    println!("Found {} possibilities, explored {} invalid ones", total, total_not);

    return total;
}