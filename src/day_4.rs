use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct PatternCounter<const N: usize> {
    pattern: [char; N],
    count: usize,
    current: usize,
}

impl<const N: usize> PatternCounter<N> {
    fn new(pattern: [char; N]) -> PatternCounter<N> {
        PatternCounter {
            count: 0,
            current: 0,
            pattern,
        }
    }

    fn advance(&mut self, x: char) -> bool {
        if self.pattern[self.current] == x {
            if self.current == N - 1 {
                self.count += 1;
                self.current = if self.pattern[0] == x { 1 } else { 0 };

                return true;
            } else {
                self.current += 1;
            }
        } else {
            self.current = if self.pattern[0] == x { 1 } else { 0 };
        }

        return false;
    }

    fn restart(&mut self) {
        self.current = 0;
    }
}

const LINE_LENGTH: usize = 140;

pub fn part_1(data: File) -> usize {
    const PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];
    const PATTERN_REV: [char; 4] = ['S', 'A', 'M', 'X'];

    let mut horizontal_forward = PatternCounter::new(PATTERN);
    let mut horizontal_backward = PatternCounter::new(PATTERN_REV);

    let mut vertical_forward = [PatternCounter::new(PATTERN); LINE_LENGTH];
    let mut vertical_backward = [PatternCounter::new(PATTERN_REV); LINE_LENGTH];

    let mut diag_forward = [PatternCounter::new(PATTERN); LINE_LENGTH];
    let mut diag_backward = [PatternCounter::new(PATTERN_REV); LINE_LENGTH];
    let mut diag_base = LINE_LENGTH;

    let mut diag2_forward = [PatternCounter::new(PATTERN); LINE_LENGTH];
    let mut diag2_backward = [PatternCounter::new(PATTERN_REV); LINE_LENGTH];
    let mut diag2_base = 0;

    for line in BufReader::new(data).lines().flatten() {
        assert_eq!(line.len(), LINE_LENGTH);

        for (i, x) in line.chars().enumerate() {
            horizontal_forward.advance(x);
            horizontal_backward.advance(x);
            vertical_forward[i].advance(x);
            vertical_backward[i].advance(x);

            diag_forward[(diag_base + i) % LINE_LENGTH].advance(x);
            diag_backward[(diag_base + i) % LINE_LENGTH].advance(x);

            diag2_forward[(diag2_base + i) % LINE_LENGTH].advance(x);
            diag2_backward[(diag2_base + i) % LINE_LENGTH].advance(x);
        }

        diag_base = if diag_base == 1 { LINE_LENGTH } else { diag_base - 1 };
        diag_forward[diag_base % LINE_LENGTH].restart();
        diag_backward[diag_base % LINE_LENGTH].restart();

        diag2_forward[diag2_base % LINE_LENGTH].restart();
        diag2_backward[diag2_base % LINE_LENGTH].restart();
        diag2_base = if diag2_base == LINE_LENGTH - 1 { 0 } else { diag2_base + 1 };

        horizontal_forward.restart();
        horizontal_backward.restart();
    }

    return horizontal_forward.count
        + horizontal_backward.count
        + vertical_forward.into_iter().map(|x| x.count).sum::<usize>()
        + vertical_backward.into_iter().map(|x| x.count).sum::<usize>()
        + diag_forward.into_iter().map(|x| x.count).sum::<usize>()
        + diag_backward.into_iter().map(|x| x.count).sum::<usize>()
        + diag2_forward.into_iter().map(|x| x.count).sum::<usize>()
        + diag2_backward.into_iter().map(|x| x.count).sum::<usize>();
}

pub fn part_2(data: File) -> usize {
    const PATTERN: [char; 3] = ['M', 'A', 'S'];
    const PATTERN_REV: [char; 3] = ['S', 'A', 'M'];

    let mut diag_forward = [PatternCounter::new(PATTERN); LINE_LENGTH];
    let mut diag_backward = [PatternCounter::new(PATTERN_REV); LINE_LENGTH];
    let mut diag_base = LINE_LENGTH;

    let mut diag2_forward = [PatternCounter::new(PATTERN); LINE_LENGTH];
    let mut diag2_backward = [PatternCounter::new(PATTERN_REV); LINE_LENGTH];
    let mut diag2_base = 0;

    let mut total = 0;

    for line in BufReader::new(data).lines().flatten() {
        assert_eq!(line.len(), LINE_LENGTH);

        let mut diag2_check = [false, false];
        for (i, x) in line.chars().enumerate() {
            let diag_curr = diag_forward[(diag_base + i) % LINE_LENGTH].advance(x)
                | diag_backward[(diag_base + i) % LINE_LENGTH].advance(x);

            let diag2_curr = diag2_forward[(diag2_base + i) % LINE_LENGTH].advance(x)
                | diag2_backward[(diag2_base + i) % LINE_LENGTH].advance(x);

            if diag_curr && diag2_check[0] {
                total += 1;
            }

            diag2_check = [diag2_check[1], diag2_curr];
        }

        diag_base = if diag_base == 1 { LINE_LENGTH } else { diag_base - 1 };
        diag_forward[diag_base % LINE_LENGTH].restart();
        diag_backward[diag_base % LINE_LENGTH].restart();

        diag2_forward[diag2_base % LINE_LENGTH].restart();
        diag2_backward[diag2_base % LINE_LENGTH].restart();
        diag2_base = if diag2_base == LINE_LENGTH - 1 { 0 } else { diag2_base + 1 };
    }

    return total;
}