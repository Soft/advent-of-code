extern crate itertools;
use itertools::Itertools;

const SUFFIX: &[usize] = &[17, 31, 73, 47, 23];

fn reverse_span_wrapping<T: Copy>(slice: &mut [T], ind: usize, len: usize) {
    let slice_len = slice.len();
    for i in 0..len/2 {
        let a = (ind + i) % slice_len;
        let b = (ind + (len - 1) - i) % slice_len;
        slice.swap(a, b);
    }

}

pub fn round<T: Copy>(slice: &mut [T], lengths: &[usize], skip_size: &mut usize, i: &mut usize) {
    for l in lengths.iter() {
        reverse_span_wrapping(slice, *i, *l);
        *i += *l + *skip_size;
        *skip_size += 1;
    }
}

pub fn hash(mut lengths: Vec<usize>) -> Vec<usize> {
    lengths.extend(SUFFIX);

    let mut base: Vec<usize> = (0..256).collect();
    let mut skip_size = 0;
    let mut position = 0;

    for _ in 0..64 {
        round(&mut base, &lengths, &mut skip_size, &mut position);
    }

    to_dense(&base)
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


