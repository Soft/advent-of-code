extern crate knot_hash;
extern crate bit_vec;

use std::collections::{HashSet, VecDeque};
use bit_vec::BitVec;

const INPUT: &str = "jzgqcdpd";

fn neighbors((x, y): (usize, usize), bounds: usize) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    vec![(x-1, y), (x, y-1), (x+1, y), (x, y+1)]
        .into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < bounds as isize && y < bounds as isize)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn mark_region(area: &[BitVec], visited: &mut HashSet<(usize, usize)>, start: (usize, usize)) {
    let bounds = area.len();
    let mut stack = VecDeque::new();
    stack.push_back(start);
    while let Some((x, y)) = stack.pop_front() {
        if area[x][y] && !visited.contains(&(x, y)) {
            visited.insert((x, y));
            for neighbor in neighbors((x, y), bounds) {
                stack.push_back(neighbor);
            }
        }
    }
}

fn main() {
    let hashes: Vec<[u8; 16]> = (0..128)
        .map(|i| format!("{}-{}", INPUT, i).as_bytes().to_owned())
        .map(knot_hash::hash)
        .collect();
    let reserved: u32 = hashes
        .iter()
        .map(|h| h.iter()
             .map(|b| b.count_ones())
             .sum::<u32>())
        .sum();
    println!("{}", reserved);

    let bits: Vec<_> = hashes.iter()
        .map(|r| BitVec::from_bytes(r))
        .collect();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut counter = 0;

    for x in 0..128 {
        for y in 0..128 {
            if !seen.contains(&(x,y)) && bits[x][y] {
                mark_region(&bits, &mut seen, (x, y));
                counter += 1;
            }
        }
    }

    println!("{}", counter);

}


