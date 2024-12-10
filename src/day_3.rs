use std::fs::File;
use utf8_read::Reader;

enum MulParserState {
    Left(usize),
    ArgLeft,
    ArgRight,
    Failed,
    Finished(usize, usize),
}

struct MulParser {
    state: MulParserState,
    left: usize,
    right: usize
}

impl MulParser {
    fn new() -> MulParser {
        MulParser { left: 0, right: 0, state: MulParserState::Left(0) }
    }

    fn advance(&mut self, x: char) {
        self.state = match &self.state {
            MulParserState::Left(i) => match (i, x) {
                (0, 'm') => MulParserState::Left(1),
                (1, 'u') => MulParserState::Left(2),
                (2, 'l') => MulParserState::Left(3),
                (3, '(') => MulParserState::ArgLeft,
                (_, _) => MulParserState::Failed
            },
            MulParserState::ArgLeft => if x == ',' { MulParserState::ArgRight } else {
                match x.to_digit(10) {
                    Some(x) => {
                        self.left = self.left * 10 + x as usize;
                        MulParserState::ArgLeft
                    },
                    None => MulParserState::Failed
                }
            },
            MulParserState::ArgRight => if x == ')' { MulParserState::Finished(self.left, self.right) } else {
                match x.to_digit(10) {
                    Some(x) => {
                        self.right = self.right * 10 + x as usize;
                        MulParserState::ArgRight
                    },
                    None => MulParserState::Failed
                }
            },
            MulParserState::Finished(_, _) => MulParserState::Failed,
            MulParserState::Failed => MulParserState::Failed,
        };
    }

    fn reset(&mut self) {
        self.state = MulParserState::Left(0);
        self.left = 0;
        self.right = 0;
    }

    fn advance_or_reset(&mut self, x: char) {
        self.advance(x);

        if let MulParserState::Failed = &self.state {
            self.reset();

            self.advance(x);
            if let MulParserState::Failed = &self.state { self.reset(); }
        }
    }
}

pub fn part_1(data: File) -> usize {
    let mut parser = MulParser::new();
    let mut total = 0;

    for x in Reader::new(&data).into_iter().flatten() {
        parser.advance_or_reset(x.clone());

        if let MulParserState::Finished(x, y) = &parser.state {
            total += x * y;
            parser.reset();
        }
    }

    return total;
}


pub fn part_2(data: File) -> usize {
    let mut parser = MulParser::new();
    let mut total = 0;

    let mut do_state = 0;
    let mut dont_state = 0;

    let mut active = true;

    for x in Reader::new(&data).into_iter().flatten() {
        if x == "do()"[do_state..=do_state].chars().next().unwrap() {
            do_state += 1;

            if do_state == 4 {
                do_state = 0;
                active = true;
            }
        } else {
            do_state = 0;
        }

        if x == "don't()"[dont_state..=dont_state].chars().next().unwrap() {
            dont_state += 1;

            if dont_state == 7 {
                dont_state = 0;
                active = false;
            }
        } else {
            dont_state = 0;
        }

        parser.advance_or_reset(x.clone());

        if let MulParserState::Finished(x, y) = &parser.state {
            if active { total += x * y };
            parser.reset();
        }
    }

    return total;
}