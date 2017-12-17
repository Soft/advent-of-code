#[macro_use]
extern crate nom;

use nom::digit;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

const INPUT: &str = include_str!("input/day16.txt");

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl Move {
    fn perform(&self, slice: &mut [char]) {
        match *self {
            Move::Spin(n) => {
                let len = slice.len();
                let copy = slice.to_owned();
                let (start, end) = copy.split_at(len - n);
                slice[..n].clone_from_slice(&end[..n]);
                slice[n..(len-n + n)].clone_from_slice(&start[..len-n]);
            },
            Move::Exchange(a, b) => {
                slice.swap(a, b);
            },
            Move::Partner(a, b) => {
                let a = slice.iter().position(|c| *c == a).unwrap();
                let b = slice.iter().position(|c| *c == b).unwrap();
                slice.swap(a, b);
            }
        }
    }
}

named!(parse_input<&str, Vec<Move>>,
       separated_list!(tag_s!(","), alt!(spin | exchange | partner)));

named!(spin<&str, Move>,
       do_parse!(tag_s!("s") >> v: digit >>
                 (Move::Spin(v.parse().unwrap()))));

named!(exchange<&str, Move>,
       do_parse!(tag_s!("x") >> a: digit >> tag_s!("/") >> b: digit >>
                 (Move::Exchange(a.parse().unwrap(),
                                 b.parse().unwrap()))));

named!(partner<&str, Move>,
       do_parse!(tag_s!("p") >>
                 a: map!(take_s!(1), |s| s.chars().next().unwrap()) >>
                 tag_s!("/") >>
                 b: map!(take_s!(1), |s| s.chars().next().unwrap()) >>
                 (Move::Partner(a, b))));

fn run_program(program: &[Move], mut arrangement: &mut [char]) {
    for m in program {
        m.perform(&mut arrangement);
    }
}

fn main() {
    let moves = parse_input(INPUT).unwrap().1;
    let mut arrangement: Vec<char> = (0..16).map(|i| (b'a' as u8 + i) as char).collect();
    moves.iter().for_each(|m| m.perform(&mut arrangement));
    println!("{}", arrangement.iter().collect::<String>());

    let iterations = 1_000_000_000-1;
    let mut cache: HashMap<Vec<char>, (Vec<char>, usize)> = HashMap::new();
    let mut i = 0;

    while i < iterations {
        match cache.entry(arrangement.clone()) {
            Entry::Occupied(v) => {
                let &(ref result, ref n) = v.get();
                let skip = 2*i - *n;
                arrangement = result.to_owned();
                if skip < iterations {
                    i = skip;
                }
            },
            Entry::Vacant(v) => {
                run_program(&moves, &mut arrangement);
                v.insert((arrangement.clone(), i));
            }
        }
        i += 1;
    }

    println!("{}", arrangement.iter().collect::<String>());
}

