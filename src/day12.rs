extern crate petgraph;
#[macro_use]
extern crate nom;

use nom::digit;

use petgraph::algo;
use petgraph::graphmap::UnGraphMap;
use petgraph::visit::Bfs;

const INPUT: &str = include_str!("input/day12.txt");

type Entry<'a> = (u32, Vec<u32>);

named!(parse_input<&str, Vec<Entry>>, many1!(entry));

named!(entry<&str, Entry>,
       do_parse!(n: ws!(digit) >>
                 ws!(tag_s!("<->")) >>
                 ns: separated_list!(tag_s!(", "),
                                     digit) >>
                 tag_s!("\n") >>
                 ((n.parse().unwrap(),
                   ns.into_iter()
                   .map(|n| n.parse()
                        .unwrap())
                   .collect()))));

fn main() {
    let entries = parse_input(INPUT)
        .unwrap().1;
    let graph = UnGraphMap::<_, ()>
        ::from_edges(entries
                     .into_iter()
                     .flat_map(|(n, ns)| ns
                               .into_iter()
                               .map(|c| (n, c))
                               .collect::<Vec<(u32, u32)>>())
                     .collect::<Vec<(u32, u32)>>());
    let mut bfs = Bfs::new(&graph, 0);
    let mut reachable = 0;
    while let Some(_) = bfs.next(&graph) {
        reachable += 1;
    }
    println!("{}", reachable);
    println!("{}", algo::connected_components(&graph));
}
