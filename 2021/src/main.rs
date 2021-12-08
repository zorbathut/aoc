
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
    let digis = &["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"].iter().map(|x| x.as_bytes().iter().map(|b| b - ('a' as u8)).collect()).collect::<Vec<Vec<u8>>>();
    //dbg!(digis);

    let mut acu = 0;
    for line in read_lines() {
        let refs: Vec<Vec<u8>> = line.split('|').nth(0).unwrap().trim().split(' ').map(|s| s.as_bytes().iter().map(|b| b - ('a' as u8)).collect()).collect();
        let keys: Vec<Vec<u8>> = line.split('|').nth(1).unwrap().trim().split(' ').map(|s| s.as_bytes().iter().map(|b| b - ('a' as u8)).collect()).collect();

        //let perm = vec![2u8, 5u8, 6u8, 0u8, 1u8, 3u8, 4u8];

        for perm in (0..7).permutations(7) {
            let mut working = true;
            for it in &refs {
                if !digis.contains(&tweak(&perm, &it)) {
                    //dbg!(it, tweak(&perm, &it));
                    working = false;
                    break;
                }
            }

            if working {
                dbg!(&perm);
                let mut tacu = 0;
                for it in keys {
                    let mat = tweak(&perm, &it);
                    tacu *= 10;
                    tacu += digis.iter().position(|r| *r == mat).unwrap();
                }
                acu += tacu;
                break;
            }
        }
    }

    dbg!(acu);
}
