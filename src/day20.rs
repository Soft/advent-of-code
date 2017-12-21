#[macro_use]
extern crate nom;
extern crate itertools;

use nom::digit;
use itertools::Itertools;

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

fn grouped_min_by<I, E, F, K>(iter: I, key: F) -> Vec<E>
    where I: Iterator<Item=E>,
          F: FnMut(&E) -> K,
          K: PartialEq + Ord + Clone {
    let grouped = iter.group_by(key);
    let grouped = grouped.into_iter()
        .min_by_key(|&(ref k, _)| k.clone())
        .map(|v| v.1.collect::<Vec<_>>())
        .unwrap_or_else(|| vec![]);
    grouped
}

fn main() {
    let input = parse_input(INPUT).unwrap().1;
    let input: Vec<_> = input.into_iter().enumerate().collect();

    let mut min = grouped_min_by(input.into_iter(), |&(_, (_, _, a))| distance((0, 0, 0), a));
    if min.len() > 1 {
        min = grouped_min_by(min.into_iter(), |&(_, (_, v, _))| distance((0, 0, 0), v));
        if min.len() > 1 {
            min = grouped_min_by(min.into_iter(), |&(_, (p, _, _))| distance((0, 0, 0), p));
        }
    }

    println!("{}", min[0].0);
}


