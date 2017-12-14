extern crate knot_hash;

const INPUT: &str = include_str!("input/day10.txt");

fn main() {
    let lengths: Vec<usize> = INPUT
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut nums: Vec<i32> = (0..256).collect();
    let mut skip_size = 0;
    let mut position = 0;
    knot_hash::round(&mut nums, &lengths, &mut skip_size, &mut position);
    let multiplied = nums[0] * nums[1];

    println!("{}", multiplied);

    let lengths: Vec<usize> = INPUT
        .trim()
        .as_bytes()
        .iter()
        .map(|b| *b as usize)
        .collect();

    let dense = knot_hash::hash(lengths);

    let string: String = dense.iter()
        .map(|v| format!("{:02x}", v))
        .collect();

    println!("{}", string);
}

