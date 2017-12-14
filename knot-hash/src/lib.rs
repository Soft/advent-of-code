extern crate itertools;
use itertools::Itertools;

const SUFFIX: &[u8] = &[17, 31, 73, 47, 23];

fn reverse_span_wrapping<T: Copy>(slice: &mut [T], ind: usize, len: u8) {
    let slice_len = slice.len();
    for i in 0..(len/2) as usize {
        let a = (ind + i) % slice_len;
        let b = (ind + (len as usize - 1) - i) % slice_len;
        slice.swap(a, b);
    }
}

pub fn round<T: Copy>(slice: &mut [T], lengths: &[u8], skip_size: &mut usize, i: &mut usize) {
    for l in lengths.iter() {
        reverse_span_wrapping(slice, *i, *l);
        *i += *l as usize + *skip_size;
        *skip_size += 1;
    }
}

pub fn hash(mut key: Vec<u8>) -> [u8; 16] {
    key.extend(SUFFIX);

    let mut base: Vec<u8> = (0..256 as usize)
        .map(|v| v as u8)
        .collect();
    let mut skip_size = 0;
    let mut position = 0;

    for _ in 0..64 {
        round(&mut base, &key, &mut skip_size, &mut position);
    }

    to_dense(&base)
}

fn to_dense(slice: &[u8]) -> [u8; 16] {
    let mut result: [u8; 16] = [0; 16];
    slice.into_iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk
             .cloned()
             .fold1(|a, b| a ^ b)
             .unwrap())
        .enumerate()
        .for_each(|(i, v)| result[i]=v);
    result
}


