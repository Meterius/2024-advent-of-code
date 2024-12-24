use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum DirButton { Left, Right, Up, Down, Enter }

impl DirButton {
    fn iter() -> impl Iterator<Item=DirButton> {
        return [DirButton::Left, DirButton::Right, DirButton::Up, DirButton::Down, DirButton::Enter].into_iter();
    }

    fn as_char(&self) -> char {
        match self {
            DirButton::Left => '<',
            DirButton::Right => '>',
            DirButton::Up => '^',
            DirButton::Down => 'v',
            DirButton::Enter => 'A'
        }
    }

    fn position(&self) -> (usize, usize) {
        match self {
            DirButton::Left => (0, 1),
            DirButton::Right => (2, 1),
            DirButton::Up => (1, 0),
            DirButton::Down => (1, 1),
            DirButton::Enter => (2, 0),
        }
    }

    fn change_cost(&self, other: &DirButton) -> usize {
        let pa = self.position();
        let pb = other.position();
        return pa.0.abs_diff(pb.0) + pa.1.abs_diff(pb.1);
    }

    fn implementations(&self, other: &DirButton) -> Vec<Vec<DirButton>> {
        let mut seqs = vec![];

        fn implement_hor(x0: usize, x1: usize, out: &mut Vec<DirButton>) {
            if x0 <= x1 {
                (x0..x1).for_each(|y| out.push(DirButton::Right));
            } else {
                (x1..x0).for_each(|y| out.push(DirButton::Left));
            }
        }

        fn implement_ver(y0: usize, y1: usize, out: &mut Vec<DirButton>) {
            if y0 <= y1 {
                (y0..y1).for_each(|y| out.push(DirButton::Down));
            } else {
                (y1..y0).for_each(|y| out.push(DirButton::Up));
            }
        }

        let start = self.position();
        let end = other.position();

        if start.1 == end.1 || start.0 == end.0 {
            let mut seq = vec![];
            implement_hor(start.0, end.0, &mut seq);
            implement_ver(start.1, end.1, &mut seq);
            seqs.push(seq);
        } else {
            if *other != DirButton::Left {
                let mut hor_ver = vec![];
                implement_hor(start.0, end.0, &mut hor_ver);
                implement_ver(start.1, end.1, &mut hor_ver);
                seqs.push(hor_ver);
            }

            if *self != DirButton::Left {
                let mut ver_hor = vec![];
                implement_ver(start.1, end.1, &mut ver_hor);
                implement_hor(start.0, end.0, &mut ver_hor);
                seqs.push(ver_hor);
            }
        }

        return seqs;
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum KeypadButton {
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, Enter
}

impl KeypadButton {
    fn from(x: char) -> KeypadButton {
        match x {
            '0' => KeypadButton::N0,
            '1' => KeypadButton::N1,
            '2' => KeypadButton::N2,
            '3' => KeypadButton::N3,
            '4' => KeypadButton::N4,
            '5' => KeypadButton::N5,
            '6' => KeypadButton::N6,
            '7' => KeypadButton::N7,
            '8' => KeypadButton::N8,
            '9' => KeypadButton::N9,
            'A' => KeypadButton::Enter,
            _ => panic!("Invalid")
        }
    }

    fn position(&self) -> (usize, usize) {
        match self {
            KeypadButton::Enter => (2, 3),
            KeypadButton::N0 => (1, 3),
            KeypadButton::N1 => (0, 2),
            KeypadButton::N2 => (1, 2),
            KeypadButton::N3 => (2, 2),
            KeypadButton::N4 => (0, 1),
            KeypadButton::N5 => (1, 1),
            KeypadButton::N6 => (2, 1),
            KeypadButton::N7 => (0, 0),
            KeypadButton::N8 => (1, 0),
            KeypadButton::N9 => (2, 0),
        }
    }

    fn implementations(&self, other: &KeypadButton) -> Vec<Vec<DirButton>> {
        let mut seqs = vec![];

        fn implement_hor(x0: usize, x1: usize, out: &mut Vec<DirButton>) {
            if x0 <= x1 {
                (x0..x1).for_each(|y| out.push(DirButton::Right));
            } else {
                (x1..x0).for_each(|y| out.push(DirButton::Left));
            }
        }

        fn implement_ver(y0: usize, y1: usize, out: &mut Vec<DirButton>) {
            if y0 <= y1 {
                (y0..y1).for_each(|y| out.push(DirButton::Down));
            } else {
                (y1..y0).for_each(|y| out.push(DirButton::Up));
            }
        }

        let start = self.position();
        let end = other.position();

        if start.1 == end.1 || start.0 == end.0 {
            let mut seq = vec![];
            implement_hor(start.0, end.0, &mut seq);
            implement_ver(start.1, end.1, &mut seq);
            seqs.push(seq);
        } else {
            if start.1 != 3 || end.0 != 0 {
                let mut hor_ver = vec![];
                implement_hor(start.0, end.0, &mut hor_ver);
                implement_ver(start.1, end.1, &mut hor_ver);
                seqs.push(hor_ver);
            }

            if start.0 != 0 || end.1 != 3 {
                let mut ver_hor = vec![];
                implement_ver(start.1, end.1, &mut ver_hor);
                implement_hor(start.0, end.0, &mut ver_hor);
                seqs.push(ver_hor);
            }
        }

        // println!("{:?} to {:?}; {:?}", self, other, seqs);

        return seqs;
    }
}

lazy_static! {
    static ref DIR_BUTTON_TRANSISTIONS: HashMap::<(DirButton, DirButton), Vec<Vec<DirButton>>> = {
        let mut transitions = HashMap::new();

        let mut adjacent = HashMap::new();
        adjacent.insert(DirButton::Left, vec![
            (DirButton::Down, DirButton::Right)
        ]);
        adjacent.insert(DirButton::Down, vec![
            (DirButton::Left, DirButton::Left),
            (DirButton::Right, DirButton::Right),
            (DirButton::Up, DirButton::Up),
        ]);
        adjacent.insert(DirButton::Right, vec![
            (DirButton::Enter, DirButton::Up),
            (DirButton::Down, DirButton::Left),
        ]);
        adjacent.insert(DirButton::Up, vec![
            (DirButton::Enter, DirButton::Right),
            (DirButton::Down, DirButton::Down),
        ]);
        adjacent.insert(DirButton::Enter, vec![
            (DirButton::Up, DirButton::Left),
            (DirButton::Right, DirButton::Down),
        ]);

        for start in DirButton::iter() {
            transitions.insert((start, start), vec![vec![]]);

            let mut queue = vec![(vec![], start, HashSet::from([start]))];
            while let Some((path, loc, visited)) = queue.pop() {
                for (next, dir) in adjacent[&loc].iter() {
                    if !visited.contains(next) {
                        let mut next_visited = visited.clone();
                        next_visited.insert(next.clone());

                        let mut next_path = path.clone();
                        next_path.push(dir.clone());

                        transitions.entry((start, next.clone()))
                            .and_modify(|x: &mut Vec<Vec<DirButton>>| { x.push(next_path.clone()); })
                            .or_insert(vec![next_path.clone()]);

                        queue.push((next_path, next.clone(), next_visited));
                    }
                }
            }
        }

        return transitions;
    };
}

// Returns a map, such that given (a, b)
// if k nested directional keypads are in state <Enter, ..., Enter, a>
// returns the minimal length of a sequence to output b on the output keypad with resulting state <Enter, ..., Enter, b>
// given that the sequence is written to the first keypad
fn output_cost_direction_keypad(k: usize) -> HashMap::<(DirButton, DirButton), usize> {
    if k == 0 {
        return HashMap::from_iter(
            DirButton::iter().flat_map(
                |start| DirButton::iter().map(move |end| ((start, end), 1))
            )
        );
    }

    let prev = output_cost_direction_keypad(k - 1);
    let mut next = HashMap::new();

    for start in DirButton::iter() {
        for end in DirButton::iter() {
            // we guess the sequence of buttons on the k - 1 th keypad that would result in
            // the desired start to end transition and output, that cost can be determined
            // using the prior table
            // cost_prior(seq) = sum_i prev[seq[i - 1], seq[i]]
            // next[start, end] = min { cost_prior(z) | z moves cursor from start to end and then outputs enter }

            next.insert((start, end), DIR_BUTTON_TRANSISTIONS[&(start, end)]
                .iter()
                .map(|seq|
                    if seq.len() == 0 {
                        prev[&(DirButton::Enter, DirButton::Enter)]
                    } else {
                        prev[&(DirButton::Enter, seq[0])] + prev[&(seq[seq.len() - 1], DirButton::Enter)]
                        + (0..seq.len() - 1).map(|i| prev[&(seq[i], seq[i + 1])]).sum::<usize>()
                    }
                )
                .min()
                .unwrap()
            );
        }
    }

    return next;
}

fn cost_keypad(output: impl Iterator<Item=KeypadButton>, k: usize) -> usize {
    let unit_cost = output_cost_direction_keypad(k);

    let mut cost = 0;
    let mut current = KeypadButton::Enter;
    for button in output {
        cost += current
            .implementations(&button)
            .iter()
            .map(|seq|
                unit_cost[&(DirButton::Enter, seq[0])]
                    + (0..seq.len() - 1).map(|i| unit_cost[&(seq[i], seq[i + 1])]).sum::<usize>()
                    + if seq.len() != 0 {
                    unit_cost[&(seq[seq.len() - 1], DirButton::Enter)]
                } else { 0 }
            )
            .min()
            .unwrap();

        current = button;
    }

    return cost;
}

pub fn part_1(data: File) -> usize {
    let mut total = 0;

    for line in BufReader::new(data).lines().flatten() {
        total += line[..line.len() - 1].parse::<usize>().unwrap() * cost_keypad(line.chars().map(|x| KeypadButton::from(x)), 2);
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    let mut total = 0;

    for line in BufReader::new(data).lines().flatten() {
        total += line[..line.len() - 1].parse::<usize>().unwrap() * cost_keypad(line.chars().map(|x| KeypadButton::from(x)), 25);
    }

    return total;
}