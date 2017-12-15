use std::collections::VecDeque;

const INPUT: (u64, u64) = (277, 349);

fn main() {
    let mut count = 0;
    let mult_a = 16807;
    let mult_b = 48271;
    let divider = 2147483647;
    let mask = u16::max_value() as u64;
    let mut prev_a = INPUT.0;
    let mut prev_b = INPUT.1;

    for _ in 0..40_000_000 {
        prev_a = prev_a * mult_a % divider;
        prev_b = prev_b * mult_b % divider;
        if prev_a & mask == prev_b & mask {
            count += 1;
        }
    }

    println!("{}", count);

    let mut a_stack = VecDeque::new();
    let mut b_stack = VecDeque::new();
    let mut comparisons = 0;

    prev_a = INPUT.0;
    prev_b = INPUT.1;
    count = 0;

    while comparisons < 5_000_000 {
        prev_a = prev_a * mult_a % divider;
        prev_b = prev_b * mult_b % divider;
        if prev_a % 4 == 0 {
            a_stack.push_back(prev_a);
        }
        if prev_b % 8 == 0 {
            b_stack.push_back(prev_b);
        }
        if !a_stack.is_empty() && !b_stack.is_empty() {
            let a_val = a_stack.pop_front().unwrap();
            let b_val = b_stack.pop_front().unwrap();
            comparisons += 1;
            if a_val & mask == b_val & mask {
                count += 1;
            }
        }
    }

    println!("{}", count);
}


