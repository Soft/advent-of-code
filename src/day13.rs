use std::collections::HashMap;

const INPUT: &str = include_str!("input/day13.txt");

struct Scanner {
    i: i32,
    range: i32,
    offset: i32,
}

impl Scanner {
    fn new(range: i32) -> Scanner {
        Scanner {i: 0, range, offset: 1}
    }

    fn position(&self) -> i32 {
        self.i
    }

    fn range(&self) -> i32 {
        self.range
    }
}

impl Iterator for Scanner {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let v = self.i;
        self.i += self.offset;
        if self.i == self.range - 1 || self.i == 0 {
            self.offset *= -1;
        }
        Some(v)
    }
}

fn parse_input(input: &str) -> HashMap<usize, Scanner> {
    let mut scanners = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(':');
        let layer = parts.next().unwrap().parse().unwrap();
        let range: i32 = parts.next().unwrap().trim().parse().unwrap();
        scanners.insert(layer, Scanner::new(range));
    }
    scanners
}

fn main() {
    let mut scanners = parse_input(INPUT);
    let layers = scanners.keys()
        .max()
        .unwrap()
        .to_owned();
    let mut severity = 0;

    for i in 0..layers+1 {
        if let Some(current) = scanners.get(&i) {
            if current.position() == 0 {
                severity += i as i32 * current.range();
            }
        }
        scanners.values_mut().for_each(|s| { s.next(); });
    }

    println!("{}", severity);
}

