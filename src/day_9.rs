use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use crate::common::Matrix;

#[derive(Debug, Clone)]
struct Chunk {
    id: Option<usize>,
    size: usize,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.id.clone() {
            write!(f, "D{id}S{}", self.size)?;
        } else {
            write!(f, "F0S{}", self.size)?;
        }

        Ok(())
    }
}

fn parse_disk(line: String) -> Vec<Chunk> {
    let mut chunks = Vec::with_capacity(line.len());

    for (idx, c) in line.chars().enumerate() {
        chunks.push(Chunk {
            id: if idx % 2 == 0 { Some(idx / 2) } else { None },
            size: c.to_digit(10).unwrap() as usize,
        });
    }

    return chunks;
}

fn checksum(chunks: &Vec<Chunk>) -> usize {
    let mut offset = 0;
    let mut total = 0;

    for chunk in chunks.iter() {
        if let Some(id) = chunk.id.clone() {
            total += id * (chunk.size * offset + (chunk.size * (chunk.size - 1)) / 2);
        }

        offset += chunk.size;
    }

    return total;
}

fn compact(chunks: &mut Vec<Chunk>) {
    let mut free_pointer = 0;
    let mut data_pointer = chunks.len() - 1;

    loop {
        while free_pointer < chunks.len() && chunks[free_pointer].id.is_none() {
            free_pointer += 1;
        }

        while data_pointer != 0 && chunks[data_pointer].id.is_some() {
            data_pointer -= 1;
        }

        if free_pointer > data_pointer {
            break;
        }

        if chunks[free_pointer].size <= chunks[data_pointer].size {
            chunks[free_pointer].id = chunks[data_pointer].id.clone();

            if chunks[free_pointer].size == chunks[data_pointer].size {
                chunks[data_pointer].id = None;
            } else {
                chunks[data_pointer].size -= chunks[free_pointer].size;
                chunks.insert(data_pointer + 1, Chunk {
                    id: None,
                    size: chunks[free_pointer].size,
                });
            }
        } else {
            let rem = chunks[free_pointer].size - chunks[data_pointer].size;
            chunks[free_pointer].id = chunks[data_pointer].id.clone();
            chunks[data_pointer].id = None;
            chunks[free_pointer].size -= chunks[data_pointer].size;
            chunks.insert(free_pointer + 1, Chunk { id: None, size: rem });
            data_pointer -= 1;
        }
    };
}

pub fn part_1(data: File) -> usize {
    let mut chunks = parse_disk(BufReader::new(data).lines().flatten().next().unwrap());

    println!("{}", String::from_iter(chunks.iter().map(|c| format!("{c} "))));
    compact(&mut chunks);
    println!("{}", String::from_iter(chunks.iter().map(|c| format!("{c} "))));
    println!("{}", checksum(&chunks));

    return 0;
}

pub fn part_2(data: File) -> usize {
    return 0;
}