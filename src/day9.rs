#[macro_use]
extern crate nom;

const INPUT: &[u8] = include_bytes!("input/day9.txt");

named_args!(group(n: u32)<(u32, u32)>,
            do_parse!(tag_s!("{") >>
                      v: separated_list!(tag_s!(","),
                                         alt!(call!(group, n+1) |
                                              map!(garbage, |n| (0, n)))) >>
                      tag_s!("}") >>
                      ({
                          let i: u32 = v.iter().map(|&(v,_)| v).sum();
                          let g: u32 = v.iter().map(|&(_,v)| v).sum();
                          (n + i, g)
                      })));

named!(garbage<u32>,
       delimited!(tag_s!("<"),
                  map!(many0!(
                      alt!(value!(0, escape) |
                           value!(1, none_of!(">")))),
                  |v| v.iter().sum()),
                 tag_s!(">")));

named!(escape<()>,
       do_parse!(tag_s!("!") >> take_s!(1) >> (())));

fn main() {
    let (sum, garbage) = group(INPUT, 1).unwrap().1;
    println!("{}", sum);
    println!("{}", garbage);
}

