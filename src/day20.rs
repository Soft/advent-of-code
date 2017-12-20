#[macro_use]
extern crate nom;

use nom::digit;

const INPUT: &str = include_str!("input/day20.txt");

type Triplet = (i64, i64, i64);

named!(parse_input<&str, Vec<(Triplet, Triplet, Triplet)>>,
       many1!(do_parse!(e: entry >> tag_s!("\n") >> (e))));

named!(entry<&str, (Triplet, Triplet, Triplet)>,
       do_parse!(tag_s!("p=") >> p: triplet >>
                 tag_s!(", v=") >> v: triplet >>
                 tag_s!(", a=") >> a:triplet >>
                 (p,v,a)));

named!(triplet<&str, Triplet>,
            do_parse!(tag_s!("<") >>
                      x: number >> tag_s!(",") >> y: number >> tag_s!(",") >> z: number >>
                      tag_s!(">") >> (x,y,z)));

named!(number<&str, i64>,
       do_parse!(s: opt!(tag_s!("-")) >> d: digit >>
                 (d.parse::<i64>().unwrap() * s.map(|_| -1).unwrap_or(1))));

fn distance((x1, y1, z1): Triplet, (x2, y2, z2): Triplet) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}

fn main() {
    let input = parse_input(INPUT).unwrap().1;

    let min = input.iter()
        .enumerate()
        .min_by_key(|&(_, &(_, _, v))| distance((0,0,0), v));

    println!("{}", min.unwrap().0);
}


