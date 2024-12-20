use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;
use itertools::Itertools;

fn parse_program(data: File) -> (Vec<usize>, [usize; 3]) {
    let mut lines = BufReader::new(data).lines().flatten();

    let registers = [
        lines.next().unwrap().split(':').nth(1).unwrap().trim().parse::<usize>().unwrap(),
        lines.next().unwrap().split(':').nth(1).unwrap().trim().parse::<usize>().unwrap(),
        lines.next().unwrap().split(':').nth(1).unwrap().trim().parse::<usize>().unwrap(),
    ];

    let code = lines.nth(1).unwrap()
        .split(':').nth(1).unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    return (code, registers);
}

fn eval_program(
    code: &[usize],
    registers: &[usize; 3],
    mut pointer_callback: impl FnMut(usize),
) -> Vec<usize> {
    let mut current = registers.clone();
    let mut output = Vec::new();
    let mut pointer = 0;

    while pointer + 1 < code.len() {
        pointer_callback(pointer);

        let op_code = code[pointer];
        let operand = code[pointer + 1];

        let as_combo_operand = || match operand {
            0..=3 => operand,
            4..=6 => current[operand - 4],
            _ => panic!("Invalid program"),
        };

        pointer += 2;

        match op_code {
            0 => {
                current[0] = current[0] / 2usize.pow(as_combo_operand() as u32);
            },
            1 => {
                current[1] ^= operand;
            },
            2 => {
                current[1] = as_combo_operand() % 8;
            },
            3 => {
                if current[0] != 0 {
                    pointer = operand;
                }
            },
            4 => {
                current[1] ^= current[2];
            },
            5 => {
                output.push(as_combo_operand() % 8);
            },
            6 => {
                current[1] = current[0] / 2usize.pow(as_combo_operand() as u32);
            },
            7 => {
                current[2] = current[0] / 2usize.pow(as_combo_operand() as u32);
            },
            _ => { panic!("Invalid program"); },
        };
    }

    return output;
}

pub fn part_1(data: File) -> String {
    let (code, registers) = parse_program(data);
    return eval_program(&code, &registers, |_| {}).into_iter().map(|x| x.to_string()).join(",");
}

pub fn part_2(data: File) -> usize {
    let (code, _) = parse_program(data);

    let mut k = 0;
    let mut nodes: Vec<(usize, usize)> = Vec::new();
    let mut next_nodes = Vec::new();
    
    nodes.push((0, 0));
    
    // nodes represents possible initial bits for register A,
    // each iteration depends on bits A[3k...3(k+1)] 
    // and for m=((A >> 3k) % 8)^1 on A[3k+m...3(k+1)+m]
    // hence each iteration we guess the bits that the iteration depends upon, check whether
    // the output up to that iteration is correct and add the guesses that are valid
    
    while k < code.len() {
        while let Some((bits, bit_count)) = nodes.pop() {
            // first we ensure every completion of bits up to A[...3(k+1)] is considered
            let rem_bits_p1 = 3 - (bit_count - 3 * k).min(3);
            let bit_count_p1 = bit_count.max(3 * (k + 1));
            
            for bits_p1_comp in 0..(1 << rem_bits_p1) {
                let bits_p1 = (bits_p1_comp << bit_count) | bits;
                
                // now, A[3k+m...3(k+1)+m] with m=((A >> 3k) % 8)^1 is also used by iteration k
                // and must therefore also be completed
                let m = ((bits_p1 >> 3 * k) % 8)^1;
                let rem_bits_p2 = (3 + m) - (bit_count_p1 - 3 * k).min(3 + m);
                let bit_count_p2 = bit_count_p1.max(3 * (k + 1) + m);
                
                for bits_p2_comp in 0..(1 << rem_bits_p2) {
                    let bits_p2 = (bits_p2_comp << bit_count_p1) | bits_p1;
                    
                    let bits_p2_output = eval_program(&code, &[bits_p2, 0, 0], |_| {});
                    
                    if k < bits_p2_output.len() && bits_p2_output.len() <= code.len() && (0..=k).all(|i| bits_p2_output[i] == code[i]) {
                        next_nodes.push((bits_p2, bit_count_p2));
                    }
                }
            }
        }
        
        swap(&mut nodes, &mut next_nodes);
        k += 1;
    }
    
    return nodes.into_iter().map(|(a, _)| a).min().unwrap();
}