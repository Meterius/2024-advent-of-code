use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::common::Matrix;

pub fn part_1(data: File) -> usize {
    let matrix = Matrix::from_lines(
        BufReader::new(data).lines().flatten(),
        |x, _| x,
    );

    let mut checked = vec![false; matrix.buffer.len()];

    let mut total = 0;

    for (i, &x) in matrix.buffer.iter().enumerate() {
        if !checked[i] {
            checked[i] = true;

            let mut queue = vec![i];
            let mut area = 0;
            let mut perimeter = 0;

            while let Some(j) = queue.pop() {
                let p = matrix.index_to_point(j).unwrap();

                area += 1;

                for d in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
                    let j2 = matrix.point_to_index((p.0 + d.0, p.1 + d.1));

                    if let Some(j2) = j2 {
                        if matrix.buffer[j2] == x {
                            if !checked[j2] {
                                checked[j2] = true;
                                queue.push(j2);
                            }
                        } else {
                            perimeter += 1;
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }

            total += area * perimeter;
        }
    }

    return total;
}

pub fn part_2(data: File) -> usize {
    let matrix = Matrix::from_lines(
        BufReader::new(data).lines().flatten(),
        |x, _| x,
    );

    let mut checked = vec![false; matrix.buffer.len()];

    let mut total = 0;

    for (i, &x) in matrix.buffer.iter().enumerate() {
        if !checked[i] {
            checked[i] = true;

            let mut queue = vec![i];
            let mut sides = 0;
            let mut area = 0;

            while let Some(j) = queue.pop() {
                let p = matrix.index_to_point(j).unwrap();
                area += 1;

                for d in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
                    let j2 = matrix.point_to_index((p.0 + d.0, p.1 + d.1));

                    if let Some(j2) = j2 {
                        if matrix.buffer[j2] == x {
                            if !checked[j2] {
                                checked[j2] = true;
                                queue.push(j2);
                            }
                        }
                    }
                }

                for sx in [-1, 1].into_iter() {
                    for sy in [-1, 1].into_iter() {
                        let hor_ver_adj = [(1, 0), (0, 1)].into_iter()
                            .map(|d| matrix.point_to_index((p.0 + d.0 * sx, p.1 + d.1 * sy))
                                .map_or(0, |j2| if matrix.buffer[j2] == x { 1 } else { 0 })
                            ).sum::<usize>();

                        let diag_adj = matrix.point_to_index((p.0 + sx, p.1 + sy))
                            .map_or(false, |j2| if matrix.buffer[j2] == x { true } else { false });

                        if hor_ver_adj == 0 {
                            sides += 1;
                        } else if hor_ver_adj == 2 && !diag_adj {
                            sides += 1;
                        }
                    }
                }
            }

            total += sides * area;
        }
    }

    return total;
}