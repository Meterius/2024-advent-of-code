use std::collections::BTreeMap;

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