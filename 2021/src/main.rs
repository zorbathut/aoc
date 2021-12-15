
use std::mem;
use std::io;
use std::cmp;
use std::fmt;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;
use std::iter::FromIterator;
use num::integer;
use itertools::Itertools;
use multiset::HashMultiSet;

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

lazy_static! {
    static ref REGEX_TILE: regex::Regex = Regex::new(r"(e|w|ne|nw|se|sw)").unwrap();
}

fn main() {
    let mut olines = read_lines().iter().map(|l| l.chars().map(|c| (c as i32 - '0' as i32) as u32).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let mut lines: Vec<Vec<u32>> = Vec::new();
    for a in 0..5 {
        for lin in olines.iter() {
            let mut nlin = Vec::new();
            for b in 0..5 {
                for k in lin.iter().map(|v| (v - 1 + a + b) % 9 + 1) {
                    nlin.push(k);
                }
            }
            lines.push(nlin);
        }
    }

    let mut pos: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    pos.push((0, 0, 0));

    let dx = vec![0, 0, 1, -1];
    let dy = vec![1, -1, 0, 0];
    
    loop {
        let nx = pos.pop().unwrap();

        //dbg!(&nx);

        if nx.1 + 1 == lines.len() as i32 && nx.2 + 1 == lines[0].len() as i32 {
            dbg!(nx.0);
            break;
        }

        for t in 0..4 {
            let tx = nx.1 + dx[t];
            let ty = nx.2 + dy[t];

            if tx < 0 || ty < 0 || tx >= lines.len() as i32 || ty >= lines[0].len() as i32 {
                continue;
            }

            if lines[tx as usize][ty as usize] >= 100 {
                continue;
            }

            let tc = nx.0 - lines[tx as usize][ty as usize] as i32;
            lines[tx as usize][ty as usize] = 100;

            pos.push((tc, tx, ty));
        }
    }
}
