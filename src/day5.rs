const INPUT: &str = include_str!("input/day5.txt");

fn execute<F>(mem: &mut [i32], transform: F) -> u32
    where F: Fn(i32) -> i32 {
    let mut ind = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let val = mem[ind] + ind as i32; // Yes this could overflow
        mem[ind] = transform(mem[ind]);
        if val < 0 || val >= mem.len() as i32 {
            return steps;
        } else {
            ind = val as usize;
        }
    };
}

fn main() {
    let mut mem1: Vec<i32> = INPUT
        .lines()
        .map(|s| s
             .parse()
             .unwrap())
        .collect();
    let mut mem2 = mem1.clone();

    let steps = execute(&mut mem1, |v| v+1);
    println!("{}", steps);

    let steps = execute(&mut mem2, |v| if v >= 3 { v-1 } else { v+1 });
    println!("{}", steps);
}

