#[macro_use]
extern crate nom;

use nom::digit;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input/day18.txt");

// const INPUT: &str = "snd 1
// snd 2
// snd p
// rcv a
// rcv b
// rcv c
// rcv d
// ";

type Reg = char;

#[derive(Debug)]
enum Val {
    Register(Reg),
    Immediate(i64)
}

#[derive(Debug)]
enum Instr {
    Snd(Val),
    Set(Reg, Val),
    Add(Reg, Val),
    Mul(Reg, Val),
    Mod(Reg, Val),
    Rcv(Reg),
    Jgz(Val, Val),
}

named!(parse_input<&str, Vec<Instr>>, many1!(do_parse!(i: instruction >> tag_s!("\n") >> (i))));

named!(instruction<&str, Instr>,
       alt!(do_parse!(tag_s!("snd ") >>
                      v: value >>
                      (Instr::Snd(v))) |
            do_parse!(tag_s!("set ") >>
                      r: register >>
                      tag_s!(" ") >>
                      v: value >>
                      (Instr::Set(r, v))) |
            do_parse!(tag_s!("add ") >>
                      r: register >>
                      tag_s!(" ") >>
                      v: value >>
                      (Instr::Add(r, v))) |
            do_parse!(tag_s!("mul ") >>
                      r: register >>
                      tag_s!(" ") >>
                      v: value >>
                      (Instr::Mul(r, v))) |
            do_parse!(tag_s!("mod ") >>
                      r: register >>
                      tag_s!(" ") >>
                      v: value >>
                      (Instr::Mod(r, v))) |
            do_parse!(tag_s!("rcv ") >>
                      r: register >>
                      (Instr::Rcv(r))) |
            do_parse!(tag_s!("jgz ") >>
                      v1:value >>
                      tag_s!(" ")  >>
                      v2: value >>
                      (Instr::Jgz(v1, v2)))));

named!(value<&str, Val>,
       alt!(map!(immediate, Val::Immediate) |
            map!(register, Val::Register)));

named!(register<&str, Reg>,
       map!(take_s!(1), |s| s.chars().next().unwrap()));

named!(immediate<&str, i64>,
       do_parse!(s: opt!(tag_s!("-")) >> d: digit >>
                 (d.parse::<i64>().unwrap() * s.map(|_| -1).unwrap_or(1))));

impl Val {
    fn get(&self, registers: &HashMap<Reg, i64>) -> i64 {
        match *self {
            Val::Immediate(v) => v,
            Val::Register(r) => registers.get(&r)
                .unwrap_or(&0)
                .to_owned()
        }
    }
}

struct Interpreter<'a, S, R>
    where S: FnMut(i64), R: FnMut() -> Option<i64> {
    pc: usize,
    registers: HashMap<Reg, i64>,
    program: &'a [Instr],
    snd_hook: S,
    rcv_hook: R
}

impl<'a, S, R> Interpreter<'a, S, R>
    where S: FnMut(i64), R: FnMut() -> Option<i64> {

    fn new(program: &'a [Instr], snd_hook: S, rcv_hook: R) -> Interpreter<'a, S, R>
        where S: FnMut(i64), R: FnMut() -> Option<i64> {
        Interpreter { pc: 0, registers: HashMap::new(), program, snd_hook, rcv_hook }
    }

    fn step(&mut self) -> bool {
        let instr = &self.program[self.pc];
        let mut next = self.pc as i64 + 1;
        match *instr {
            Instr::Snd(ref v) => {
                let v = v.get(&self.registers);
                (self.snd_hook)(v);
            },
            Instr::Set(ref r, ref v) => {
                let v = v.get(&self.registers);
                self.registers.insert(*r, v);
            },
            Instr::Add(ref r, ref v) => {
                let v = v.get(&self.registers);
                let r = self.registers.entry(*r).or_insert(0);
                *r += v;
            },
            Instr::Mul(ref r, ref v) => {
                let v = v.get(&self.registers);
                let r = self.registers.entry(*r).or_insert(0);
                *r *= v;
            },
            Instr::Mod(ref r, ref v) => {
                let v = v.get(&self.registers);
                let r = self.registers.entry(*r).or_insert(0);
                *r %= v;
            },
            Instr::Rcv(ref r) => {
                let v = (self.rcv_hook)();
                if let Some(v) = v {
                    let r = self.registers.entry(*r).or_insert(0);
                    *r = v;
                } else {
                    return true;
                }
            },
            Instr::Jgz(ref v1, ref v2) => {
                let v1 = v1.get(&self.registers);
                let v2 = v2.get(&self.registers);
                if v1 > 0 {
                    next = self.pc as i64 + v2 as i64;
                }
            }
        };
        if next >= 0 && next < self.program.len() as i64 {
            self.pc = next as usize;
            true
        } else {
            false
        }
    }
}

fn main() {
    let program = parse_input(INPUT).unwrap().1;

    let mut last = None;
    let stop = RefCell::new(false);
    {
        let mut interpreter = Interpreter
            ::new(&program,
                  |i| { last = Some(i); },
                  || { *stop.borrow_mut() = true; None });

        loop {
            let stop = { *stop.borrow() };
            if stop || !interpreter.step() {
                break;
            }
        }
    }

    println!("{}", last.unwrap());

    let sent = RefCell::new(0);
    let queue_a = RefCell::new(VecDeque::new());
    let queue_b = RefCell::new(VecDeque::new());
    let waiting_a = RefCell::new(false);
    let waiting_b = RefCell::new(false);
    let mut running_a = true;
    let mut running_b = true;

    fn receive(source: &RefCell<VecDeque<i64>>, wait: &RefCell<bool>) -> Option<i64> {
        if let Some(v) = source.borrow_mut().pop_front() {
            *wait.borrow_mut() = false;
            Some(v)
        } else {
            *wait.borrow_mut() = true;
            None
        }
    }

    let mut interp_a = Interpreter
        ::new(&program,
              |i| queue_a.borrow_mut().push_back(i),
              || receive(&queue_b, &waiting_a));
    interp_a.registers.insert('p', 0);

    let mut interp_b = Interpreter
        ::new(&program,
              |i| {
                  *sent.borrow_mut() += 1;
                  queue_b.borrow_mut().push_back(i);
              },
              || receive(&queue_a, &waiting_b));
    interp_b.registers.insert('p', 1);

    loop {
        if running_a {
            running_a = interp_a.step();
        }
        if running_b {
            running_b = interp_b.step();
        }
        if (*waiting_a.borrow() && *waiting_b.borrow()) || !running_a || !running_b {
            break;
        }
    }

    println!("{}", *sent.borrow());
}


