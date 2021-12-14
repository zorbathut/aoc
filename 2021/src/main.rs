
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

#[derive(Debug)]
struct Line
{
    sx: usize,
    sy: usize,
    ex: usize,
    ey: usize,
}

fn main() {
    let lines = read_lines();

    let mut rules: HashMap<[char; 2], char> = HashMap::new();

    for line in lines.iter().skip(1) {
        dbg!(line);
        if let Ok((a, b)) = scan_fmt!(&line, "{} -> {}", String, char) {
            rules.insert([a.chars().nth(0).unwrap(), a.chars().nth(1).unwrap()], b);
        }
    }
    
    let mut pory: HashMap<[char; 2], i64> = HashMap::new();

    let poly = &lines[0];
    for i in 0..(poly.len() - 1) {
        *pory.entry([poly.chars().nth(i).unwrap(), poly.chars().nth(i + 1).unwrap()]).or_insert(0) += 1;
    }

    dbg!(&rules);
    dbg!(&pory);
    
    for q in 0..40 {
        let mut nexto = HashMap::new();

        for item in pory {
            *nexto.entry([item.0[0], rules[&item.0]]).or_insert(0) += item.1;
            *nexto.entry([rules[&item.0], item.0[1]]).or_insert(0) += item.1;
        }

        dbg!(&nexto);
        pory = nexto;
    }

    let mut count: HashMap<char, i64> = HashMap::new();
    for c in pory.iter() {
        *count.entry(c.0[0]).or_insert(0) += c.1;
        *count.entry(c.0[1]).or_insert(0) += c.1;
    }

    *count.entry(poly.chars().nth(0).unwrap()).or_insert(0) += 1;
    *count.entry(poly.chars().nth(poly.len() - 1).unwrap()).or_insert(0) += 1;

    dbg!(count.values().min().unwrap() / 2);
    dbg!(count.values().max().unwrap() / 2);

    dbg!((count.values().max().unwrap() - count.values().min().unwrap()) / 2);
}
