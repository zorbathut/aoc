
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

fn main() {
    let dat = read_lines();
    let balls: Vec::<i32> = dat[0].split(",").map(|b| b.parse::<i32>().unwrap()).collect();

    let mut boards = Vec::<Vec::<Vec::<i32>>>::new();
    for line in (2..dat.len()).step_by(6) {
        let mut board = Vec::<Vec::<i32>>::new();
        for y in 0..5 {
            board.push(dat[line + y].split(" ").filter(|t| t.len() > 0).map(|b| b.parse::<i32>().unwrap()).collect());
        }

        dbg!(&board);
        boards.push(board);
    }

    let mut soonest = 1000;
    let mut score = 0;
    for board in boards.iter() {
        let mut bm = board.clone();

        for (idx, ball) in balls.iter().enumerate() {
            for line in bm.iter_mut() {
                for item in line.iter_mut() {
                    if *item == *ball {
                        *item = -1;
                    }
                }
            }

            for len in 0..5 {
                let mut xwon = true;
                let mut ywon = true;
                for path in 0..5 {
                    if bm[len][path] != -1 {
                        xwon = false;
                    }
                    if bm[path][len] != -1 {
                        ywon = false;
                    }
                }
                
                if xwon || ywon {
                    if idx < soonest {
                        dbg!("WIN");
                        dbg!(&bm);

                        soonest = idx;
                        score = bm.iter().map(|line| line.iter().map(|&c| if c == -1 { 0 } else { c }).sum::<i32>()).sum::<i32>() * ball;
                    }
                }
            }
        }
    }

    dbg!(score);
}
