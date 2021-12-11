
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
    let mut octomap: Vec::<Vec<i8>> = vec![];

    let dx: Vec::<i8> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let dy: Vec::<i8> = vec![-1, 0, 1, -1, 1, -1, 0, 1];

    for line in read_lines().iter() {
        let mut lin: Vec::<i8> = vec![];
        for list in line.chars() {
            lin.push((list as i32 - '0' as i32) as i8);
        }
        octomap.push(lin);
    }

    
    for i in 0..100000 {
        dbg!(i);

        let mut flash = 0;

        for x in 0..10 {
            for y in 0..10 {
                octomap[x][y] += 1;
            }
        }

        let mut exhausted = [[false; 10]; 10];

        loop {
            let mut done = true;
            
            for x in 0..10 {
                for y in 0..10 {
                    if octomap[x][y] >= 10 && !exhausted[x][y] {
                        done = false;
                        exhausted[x][y] = true;
                        flash += 1;
                        for d in 0..8 {
                            let tx = (x as i8) + dx[d];
                            let ty = (y as i8) + dy[d];
                            if tx < 0 || tx >= 10 || ty < 0 || ty >= 10 {
                                continue
                            }

                            octomap[tx as usize][ty as usize] += 1;
                        }
                    }
                }
            }

            if done {
                break;
            }
        }

        for x in 0..10 {
            for y in 0..10 {
                if octomap[x][y] >= 10 {
                    octomap[x][y] = 0;
                }
            }
        }

        if flash == 100 {
            dbg!(i);
            break;
        }
    }
}
