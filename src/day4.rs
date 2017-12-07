extern crate itertools;
use itertools::Itertools;

const INPUT: &str = include_str!("input/day4.txt");

fn main() {
    let count = INPUT
        .lines()
        .filter(|s| {
            let words: Vec<_> = s.split(' ').collect();
            words.iter().unique().count() == words.len()
        })
        .count();
    println!("{}", count);
}

