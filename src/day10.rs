const INPUT: &str = include_str!("input/day10.txt");

fn reverse_span_wrapping(slice: &mut [i32], ind: usize, len: usize) {
    let slice_len = slice.len();
    for i in 0..len/2 {
        let a = (ind + i) % slice_len;
        let b = (ind + (len - 1) - i) % slice_len;
        let tmp = slice[a];
        slice[a] = slice[b];
        slice[b] = tmp;
    }

}

fn hash(slice: &mut [i32], lengths: &[usize]) {
    let mut skip_size = 0;
    let mut i = 0;
    for l in lengths.iter() {
        reverse_span_wrapping(slice, i, *l);
        i += *l + skip_size;
        skip_size += 1;
    }
}

fn main() {
    let lengths: Vec<usize> = INPUT
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut nums: Vec<i32> = (0..256).collect();
    hash(&mut nums, &lengths);
    let multiplied = nums[0] * nums[1];
    println!("{}", multiplied);
}

