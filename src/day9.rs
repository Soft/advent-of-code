#[macro_use]
extern crate nom;

const INPUT: &[u8] = include_bytes!("input/day9.txt");

named_args!(group(n: u32)<u32>,
            do_parse!(tag_s!("{") >>
                      v: separated_list!(tag_s!(","), alt!(call!(group, n+1) | value!(0, garbage))) >>
                      tag_s!("}") >>
                      ({
                          let v: u32 = v.iter().sum();
                          n + v
                      })));

named!(garbage<()>,
       delimited!(tag_s!("<"),
                  value!((), many0!(
                      alt!(escape |
                           value!((), none_of!(">"))))),
                 tag_s!(">")));

named!(escape<()>,
       do_parse!(tag_s!("!") >> take_s!(1) >> (())));

fn main() {
    let sum = group(INPUT, 1).unwrap().1;
    println!("{}", sum)
}

