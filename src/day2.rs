extern crate itertools;
use itertools::Itertools;

const INPUT: &str = include_str!("input/day2.txt");

fn main() {
    let table: Vec<Vec<i32>> = INPUT
        .lines()
        .map(|s| {
            s.trim()
                .split('\t')
                .map(|f| f
                     .parse()
                     .unwrap())
                .collect()
        }).collect();

    let checksum1: i32 = table
        .iter()
        .map(|r| {
            let max = r.iter().max().unwrap();
            let min = r.iter().min().unwrap();
            max - min
        }).sum();

    let checksum2: i32 = table
        .iter()
        .map(|r| r.iter()
             .tuple_combinations()
             .flat_map(|(a, b)| vec![(a, b), (b, a)])
             .find(|&(a, b)| a % b == 0)
             .map(|(a, b)| a / b)
             .unwrap())
        .sum();

    println!("{}", checksum1);
    println!("{}", checksum2);
}
