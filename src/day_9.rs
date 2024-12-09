use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::repeat;
use crate::common::Matrix;

#[derive(Debug, Clone)]
struct Chunk {
    id: Option<usize>,
    size: usize,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.id.clone() {
            write!(f, "{}", String::from_iter(repeat(id.to_string().as_str()).take(self.size)))?;
        } else {
            write!(f, "{}", String::from_iter(repeat('_').take(self.size)))?;
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
            if chunk.size != 0 {
                total += id * (chunk.size * offset + (chunk.size * (chunk.size - 1)) / 2);
            }
        }

        offset += chunk.size;
    }

    return total;
}

fn compact(chunks: &mut Vec<Chunk>) {
    let mut free_pointer = 0;
    let mut data_pointer = chunks.len() - 1;

    loop {
        while free_pointer < chunks.len() && chunks[free_pointer].id.is_some() {
            free_pointer += 1;
        }

        while data_pointer != 0 && chunks[data_pointer].id.is_none() {
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
            chunks[free_pointer].size = rem;
            chunks.insert(free_pointer, chunks[data_pointer].clone());
            data_pointer += 1;
            chunks[data_pointer].id = None;
        }
    };
}

fn compact_whole(chunks: &mut Vec<Chunk>) {
    let mut data_last_id = usize::MAX;
    let mut data_pointer = chunks.len() - 1;

    loop {
        while data_pointer != 0 && (chunks[data_pointer].id.is_none() || chunks[data_pointer].id.is_some_and(|id| id >= data_last_id)) {
            data_pointer -= 1;
        }

        if data_pointer == 0 {
            break;
        }

        data_last_id = chunks[data_pointer].id.clone().unwrap();
        
        let mut free_pointer = 0;
        
        while free_pointer < data_pointer
            && (chunks[free_pointer].id.is_some() 
            || chunks[free_pointer].size < chunks[data_pointer].size) {
            free_pointer += 1;
        }
        
        if free_pointer >= data_pointer {
            continue;
        }
        
        if chunks[free_pointer].size == chunks[data_pointer].size {
            chunks[free_pointer].id = chunks[data_pointer].id.clone();
            chunks[data_pointer].id = None;
        } else {
            chunks[free_pointer].size -= chunks[data_pointer].size;
            chunks.insert(free_pointer, Chunk { id: chunks[data_pointer].id.clone(), size: chunks[data_pointer].size });
            data_pointer += 1;
            free_pointer += 1;
            chunks[data_pointer].id = None;
        }
        
        let mut free_pointer = data_pointer;
        
        while free_pointer > 0 && chunks[free_pointer - 1].id.is_none() {
            free_pointer -= 1;
        }
        
        data_pointer = free_pointer - 1;
        
        let mut next_non_free_pointer = free_pointer;
        let mut size = chunks[free_pointer].size;
        
        while next_non_free_pointer < chunks.len() - 1 && chunks[next_non_free_pointer + 1].id.is_none() {
            next_non_free_pointer += 1;
            size += chunks[next_non_free_pointer].size;
        }
        
        chunks[free_pointer].size = size;
        (free_pointer + 1..=next_non_free_pointer).rev().for_each(|idx| { chunks.remove(idx); });
    };
}

pub fn part_1(data: File) -> usize {
    let mut chunks = parse_disk(BufReader::new(data).lines().flatten().next().unwrap());
    compact(&mut chunks);
    return checksum(&chunks);
}

pub fn part_2(data: File) -> usize {
    let mut chunks = parse_disk(BufReader::new(data).lines().flatten().next().unwrap());
    compact_whole(&mut chunks);
    return checksum(&chunks);
}