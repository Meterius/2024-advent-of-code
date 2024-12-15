use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::common::Matrix;

#[derive(Clone, Debug)]
enum Tile {
    Empty, Box, Wall
}

pub fn part_1(data: File) -> usize {
    let mut p: (isize, isize) = (0, 0);

    let mut field = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut lines = BufReader::new(data).lines().flatten();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if width == 0 {
            width = line.len();
        }

        for (x, v) in line.chars().enumerate() {
            field.push(match v {
                '@' => {
                    p = (x as isize, height as isize);
                    Tile::Empty
                },
                '#' => Tile::Wall,
                'O' => Tile::Box,
                _ => Tile::Empty,
            });
        }

        height += 1;
    }

    macro_rules! get_tile {
        ($pos: expr) => {
            {
                if $pos.0 < 0 || $pos.0 >= width as isize || $pos.1 < 0 || $pos.1 >= height as isize {
                    Tile::Wall
                } else { field[width * ($pos.1 as usize) + $pos.0 as usize].clone() }
            }
        };
    }

    macro_rules! print_tiles {
        () => { {
            for (i, v) in field.iter().enumerate() {
                if i != 0 && i % width == 0 {
                    print!("\n");
                }

                if p.0 == (i % width) as isize && p.1 == (i / width) as isize {
                    print!("@");
                } else {
                    print!("{}", match v {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Box => 'O'
                    });
                }
            }
            print!("\n");
        } };
    }

    for line in lines {
        for v in line.chars() {
            let dir = match v {
                '<' => (-1, 0),
                '>' => (1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => unreachable!()
            };

            let p_next = (p.0 + dir.0, p.1 + dir.1);

            match get_tile!(p_next) {
                Tile::Empty => { p = p_next; },
                Tile::Wall => {},
                Tile::Box => {
                    let mut p_box = p_next;

                    'outer: loop {
                        let mut p_box_next = (p_box.0 + dir.0, p_box.1 + dir.1);

                        match get_tile!(p_box_next) {
                            Tile::Empty => {
                                field[width * p_box_next.1 as usize + p_box_next.0 as usize] = Tile::Box;
                                field[width * p_next.1 as usize + p_next.0 as usize] = Tile::Empty;
                                p = p_next;
                                break 'outer;
                            },
                            Tile::Wall => {
                                break 'outer;
                            },
                            Tile::Box => {
                                p_box = p_box_next;
                            }
                        }
                    };
                }
            };

            //print_tiles!();
            //println!();
        }
    }

    let mut total = 0;

    for (i, v) in field.iter().enumerate() {
        match v {
            Tile::Box => {
                total += 100 * (i / width) + (i % width);
            },
            _ => {},
        };
    }

    return total;
}

#[derive(Clone, Debug)]
enum TileWide {
    Empty, Box(bool), Wall
}

pub fn part_2(data: File) -> usize {
    let mut p: (isize, isize) = (0, 0);

    let mut field = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut lines = BufReader::new(data).lines().flatten();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if width == 0 {
            width = line.len() * 2;
        }

        for (x, v) in line.chars().enumerate() {
            match v {
                '@' => {
                    p = (2 * x as isize, height as isize);
                    field.push(TileWide::Empty);
                    field.push(TileWide::Empty);
                },
                '#' => {
                    field.push(TileWide::Wall);
                    field.push(TileWide::Wall);
                },
                'O' => {
                    field.push(TileWide::Box(false));
                    field.push(TileWide::Box(true));
                },
                _ => {
                    field.push(TileWide::Empty);
                    field.push(TileWide::Empty);
                },
            };
        }

        height += 1;
    }

    macro_rules! get_tile {
        ($pos: expr) => {
            {
                if $pos.0 < 0 || $pos.0 >= width as isize || $pos.1 < 0 || $pos.1 >= height as isize {
                    TileWide::Wall
                } else { field[width * ($pos.1 as usize) + $pos.0 as usize].clone() }
            }
        };
    }

    macro_rules! print_tiles {
        () => { {
            for (i, v) in field.iter().enumerate() {
                if i != 0 && i % width == 0 {
                    print!("\n");
                }

                if p.0 == (i % width) as isize && p.1 == (i / width) as isize {
                    print!("@");
                } else {
                    print!("{}", match v {
                        TileWide::Empty => '.',
                        TileWide::Wall => '#',
                        TileWide::Box(false) => '[',
                        TileWide::Box(true) => ']'
                    });
                }
            }
            print!("\n");
        } };
    }
    
    //print_tiles!();
    //println!("");

    for line in lines {
        for v in line.chars() {
            let dir = match v {
                '<' => (-1, 0),
                '>' => (1, 0),
                '^' => (0, -1),
                'v' => (0, 1),
                _ => unreachable!()
            };

            let p_next = (p.0 + dir.0, p.1 + dir.1);

            match get_tile!(p_next) {
                TileWide::Empty => { p = p_next; },
                TileWide::Wall => {},
                TileWide::Box(right) => {
                    let mut p_boxes = Vec::new();
                    let mut p_moved = HashSet::new();
                    
                    p_boxes.push(p_next);
                    p_moved.insert(p_next);
                    
                    if dir.1 != 0 {
                        let p_next_right = (p_next.0 + if right { -1 } else { 1 }, p_next.1);
                        p_boxes.push(p_next_right);
                        p_moved.insert(p_next_right);
                    }
                    
                    let mut front = 0;
                    'outer: while front < p_boxes.len() {
                        let next_front = p_boxes.len();
                        
                        for i in front..next_front {
                            let mut p_box_next = (p_boxes[i].0 + dir.0, p_boxes[i].1 + dir.1);

                            match get_tile!(p_box_next) {
                                TileWide::Empty => {},
                                TileWide::Wall => {
                                    front = 0;
                                    p_boxes.clear();
                                    p_moved.clear();
                                    break 'outer;
                                },
                                TileWide::Box(right) => {
                                    if dir.1 != 0 {
                                        let p_box_next_right = (p_box_next.0 + if right { -1 } else { 1 }, p_box_next.1);
                                        
                                        if !p_moved.contains(&p_box_next_right) {
                                            p_boxes.push(p_box_next_right);
                                            p_moved.insert(p_box_next_right);
                                        }

                                        if !p_moved.contains(&p_box_next) {
                                            p_boxes.push(p_box_next);
                                            p_moved.insert(p_box_next);
                                        }
                                    } else {
                                        p_boxes.push(p_box_next);
                                        p_moved.insert(p_box_next);
                                    }
                                }
                            }
                        }
                        
                        front = next_front;
                    }
                    
                    if front != 0 {
                        p = p_next;
                        
                        for p_box in p_boxes.into_iter().rev() {
                            let p_box_prev = (p_box.0 - dir.0, p_box.1 - dir.1);
                            let p_box_next = (p_box.0 + dir.0, p_box.1 + dir.1);

                            let tile = get_tile!(p_box);
                            field[width * p_box_next.1 as usize + p_box_next.0 as usize] = tile;

                            if !p_moved.contains(&p_box_prev) {
                                field[width * p_box.1 as usize + p_box.0 as usize] = TileWide::Empty;
                            }
                        }
                    }
                }
            };

            //print_tiles!();
            //println!();
        }
    }

    let mut total = 0;

    for (i, v) in field.iter().enumerate() {
        match v {
            TileWide::Box(false) => {
                total += 100 * (i / width) + (i % width);
            },
            _ => {},
        };
    }

    return total;
}