
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

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct State {
    p1: i32,
    p2: i32,
    p1s: i32,
    p2s: i32,
    p2t: i32,
}

fn main() {
    let mut states: HashMap<State, i64> = HashMap::new();
    states.insert(State { p1: 2, p2: 8, p1s: 0, p2s: 0, p2t: 0 }, 1);

    for p1s in 0..21 {
        for p2s in 0..21 {
            for p1 in 1..=10 {
                for p2 in 1..=10 {
                    for p2t in 0..2 {
                        let scx = State { p1: p1, p2: p2, p1s: p1s, p2s: p2s, p2t: p2t };
                        if !states.contains_key(&scx) {
                            continue;
                        }

                        let sct = states[&scx];

                        for a in 1..=3 {
                            for b in 1..=3 {
                                for c in 1..=3 {
                                    let mut result = scx.clone();
                                    if p2t == 0 {
                                        result.p1 = (result.p1 + a + b + c - 1) % 10 + 1;
                                        result.p1s += result.p1;
                                        result.p2t = 1;
                                    } else {
                                        result.p2 = (result.p2 + a + b + c - 1) % 10 + 1;
                                        result.p2s += result.p2;
                                        result.p2t = 0;
                                    }
                                    *states.entry(result).or_insert(0) += sct;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let p1w: i64 = states.iter().filter(|s| s.0.p1s >= 21).map(|s| s.1).sum();
    let p2w: i64 = states.iter().filter(|s| s.0.p2s >= 21).map(|s| s.1).sum();

    dbg!(p1w, p2w);
}
