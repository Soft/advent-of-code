extern crate knot_hash;

const INPUT: &str = "jzgqcdpd";

fn main() {
    let hashes: Vec<Vec<usize>> = (0..128)
        .map(|i| format!("{}-{}", INPUT, i)
             .as_bytes()
             .iter()
             .map(|c| *c as usize)
             .collect())
        .map(|b| knot_hash::hash(b))
        .collect();
    let reserved: u32 = hashes
        .into_iter()
        .map(|h| h.iter()
             .map(|b| b.count_ones())
             .sum::<u32>())
        .sum();
    println!("{}", reserved);
}


