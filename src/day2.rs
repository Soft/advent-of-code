const INPUT: &str = include_str!("input/day2.txt");

fn main() {
    let checksum: i32 = INPUT
        .lines()
        .map(|s| {
            let digits: Vec<i32> = s.trim()
                .split('\t')
                .map(|f| f
                     .parse()
                     .unwrap())
                .collect();
            let max = digits.iter().max().unwrap();
            let min = digits.iter().min().unwrap();
            max - min
        }).sum();
    println!("{}", checksum);
}
