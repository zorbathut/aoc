
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

fn main() {
    let lines = read_lines();

    let mut lookup = lines[0].chars().map(|k| k == '#').enumerate().collect::<HashMap<usize, bool>>();
    
    let mut smap: Vec<Vec<bool>> = lines.iter().skip(2).map(|l| l.chars().map(|c| c == '#').collect()).collect();

    let dx = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
    let dy = [-1, -1, -1, 0, 0, 0, 1, 1, 1];

    for i in 0..50 {
        let mut nmap = Vec::new();
        for y in -1..(smap.len() as i32 + 1) {
            let mut lin = Vec::new();
            for x in -1..(smap[0].len() as i32 + 1) {
                let mut ki: usize = 0;
                for d in 0..9 {
                    let tx = x + dx[d];
                    let ty = y + dy[d];

                    ki *= 2;

                    if tx >= 0 && ty >= 0 && tx < smap[0].len() as i32 && ty < smap.len() as i32 {
                        if smap[ty as usize][tx as usize] {
                            ki += 1;
                        }
                    } else if i % 2 == 1 {
                        ki += 1;
                    }
                }

                lin.push(lookup[&ki]);
            }
            nmap.push(lin);
        }
        smap = nmap;
    }

    dbg!(smap.iter().map(|x| x.iter().map(|c| if *c { 1 } else { 0 }).sum::<i32>()).sum::<i32>());
}
