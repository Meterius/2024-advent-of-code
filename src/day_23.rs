use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;
use itertools::{Itertools};

pub fn part_1(data: File) -> usize {
    let get_id = |name: &str| name.chars().enumerate().map(
        |(i, x)| {
            assert!(('a'..='z').contains(&x));
            (x as usize - 'a' as usize + 1) * 27usize.pow(i as u32)
        }
    ).sum::<usize>();

    let mut nodes = HashSet::new();
    let mut neighbors = HashMap::<usize, Vec<usize>>::new();
    let mut sources = HashSet::new();

    for line in BufReader::new(data).lines().flatten() {
        let (a, b) = line.split_once('-').unwrap();
        nodes.insert(get_id(a));
        nodes.insert(get_id(b));

        if a.starts_with('t') { sources.insert(get_id(a)); }
        if b.starts_with('t') { sources.insert(get_id(b)); }

        neighbors.entry(get_id(a))
            .and_modify(|v| v.push(get_id(b))).or_insert(vec![get_id(b)]);

        neighbors.entry(get_id(b))
            .and_modify(|v| v.push(get_id(a))).or_insert(vec![get_id(a)]);
    }

    let mut total = 0;
    let mut triplets = HashSet::new();

    for source in sources.iter() {
        for second in neighbors[source].iter() {
            for third in neighbors[second].iter() {
                if second < third && neighbors[source].contains(third) {
                    let mut triple = [source.clone(), second.clone(), third.clone()];
                    triple.sort();
                    total += if triplets.insert(triple) { 1 } else { 0 };
                }
            }
        }
    }

    return total;
}

pub fn part_2(data: File) -> String {
    let mut nodes = HashSet::new();
    let mut neighbors = HashMap::<usize, Vec<usize>>::new();

    let get_id = |name: &str| name.chars().enumerate().map(
        |(i, x)| {
            assert!(('a'..='z').contains(&x));
            (x as usize - 'a' as usize + 1) * 27usize.pow(i as u32)
        }
    ).sum::<usize>();

    let from_id = |id: usize| {
        let mut res = String::new();

        let mut rem = id;
        while rem != 0 {
            res.push(('a' as u8 + (rem % 27) as u8 - 1) as char);
            rem /= 27;
        }

        return res;
    };

    for line in BufReader::new(data).lines().flatten() {
        let (a, b) = line.split_once('-').unwrap();

        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 2);

        nodes.insert(get_id(a));
        nodes.insert(get_id(b));

        neighbors.entry(get_id(a))
            .and_modify(|vs| vs.push(get_id(b)))
            .or_insert_with(|| vec![get_id(b)]);

        neighbors.entry(get_id(b))
            .and_modify(|vs| vs.push(get_id(a)))
            .or_insert_with(|| vec![get_id(a)]);
    }

    let mut cliques: HashSet<BTreeSet<usize>, std::hash::RandomState> = HashSet::from_iter(
        nodes.iter().map(|v| BTreeSet::from([v.clone()]))
    );

    let mut next = HashSet::new();

    while cliques.len() > 1 {
        for clique in cliques.drain() {
            let v = clique.iter().next().unwrap().clone();

            for w in neighbors[&v].iter() {
                if !clique.contains(w) && clique.iter().all(|u| neighbors[u].contains(w)) {
                    let mut next_clique = clique.clone();
                    next_clique.insert(w.clone());
                    next.insert(next_clique);
                }
            }
        }

        swap(&mut next, &mut cliques);
    }

    assert_eq!(cliques.len(), 1);

    let mut max_clique = cliques
        .into_iter()
        .next()
        .unwrap()
        .into_iter()
        .map(|x| from_id(x))
        .collect::<Vec<_>>();

    max_clique.sort();

    return max_clique.join(",");
}