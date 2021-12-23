
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
enum Unit {
    Empty,
    A,
    B,
    C,
    D,
}

impl Unit {
    fn cost(self) -> i32
    {
        match self {
            Unit::A => 1,
            Unit::B => 10,
            Unit::C => 100,
            Unit::D => 1000,
            _ => unreachable!(),
        }
    }
    fn bx(self) -> usize
    {
        match self {
            Unit::A => 0,
            Unit::B => 1,
            Unit::C => 2,
            Unit::D => 3,
            _ => unreachable!(),
        }
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
struct State {
    cost: i32,

    hallway: [Unit; 7],
    boxes: [Unit; 16],
}

fn main() {
    let mut positions: BinaryHeap<State> = BinaryHeap::new();
    positions.push(State {
        cost: 0,
        hallway: [Unit::Empty, Unit::Empty, Unit::Empty, Unit::Empty, Unit::Empty, Unit::Empty, Unit::Empty],
        boxes: [Unit::B, Unit::D, Unit::D, Unit::D, Unit::B, Unit::C, Unit::B, Unit::C, Unit::D, Unit::B, Unit::A, Unit::A, Unit::A, Unit::A, Unit::C, Unit::C],
        //boxes: [Unit::B, Unit::A, Unit::C, Unit::D, Unit::B, Unit::C, Unit::D, Unit::A],
    });

    let mut seen: HashMap<State, i32> = HashMap::new();

    let halltoslot = [0, 1, 3, 5, 7, 9, 10];
    let boxtoslot = [2, 4, 6, 8];

    let slottohall = [0, 1, 100, 2, 100, 3, 100, 4, 100, 5, 6];

    let mut ct = 0;
    loop {
        ct += 1;
        if ct % 1000 == 0 {
            dbg!(ct);
        }

        let it = positions.pop().unwrap();

        // test
        if it.boxes == [Unit::A, Unit::A, Unit::A, Unit::A, Unit::B, Unit::B, Unit::B, Unit::B, Unit::C, Unit::C, Unit::C, Unit::C, Unit::D, Unit::D, Unit::D, Unit::D] {
            dbg!(it.cost);
            break;
        }

        // box to hallway
        for srcbox in 0..16 {
            if it.boxes[srcbox] == Unit::Empty {
                continue;
            }

            if srcbox % 4 != 0 && it.boxes[srcbox - 1] != Unit::Empty {
                continue;
            }

            for dsthall in 0..7 {
                let mut srcs = boxtoslot[srcbox / 4];
                let mut dsts = halltoslot[dsthall];

                if srcs > dsts {
                    mem::swap(&mut srcs, &mut dsts);
                }

                let mut valid = true;
                for slot in srcs..=dsts {
                    if slottohall[slot] == 100 {
                        continue;
                    }

                    if it.hallway[slottohall[slot]] != Unit::Empty {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let moves = dsts - srcs + 1 + srcbox % 4;
                    let cost = (moves as i32) * it.boxes[srcbox].cost();

                    let mut nit = it.clone();
                    mem::swap(&mut nit.boxes[srcbox], &mut nit.hallway[dsthall]);
                    nit.cost -= cost;

                    let mut zit = nit.clone();
                    zit.cost = 0;
                    if !seen.contains_key(&zit) || seen[&zit] < nit.cost {
                        positions.push(nit);
                        seen.insert(zit, nit.cost);
                        //dbg!("------------", it, srcbox, dsthall, nit);
                    }
                }
            }
        }

        // hallway to box
        for dstbox in 0..16 {
            if it.boxes[dstbox] != Unit::Empty {
                continue;
            }

            if dstbox % 4 != 3 && it.boxes[dstbox + 1] == Unit::Empty {
                continue;
            }

            if dstbox % 4 != 3 && it.boxes[dstbox + 1].bx() != dstbox / 4 {
                continue;
            }

            for srchall in 0..7 {
                if it.hallway[srchall] == Unit::Empty {
                    continue;
                }

                if dstbox / 4 != it.hallway[srchall].bx() {
                    continue;
                }
                
                let mut srcs = boxtoslot[dstbox / 4];
                let mut dsts = halltoslot[srchall];

                if srcs > dsts {
                    mem::swap(&mut srcs, &mut dsts);
                }

                let mut valid = true;
                for slot in srcs..=dsts {
                    if slottohall[slot] == 100 {
                        continue;
                    }

                    if slot == halltoslot[srchall] {
                        continue;
                    }

                    if it.hallway[slottohall[slot]] != Unit::Empty {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let moves = dsts - srcs + 1 + dstbox % 4;
                    let cost = (moves as i32) * it.hallway[srchall].cost();

                    let mut nit = it.clone();
                    mem::swap(&mut nit.hallway[srchall], &mut nit.boxes[dstbox]);
                    nit.cost -= cost;

                    let mut zit = nit.clone();
                    zit.cost = 0;
                    if !seen.contains_key(&zit) || seen[&zit] < nit.cost {
                        positions.push(nit);
                        seen.insert(zit, nit.cost);
                        //dbg!("------------", it, srchall, dstbox, nit);
                    }
                }
            }
        }
    }
}
