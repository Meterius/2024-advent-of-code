use std::fs;
use std::io;
use std::io::BufRead;

use std::collections::BTreeMap;
use std::collections::btree_map;

struct OrderedCounter<K: Ord + Copy> {
    bst: BTreeMap<K, usize>
}

impl<K: Ord + Copy> OrderedCounter<K> {
    fn new() -> OrderedCounter<K> {
        OrderedCounter::<K> {
            bst: BTreeMap::default()
        }
    }

    fn increment(&mut self, key: K) -> usize {
        self.bst.entry(key).and_modify(|v| { *v += 1; }).or_insert(1).clone()
    }

    fn get(&self, key: &K) -> Option<usize> {
        return self.bst.get(key).cloned();
    }

    fn decrement(&mut self, key: &K) -> Option<usize> {
        let x = self.bst.get_mut(key)?;
        *x -= 1;

        let x = *x;
        if x == 0 { self.bst.remove(key); }

        return Some(x);
    }

    fn pop_first(&mut self) -> Option<K> {
        let (&key, _) = self.bst.first_key_value()?;
        self.decrement(&key);
        return Some(key);
    }

    fn pop_last(&mut self) -> Option<K> {
        let (&key, _) = self.bst.last_key_value()?;
        self.decrement(&key);
        return Some(key);
    }

    fn is_empty(&self) -> bool {
        return self.bst.is_empty();
    }
}

fn day_one_parse(data: fs::File) -> usize {
    let mut left = OrderedCounter::<usize>::new();
    let mut right = OrderedCounter::<usize>::new();

    for line in io::BufReader::new(data).lines().flatten() {
        let mut numbers = line.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap());
        left.increment(numbers.next().unwrap());
        right.increment(numbers.next().unwrap());
    }

    let mut total = 0;
    while let Some(left_val) = left.pop_first() {
        total += left_val.abs_diff(right.pop_first().unwrap());
    }

    return total;
}

fn main() {
    // println!("Day One = {}", day_one_parse(fs::File::open("./data/day_1.txt").unwrap()));
}
