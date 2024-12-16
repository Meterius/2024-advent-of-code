use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::str::Lines;
use itertools::Itertools;

pub struct OrderedCounter<K: Ord + Copy> {
    bst: BTreeMap<K, usize>
}

impl<K: Ord + Copy> OrderedCounter<K> {
    pub fn new() -> OrderedCounter<K> {
        OrderedCounter::<K> {
            bst: BTreeMap::default()
        }
    }

    pub fn increment(&mut self, key: K) -> usize {
        self.bst.entry(key).and_modify(|v| { *v += 1; }).or_insert(1).clone()
    }

    pub fn get(&self, key: &K) -> Option<usize> {
        return self.bst.get(key).cloned();
    }

    pub fn decrement(&mut self, key: &K) -> Option<usize> {
        let x = self.bst.get_mut(key)?;
        *x -= 1;

        let x = *x;
        if x == 0 { self.bst.remove(key); }

        return Some(x);
    }

    pub fn pop_first(&mut self) -> Option<K> {
        let (&key, _) = self.bst.first_key_value()?;
        self.decrement(&key);
        return Some(key);
    }

    pub fn pop_last(&mut self) -> Option<K> {
        let (&key, _) = self.bst.last_key_value()?;
        self.decrement(&key);
        return Some(key);
    }

    pub fn is_empty(&self) -> bool {
        return self.bst.is_empty();
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<T: Debug + Display> {
    pub buffer: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Debug + Display> Matrix<T> {
    pub fn from_lines(mut lines: impl Iterator<Item=String>, mut parse: impl FnMut(char, (usize, usize)) -> T) -> Matrix<T> {
        if let Some(first_line) = lines.next() {
            let mut matrix = Matrix { buffer: Vec::new(), width: first_line.len(), height: 1 };

            for (i, x) in first_line.chars().enumerate() {
                matrix.buffer.push(parse(x, (i, matrix.height)));
            }

            for line in lines {
                for (i, x) in line.chars().enumerate() {
                    matrix.buffer.push(parse(x, (i, matrix.height)));
                }
                matrix.height += 1;
            }

            return matrix;
        } else {
            return Matrix { buffer: Vec::new(), width: 0, height: 0 };
        }
    }

    pub fn point_to_index(&self, (x, y): (isize, isize)) -> Option<usize> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            return None;
        } else {
            return Some(self.width * (y as usize) + (x as usize));
        }
    }

    pub fn index_to_point(&self, i: usize) -> Option<(isize, isize)> {
        if i < 0 || i > self.buffer.len() {
            return None;
        } else {
            return Some(((i % self.width) as isize, (i / self.width) as isize));
        }
    }

    pub fn get(&self, p: (isize, isize)) -> Option<&T> {
        return self.point_to_index(p).map(|i| &self.buffer[i]);
    }

    pub fn display_string(&self) -> String {
        let mut result = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                let idx = self.point_to_index((j as isize, i as isize)).unwrap();
                result.push_str(format!("{}", self.buffer[idx]).as_str());
            }
            result.push('\n');
        }

        return result;
    }
}
