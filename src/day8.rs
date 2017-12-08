#[macro_use]
extern crate nom;

use nom::{alpha, digit};
use std::collections::HashMap;

const INPUT: &str = include_str!("input/day8.txt");

#[derive(Clone,Copy,PartialEq,Eq, Debug)]
enum Op { Inc, Dec }
#[derive(Clone,Copy,PartialEq,Eq, Debug)]
enum Cmp { Eq, Ne, Lt, Gt, Le, Ge }
type Instr<'a> = (&'a str, Op, i32, &'a str, Cmp, i32);

named!(parse_input<&str, Vec<Instr>>, many1!(instr));

named!(instr<&str, Instr>,
       do_parse!(r1: ws!(alpha) >>
                 o: ws!(op) >>
                 v1: ws!(num) >>
                 ws!(tag_s!("if")) >>
                 r2: ws!(alpha) >>
                 c: ws!(cmp) >>
                 v2: num >>
                 tag_s!("\n") >>
                 ((r1, o, v1, r2, c, v2))));

named!(cmp<&str, Cmp>,
       alt_complete!(do_parse!(tag_s!("==") >> (Cmp::Eq)) |
                     do_parse!(tag_s!("!=") >> (Cmp::Ne)) |
                     do_parse!(tag_s!("<=") >> (Cmp::Le)) |
                     do_parse!(tag_s!(">=") >> (Cmp::Ge)) |
                     do_parse!(tag_s!("<")  >> (Cmp::Lt)) |
                     do_parse!(tag_s!(">")  >> (Cmp::Gt))));

named!(op<&str, Op>,
       alt!(do_parse!(tag_s!("inc") >> (Op::Inc)) |
            do_parse!(tag_s!("dec") >> (Op::Dec))));

named!(num<&str, i32>,
       do_parse!(s: opt!(tag_s!("-")) >>
                 d: digit >>
                 (d.parse::<i32>().unwrap() * s.map(|_| -1).unwrap_or(1))));

fn check_constraint<'a>(regs: &mut HashMap<&'a str, i32>, instr: &'a Instr) -> bool {
    let &(_, _, _, r, cmp, v) = instr;
    let rv = *regs.get(r).unwrap_or(&0);
    match cmp {
        Cmp::Eq => rv == v,
        Cmp::Ne => rv != v,
        Cmp::Le => rv <= v,
        Cmp::Ge => rv >= v,
        Cmp::Lt => rv < v,
        Cmp::Gt => rv > v
    }
}

fn exec<'a>(regs: &mut HashMap<&'a str, i32>, instr: &'a Instr) {
    let check = check_constraint(regs, instr);
    let (r, op, v, _, _, _) = *instr;
    let rv = regs.entry(r).or_insert(0);
    if check {
        match op {
            Op::Inc => *rv += v,
            Op::Dec => *rv -= v
        }
    }
}

fn main() {
    let instructions = parse_input(INPUT)
        .unwrap().1;
    let mut regs = HashMap::new();
    for instr in instructions.iter() {
        exec(&mut regs, instr);
    }
    let max = regs.iter()
        .max_by_key(|&(_, v)| *v)
        .unwrap();
    println!("{}", max.1)
}

