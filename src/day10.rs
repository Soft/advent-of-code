extern crate itertools;
use itertools::Itertools;

const INPUT: &str = include_str!("input/day10.txt");
const SUFFIX: &[usize] = &[17, 31, 73, 47, 23];

fn reverse_span_wrapping<T: Copy>(slice: &mut [T], ind: usize, len: usize) {
    let slice_len = slice.len();
    for i in 0..len/2 {
        let a = (ind + i) % slice_len;
        let b = (ind + (len - 1) - i) % slice_len;
        let tmp = slice[a];
        slice[a] = slice[b];
        slice[b] = tmp;
    }

}

fn round<T: Copy>(slice: &mut [T], lengths: &[usize], skip_size: &mut usize, i: &mut usize) {
    for l in lengths.iter() {
        reverse_span_wrapping(slice, *i, *l);
        *i += *l + *skip_size;
        *skip_size += 1;
    }
}

fn hash(slice: &mut [usize], lengths: &[usize]) -> Vec<usize> {
    let mut skip_size = 0;
    let mut position = 0;
    for _ in 0..64 {
        round(slice, lengths, &mut skip_size, &mut position);
    }
    to_dense(slice)
}

fn to_dense(slice: &[usize]) -> Vec<usize> {
    slice.into_iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk
             .cloned()
             .fold1(|a, b| a ^ b)
             .unwrap())
        .collect()
}

fn main() {
    let lengths: Vec<usize> = INPUT
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut nums: Vec<i32> = (0..256).collect();
    let mut skip_size = 0;
    let mut position = 0;
    round(&mut nums, &lengths, &mut skip_size, &mut position);
    let multiplied = nums[0] * nums[1];
    println!("{}", multiplied);

    let mut lengths: Vec<usize> = INPUT
        .trim()
        .as_bytes()
        .iter()
        .map(|b| *b as usize)
        .collect();
    lengths.extend(SUFFIX);

    let mut nums: Vec<usize> = (0..256).collect();

    let dense = hash(&mut nums, &lengths);

    let string: String = dense.iter()
        .map(|v| format!("{:02x}", v))
        .collect();

    println!("{}", string);
}

