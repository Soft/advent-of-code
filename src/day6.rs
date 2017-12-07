use std::collections::HashSet;

const INPUT: &str = include_str!("input/day6.txt");

fn cycle(banks: &Vec<u32>) -> Vec<u32> {
    let len = banks.len();
    let (i, v) = banks
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, v)| v)
        .unwrap();
    let mut result = banks.clone();
    result[i] = 0;
    for n in 0..*v as usize {
        result[(i as usize + 1 + n) % len] += 1;
    }
    result
}

fn main() {
    let start: Vec<u32> = INPUT
        .trim()
        .split('\t')
        .map(|s| s
             .parse()
             .unwrap())
        .collect();
    let mut seen = HashSet::new();
    let mut steps = 0;
    let mut current = start;
    loop {
        let next = cycle(&current);
        steps += 1;
        if seen.contains(&next) {
            break;
        } else {
            seen.insert(next.clone());
            current = next;
        }
    }
    println!("{:?}", steps);
}

