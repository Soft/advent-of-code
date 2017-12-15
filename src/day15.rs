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
}


