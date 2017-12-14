extern crate knot_hash;

const INPUT: &str = include_str!("input/day10.txt");

fn main() {
    let lengths: Vec<u8> = INPUT
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut nums: Vec<u8> = (0..256 as usize)
        .map(|v| v as u8)
        .collect();
    let mut skip_size = 0;
    let mut position = 0;

    knot_hash::round(&mut nums, &lengths, &mut skip_size, &mut position);
    let multiplied = nums[0] as u32 * nums[1] as u32;

    println!("{}", multiplied);

    let key: Vec<u8> = INPUT
        .trim()
        .as_bytes()
        .to_owned();

    let dense = knot_hash::hash(key);

    let string: String = dense.iter()
        .map(|v| format!("{:02x}", v))
        .collect();

    println!("{}", string);
}

