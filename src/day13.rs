#[macro_use]
extern crate nom;

use nom::digit;

use std::collections::HashMap;
use std::iter::FromIterator;

const INPUT: &str = include_str!("input/day13.txt");

#[derive(Clone, Debug)]
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

    fn step(&mut self) {
        self.i += self.offset;
        if self.i == self.range - 1 || (self.i == 0 && self.offset < 0) {
            self.offset *= -1;
        }
    }
}

named!(parse_input<&str, HashMap<usize, Scanner>>,
       map!(many0!(do_parse!(k: digit >> tag_s!(": ") >> v: digit >> tag_s!("\n") >>
                             ((k.parse().unwrap(),
                               Scanner::new(v.parse().unwrap()))))),
           HashMap::from_iter));

fn calculate_severity(layers: usize, mut scanners: &mut HashMap<usize, Scanner>)
                      -> Option<i32> {
    let mut severity = None;
    for i in 0..layers+1 {
        if let Some(current) = scanners.get(&i) {
            if current.position() == 0 {
                let severity = severity.get_or_insert(0);
                *severity += i as i32 * current.range();
            }
        }
        step_scanners(&mut scanners);
    }
    severity
}

fn step_scanners(scanners: &mut HashMap<usize, Scanner>) {
    scanners.values_mut().for_each(|s| s.step());
}

fn main() {
    let scanners = parse_input(INPUT)
        .unwrap().1;
    let layers = scanners.keys()
        .max()
        .unwrap()
        .to_owned();

    let mut scanners1 = scanners.clone();

    let severity = calculate_severity(layers, &mut scanners1)
        .unwrap_or(0);

    println!("{}", severity);

    let mut scanners2 = scanners.clone();

    let wait = (1..).find(|_| {
        step_scanners(&mut scanners2);
        calculate_severity(layers, &mut scanners2.clone()).is_none()
    }).unwrap();

    println!("{}", wait);
}

