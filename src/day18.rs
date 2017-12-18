#[macro_use]
extern crate nom;

use nom::digit;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input/day18.txt");

type Register = char;

#[derive(Debug)]
enum Value {
    Register(Register),
    Immediate(i64)
}

#[derive(Debug)]
enum Instruction {
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Register),
    Jgz(Value, Value),
}

named!(parse_input<&str, Vec<Instruction>>,
       many1!(do_parse!(i: instruction >> tag_s!("\n") >> (i))));

named!(instruction<&str, Instruction>,
       alt!(do_parse!(tag_s!("snd ") >> v: value >>
                      (Instruction::Snd(v))) |
            do_parse!(tag_s!("set ") >> r: register >> tag_s!(" ") >> v: value >>
                      (Instruction::Set(r, v))) |
            do_parse!(tag_s!("add ") >> r: register >> tag_s!(" ") >> v: value >>
                      (Instruction::Add(r, v))) |
            do_parse!(tag_s!("mul ") >> r: register >> tag_s!(" ") >> v: value >>
                      (Instruction::Mul(r, v))) |
            do_parse!(tag_s!("mod ") >> r: register >> tag_s!(" ") >> v: value >>
                      (Instruction::Mod(r, v))) |
            do_parse!(tag_s!("rcv ") >> r: register >>
                      (Instruction::Rcv(r))) |
            do_parse!(tag_s!("jgz ") >> v1:value >> tag_s!(" ")  >> v2: value >>
                      (Instruction::Jgz(v1, v2)))));

named!(value<&str, Value>,
       alt!(map!(immediate, Value::Immediate) |
            map!(register, Value::Register)));

named!(register<&str, Register>,
       map!(take_s!(1), |s| s.chars().next().unwrap()));

named!(immediate<&str, i64>,
       do_parse!(s: opt!(tag_s!("-")) >> d: digit >>
                 (d.parse::<i64>().unwrap() * s.map(|_| -1).unwrap_or(1))));

impl Value {
    fn get(&self, registers: &HashMap<Register, i64>) -> i64 {
        match *self {
            Value::Immediate(v) => v,
            Value::Register(r) => registers.get(&r)
                .unwrap_or(&0)
                .to_owned()
        }
    }
}

struct Interpreter<'a, S, R>
    where S: FnMut(i64), R: FnMut() -> Option<i64> {
    halted: bool,
    pc: usize,
    registers: HashMap<Register, i64>,
    program: &'a [Instruction],
    snd_hook: S,
    rcv_hook: R
}

impl<'a, S, R> Interpreter<'a, S, R>
    where S: FnMut(i64), R: FnMut() -> Option<i64> {

    fn new(program: &'a [Instruction], snd_hook: S, rcv_hook: R) -> Interpreter<'a, S, R>
        where S: FnMut(i64), R: FnMut() -> Option<i64> {
        Interpreter {
            halted: false,
            pc: 0,
            registers: HashMap::new(),
            program,
            snd_hook,
            rcv_hook
        }
    }

    fn step(&mut self) {
        if self.halted {
            return;
        }

        let instr = &self.program[self.pc];
        let mut next = self.pc as i64 + 1;
        match *instr {
            Instruction::Snd(ref v) => {
                let v = v.get(&self.registers);
                (self.snd_hook)(v);
            },
            Instruction::Set(ref r, ref v) => {
                let v = v.get(&self.registers);
                self.registers.insert(*r, v);
            },
            Instruction::Add(ref r, ref v) => {
                let v = v.get(&self.registers);
                let r = self.registers.entry(*r).or_insert(0);
                *r += v;
            },
            Instruction::Mul(ref r, ref v) => {
                let v = v.get(&self.registers);
                let r = self.registers.entry(*r).or_insert(0);
                *r *= v;
            },
            Instruction::Mod(ref r, ref v) => {
                let v = v.get(&self.registers);
                let r = self.registers.entry(*r).or_insert(0);
                *r %= v;
            },
            Instruction::Rcv(ref r) => {
                let v = (self.rcv_hook)();
                if let Some(v) = v {
                    let r = self.registers.entry(*r).or_insert(0);
                    *r = v;
                } else {
                    return;
                }
            },
            Instruction::Jgz(ref v1, ref v2) => {
                let v1 = v1.get(&self.registers);
                let v2 = v2.get(&self.registers);
                if v1 > 0 {
                    next = self.pc as i64 + v2 as i64;
                }
            }
        };
        if next >= 0 && next < self.program.len() as i64 {
            self.pc = next as usize;
        } else {
            self.halted = true;
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
            interpreter.step();
            if stop || interpreter.halted {
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
        interp_a.step();
        interp_b.step();
        if (*waiting_a.borrow() && *waiting_b.borrow()) || (interp_a.halted && interp_b.halted) {
            break;
        }
    }

    println!("{}", *sent.borrow());
}


