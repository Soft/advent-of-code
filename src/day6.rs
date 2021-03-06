use std::collections::HashSet;

const INPUT: &str = include_str!("input/day6.txt");

fn cycle(banks: &[u32]) -> Vec<u32> {
    let len = banks.len();
    let (i, v) = banks
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, v)| v)
        .unwrap();
    let mut result = banks.to_owned();
    result[i] = 0;
    for n in 0..*v as usize {
        result[(i as usize + 1 + n) % len] += 1;
    }
    result
}

fn main() {
    let mut current: Vec<u32> = INPUT
        .trim()
        .split('\t')
        .map(|s| s
             .parse()
             .unwrap())
        .collect();
    let mut seen = HashSet::new();
    let mut steps = 0;
    let mut next;
    loop {
        next = cycle(&current);
        steps += 1;
        if seen.contains(&next) {
            break;
        } else {
            seen.insert(next.clone());
            current = next;
        }
    }
    println!("{}", steps);

    let target = next.clone();
    current = next;
    for i in 1.. {
        next = cycle(&current);
        if next == target {
            println!("{}", i);
            break;
        } else {
            current = next;
        }
    }
}

