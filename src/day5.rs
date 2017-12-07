const INPUT: &str = include_str!("input/day5.txt");

fn main() {
    let mut mem: Vec<_> = INPUT
        .lines()
        .map(|s| s
             .parse::<i32>()
             .unwrap())
        .collect();
    let mut ind = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let val = mem[ind] + ind as i32; // Yes this could overflow
        mem[ind] += 1;
        if val < 0 || val >= mem.len() as i32 {
            break;
        } else {
            ind = val as usize;
        }
    };
    println!("{}", steps);
}

