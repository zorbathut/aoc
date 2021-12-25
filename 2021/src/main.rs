
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

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Empty,
    East,
    South,
}

fn main() {
    let mut map: Vec<Vec<State>> = read_lines().iter().map(|l| {
        l.chars().map(|c| match c {
            '.' => State::Empty,
            '>' => State::East,
            'v' => State::South,
            _ => unreachable!(),
        }).collect()
    }).collect();
    
    let mut ticks = 1;
    loop {
        let mut nmap: Vec<Vec<State>> = map.clone();
        let mut change = false;

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == State::East && map[y][(x + 1) % map[0].len()] == State::Empty {
                    nmap[y][x] = State::Empty;
                    nmap[y][(x + 1) % map[0].len()] = State::East;
                    change = true;
                }
            }
        }
        map = nmap;
        nmap = map.clone();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == State::South && map[(y + 1) % map.len()][x] == State::Empty {
                    nmap[y][x] = State::Empty;
                    nmap[(y + 1) % map.len()][x] = State::South;
                    change = true;
                }
            }
        }
        map = nmap;

        if !change {
            dbg!(ticks);
            break;
        }
        ticks += 1;
    }

}
