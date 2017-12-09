extern crate itertools;
use itertools::Itertools;

const INPUT: &str = include_str!("input/day1.txt");

fn main() {
    let digits = INPUT
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let len = digits.len();

    let sum1: u32 = digits
        .iter()
        .cycle()
        .take(len+1)
        .tuple_windows()
        .filter_map(|(a, b)| if a==b { Some(a) } else { None })
        .sum();

    let halfway_around = digits
        .iter()
        .cycle()
        .skip(len/2);
    let sum2: u32 = digits
        .iter()
        .cycle()
        .zip(halfway_around)
        .take(len)
        .filter_map(|(a, b)| if a==b { Some(a) } else { None })
        .sum();

    println!("{}", sum1);
    println!("{}", sum2);
}

