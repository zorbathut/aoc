
use std::mem;
use std::io;
use std::cmp;
use std::fmt;
use regex::Regex;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;
use std::iter::FromIterator;
use num::integer;
use itertools::Itertools;
use multiset::HashMultiSet;
use hex;
use pest::Parser;
use pest_derive::Parser;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate scan_fmt;

fn read_numbers() -> Vec<i64> {
    let mut rv: Vec<i64> = Vec::new();

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => return rv,
            Ok(_) => {
                input = input.trim().to_string();
                rv.push(input.parse::<i64>().unwrap());
            },
            Err(_) => return rv,
        }
        input.clear();
    }
}

fn read_lines() -> Vec<String> {
    let mut rv: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => return rv,
            Ok(_) => {
                input = input.trim().to_string();
                rv.push(input);
            },
            Err(_) => return rv,
        }
    }
}

fn read_groups() -> Vec<Vec<String>> {
    let mut rv: Vec<Vec<String>> = Vec::new();
    let mut group = Vec::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                if group.len() > 0 {
                    rv.push(group);
                }
                return rv;
            },
            Ok(_) => {
                input = input.trim().to_string();
                if input.len() == 0 {
                    rv.push(group);
                    group = Vec::new();
                } else {
                    group.push(input);
                }
            },
            Err(_) => {
                panic!();
            }
        }
    }
}

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn read_directions() -> Vec<Direction> {
    let lines = read_lines();

    let mut directions: Vec<Direction> = Vec::new();

    let re = Regex::new(r"(?P<dir>[a-z]+) (?P<arg>[\d]+)").unwrap();

    for line in lines {
        println!("{:#?}", line);

        let captures = re.captures(&line).unwrap();

        let inst = captures.name("dir").unwrap().as_str();
        let arg = captures.name("arg").unwrap().as_str().parse::<i32>().unwrap();
        
        if inst == "forward" {
            directions.push(Direction::Forward(arg));
        } else if inst == "down" {
            directions.push(Direction::Down(arg));
        } else if inst == "up" {
            directions.push(Direction::Up(arg));
        }
    }

    directions
}

trait StdinExt {
    fn read_line_direct(&mut self) -> String;
}

impl StdinExt for io::Stdin {
    fn read_line_direct(&mut self) -> String
    {
        let mut readstr: String = String::new();
        self.read_line(&mut readstr).ok();
        readstr.trim().to_string()
    }
}

#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Hash)]
#[derive(Debug)]
enum Inst {
    Inp(usize),
    AddR(usize, usize),
    MulR(usize, usize),
    DivR(usize, usize),
    ModR(usize, usize),
    EqlR(usize, usize),
    AddL(usize, i64),
    MulL(usize, i64),
    DivL(usize, i64),
    ModL(usize, i64),
    EqlL(usize, i64),
}

#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Hash)]
#[derive(Debug)]
struct State {
    index: usize,
    regs: [i64; 4],
}

fn run(commands: &Vec<Inst>, seen: &mut HashSet<State>, mut state: State, inp: &mut Vec<i64>) {
    if state.index >= commands.len() {
        if state.regs[3] == 0 {
            dbg!(inp);
        }
        
        return;
    }

    if seen.contains(&state) {
        return;
    }
    seen.insert(state);

    let ido = state.index;
    state.index += 1;
    match commands[ido] {
        Inst::Inp(l) => {
            for i in (1..=9) {
                inp.push(i);
                state.regs[l] = i;
                run(commands, seen, state, inp);
                inp.pop();
            }

            return;
        },
        Inst::AddR(l, r) => state.regs[l] += state.regs[r],
        Inst::MulR(l, r) => state.regs[l] *= state.regs[r],
        Inst::DivR(l, r) => state.regs[l] /= state.regs[r],
        Inst::ModR(l, r) => state.regs[l] %= state.regs[r],
        Inst::EqlR(l, r) => state.regs[l] = (if state.regs[l] == state.regs[r] { 1 } else { 0 }),
        Inst::AddL(l, r) => state.regs[l] += r,
        Inst::MulL(l, r) => state.regs[l] *= r,
        Inst::DivL(l, r) => state.regs[l] /= r,
        Inst::ModL(l, r) => state.regs[l] %= r,
        Inst::EqlL(l, r) => state.regs[l] = (if state.regs[l] == r { 1 } else { 0 }),
    }

    run(commands, seen, state, inp);
}

fn main() {
    let commands: Vec<Inst> = read_lines().iter().map(|l| {
        if let Ok((i,a,b)) = scan_fmt!(l, "{} {} {}", String, String, String) {
            let ar = (a.chars().nth(0).unwrap() as i32 - 'w' as i32) as usize;
            if let Ok(l) = b.parse::<i64>() {
                match i.as_str() {
                    "add" => Inst::AddL(ar, l),
                    "mul" => Inst::MulL(ar, l),
                    "div" => Inst::DivL(ar, l),
                    "mod" => Inst::ModL(ar, l),
                    "eql" => Inst::EqlL(ar, l),
                    _ => unreachable!(),
                }
            } else {
                let br = (b.chars().nth(0).unwrap() as i32 - 'w' as i32) as usize;
                match i.as_str() {
                    "add" => Inst::AddR(ar, br),
                    "mul" => Inst::MulR(ar, br),
                    "div" => Inst::DivR(ar, br),
                    "mod" => Inst::ModR(ar, br),
                    "eql" => Inst::EqlR(ar, br),
                    _ => unreachable!(),
                }
            }
        } else if let Ok((i,a)) = scan_fmt!(l, "{} {}", String, String) {
            let ar = (a.chars().nth(0).unwrap() as i32 - 'w' as i32) as usize;
            Inst::Inp(ar)
        } else {
            unreachable!();
        }
    }).collect();

    let mut seen: HashSet<State> = HashSet::new();
    let mut inp: Vec<i64> = Vec::new();
    run(&commands, &mut seen, State { index: 0, regs: [0; 4] }, &mut inp);
}
