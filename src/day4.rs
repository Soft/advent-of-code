extern crate itertools;
use itertools::Itertools;

use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("input/day4.txt");

fn main() {
    let rows: Vec<Vec<_>> = INPUT
        .lines()
        .map(|s| s.split(' ').collect())
        .collect();

    let count1 = rows
        .iter()
        .filter(|words| {
            words.iter().unique().count() == words.len()
        })
        .count();

    let count2 = rows
        .iter()
        .filter(|words| {
            let sets: Vec<HashSet<_>> = words
                .iter()
                .map(|w| HashSet::from_iter(w.chars()))
                .collect();
            sets.iter()
                .tuple_combinations()
                .all(|(a, b)| a != b)
        })
        .count();

    println!("{}", count1);
    println!("{}", count2);
}

