extern crate knot_hash;

const INPUT: &str = "jzgqcdpd";

fn main() {
    let hashes: Vec<[u8; 16]> = (0..128)
        .map(|i| format!("{}-{}", INPUT, i).as_bytes().to_owned())
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


