extern crate itertools;
use itertools::Itertools;

const INPUT: &str = include_str!("input/day1.txt");

fn main() {
    let digits = INPUT
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let digits = {
        let first = digits[0].clone();
        let mut digits = digits;
        digits.push(first);
        digits
    };
    let sum: u32 = digits
        .iter()
        .tuple_windows::<(_, _)>()
        .filter_map(|(a, b)| if a==b { Some(a) } else { None })
        .sum();
    println!("{}", sum);
}

