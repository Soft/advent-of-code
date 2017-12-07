extern crate petgraph;
#[macro_use]
extern crate nom;

use petgraph::Direction;
use petgraph::graphmap::DiGraphMap;
use nom::{alpha, digit};

const INPUT: &str = include_str!("input/day7.txt");

type Entry<'a> = (&'a str, u32, Vec<&'a str>);

named!(parse_input<&str, Vec<Entry>>, many1!(entry));

named!(entry<&str, Entry>,
       do_parse!(x: alpha >>
                 tag_s!(" (") >>
                 w: digit >>
                 tag_s!(")") >>
                 cs: opt!(list) >>
                 tag_s!("\n") >>
                 ((x, w.parse().unwrap(), cs.unwrap_or(vec![])))));

named!(list<&str, Vec<&str>>,
       do_parse!(tag_s!(" -> ") >> x: alpha >> xs: many0!(list_tail) >> ({
           let mut res = Vec::new();
           res.push(x);
           res.extend(xs);
           res
       })));

named!(list_tail<&str, &str>,
       do_parse!(tag_s!(", ") >> x: alpha >> (x)));

fn main() {
    let entries = parse_input(INPUT)
        .unwrap().1;

    let mut graph: DiGraphMap<&str, ()> = DiGraphMap::new();
    for &(ref e, _, ref cs) in entries.iter() {
        graph.add_node(e);
        cs.iter().for_each(|c| {
            graph.add_edge(e, c, ());
        });
    }

    // Since we know the input will be a valid tree this should be fine
    let mut node = graph
        .nodes()
        .next()
        .unwrap();
    loop {
        let mut p = graph.neighbors_directed(node, Direction::Incoming);
        match p.next() {
            Some(p) => node = p,
            None => break
        }
    }

    println!("{}", node);
}

