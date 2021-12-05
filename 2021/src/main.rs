
use std::mem;
use std::io;
use std::cmp;
use std::fmt;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::FromIterator;
use num::integer;
use itertools::Itertools;

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

#[derive(Debug)]
struct Line
{
    sx: usize,
    sy: usize,
    ex: usize,
    ey: usize,
}

fn main() {
    let re = Regex::new(r"(?P<sx>[0-9]+),(?P<sy>[0-9]+) \-> (?P<ex>[0-9]+),(?P<ey>[0-9]+)").unwrap();

    let mut lines = Vec::new();

    for line in read_lines() {
        println!("{:#?}", line);

        let captures = re.captures(&line).unwrap();
        lines.push(Line {
            sx: captures.name("sx").unwrap().as_str().parse::<usize>().unwrap(),
            ex: captures.name("ex").unwrap().as_str().parse::<usize>().unwrap(),
            sy: captures.name("sy").unwrap().as_str().parse::<usize>().unwrap(),
            ey: captures.name("ey").unwrap().as_str().parse::<usize>().unwrap(),
        })
    }

    let mut state = vec![[0u16; 1000]; 1000];
    for line in lines.iter() {
        if line.sx == line.ex {
            for y in cmp::min(line.sy, line.ey)..=cmp::max(line.sy, line.ey) {
                state[line.sx][y] = state[line.sx][y] + 1;
            }
        } else if line.sy == line.ey {
            for x in cmp::min(line.sx, line.ex)..=cmp::max(line.sx, line.ex) {
                state[x][line.sy] = state[x][line.sy] + 1;
            }
        }
    }

    //dbg!(&state);

    dbg!(state.iter().map(|l| l.iter().filter(|c| **c >= 2).count()).sum::<usize>());
}
