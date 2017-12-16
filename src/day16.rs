#[macro_use]
extern crate nom;

use nom::digit;

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
                for i in 0..n {
                    slice[i] = end[i];
                }
                for i in 0..len-n {
                    slice[i+n] = start[i];
                }
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


fn main() {
    let moves = parse_input(INPUT).unwrap().1;
    let mut arrangement: Vec<char> = (0..16).map(|i| ('a' as u8 + i) as char).collect();
    moves.iter().for_each(|m| m.perform(&mut arrangement));
    println!("{}", arrangement.iter().collect::<String>());
}

