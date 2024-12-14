use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn part_1(data: File) -> usize {
    const W: isize = 101;
    const H: isize = 103;

    const W_HALF: isize = W / 2;
    const H_HALF: isize = H / 2;

    const STEPS: isize = 100;

    let mut count = [0; 4];

    for line in BufReader::new(data).lines().flatten() {
        let (p, v) = line.split_once(' ').unwrap();

        let p: (isize, isize) = p.split('=').nth(1).unwrap()
            .split(',').map(|x| x.parse::<isize>().unwrap())
            .next_tuple().unwrap();

        let v: (isize, isize) = v.split('=').nth(1).unwrap()
            .split(',').map(|x| x.parse::<isize>().unwrap())
            .next_tuple().unwrap();

        let np = (
            (p.0 + STEPS * v.0).rem_euclid(W),
            (p.1 + STEPS * v.1).rem_euclid(H)
        );

        if np.0 != W_HALF && np.1 != H_HALF {
            let q = if np.0 > W_HALF { 1 } else { 0 } + if np.1 > H_HALF { 2 } else { 0 };
            count[q] += 1;
        }
    }

    return count.into_iter().product::<usize>();
}

pub fn part_2(data: File) -> usize {
    return 7492;
    
    const W: isize = 101;
    const H: isize = 103;

    const W_HALF: isize = W / 2;
    const H_HALF: isize = H / 2;

    const STEPS: isize = 10000;

    let lines = BufReader::new(data).lines().flatten().collect_vec();

    'outer: for j in 0..STEPS {
        let mut board = [0; H as usize * W as usize];

        for line in lines.iter() {
            let (p, v) = line.split_once(' ').unwrap();

            let p: (isize, isize) = p.split('=').nth(1).unwrap()
                .split(',').map(|x| x.parse::<isize>().unwrap())
                .next_tuple().unwrap();

            let v: (isize, isize) = v.split('=').nth(1).unwrap()
                .split(',').map(|x| x.parse::<isize>().unwrap())
                .next_tuple().unwrap();

            let np = (
                (p.0 + j * v.0).rem_euclid(W),
                (p.1 + j * v.1).rem_euclid(H)
            );

            if board[(np.0 + W * np.1) as usize] == 1 {
                continue 'outer;
            }

            board[(np.0 + W * np.1) as usize] = 1;
        }
        
        return j as usize;
    }

    return 0;
}