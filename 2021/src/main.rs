
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

fn tweak(permutation: &Vec<u8>, val: &Vec<u8>) -> Vec<u8>
{
    val.iter().map(|&c| permutation[c as usize]).sorted().collect()
}

fn main() {
    let mut lopo = read_lines().iter().map(|l| l.as_bytes().iter().map(|b| (*b as i8) - ('0' as i8)).collect()).collect::<Vec<Vec<i8>>>();
    
    let mx = lopo.len() as i32;
    let my = lopo[0].len() as i32;

    let dx = vec![0, 0, 1, -1];
    let dy = vec![1, -1, 0, 0];

    let mut acu = 0i32;

    let mut basins = Vec::new();

    for x in 0..mx {
        for y in 0..my {
            if lopo[x as usize][y as usize] == 9 {
                continue;
            }

            let mut size = 0;
            let mut points = Vec::new();
            points.push((x, y));

            while points.len() > 0 {
                let pt = points.pop().unwrap();
                if pt.0 < 0 || pt.0 >= mx || pt.1 < 0 || pt.1 >= my {
                    continue;
                }
                if lopo[pt.0 as usize][pt.1 as usize] == 9 {
                    continue;
                }

                lopo[pt.0 as usize][pt.1 as usize] = 9;
                size += 1;

                for k in 0..4 {
                    points.push((pt.0 + dx[k], pt.1 + dy[k]));
                }
            }

            basins.push(size);
        }
    }

    basins.sort();
    dbg!(&basins);

    dbg!(basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap());
}
