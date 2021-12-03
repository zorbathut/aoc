
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

    let mut gamma = 0;
    let mut epsilon = 0;

    let mut oxy = HashSet::<String>::from_iter(dat.iter().cloned());
    let mut co2 = HashSet::<String>::from_iter(dat.iter().cloned());

    for dig in 0..dat[0].len() {
        let mut ones = dat.iter().map(|x| x.chars().nth(dig).unwrap()).filter(|x| *x == '1').count();

        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if ones * 2 >= dat.len() {
            gamma = gamma + 1;
        } else {
            epsilon = epsilon + 1;
        }

        if oxy.len() > 1 {
            let mut oxones = oxy.iter().map(|x| x.chars().nth(dig).unwrap()).filter(|x| *x == '1').count();
            dbg!(oxones);

            if (oxones * 2 >= oxy.len())
            {
                oxy = HashSet::from_iter(oxy.iter().filter(|x| x.chars().nth(dig).unwrap() == '1').cloned());
            }
            else
            {
                oxy = HashSet::from_iter(oxy.iter().filter(|x| x.chars().nth(dig).unwrap() == '0').cloned());
            }

            dbg!(oxy.len());
        }
        if co2.len() > 1 {
            let mut coones = co2.iter().map(|x| x.chars().nth(dig).unwrap()).filter(|x| *x == '1').count();
            dbg!(coones);

            if (coones * 2 >= co2.len())
            {
                co2 = HashSet::from_iter(co2.iter().filter(|x| x.chars().nth(dig).unwrap() == '0').cloned());
            }
            else
            {
                co2 = HashSet::from_iter(co2.iter().filter(|x| x.chars().nth(dig).unwrap() == '1').cloned());
            }

            dbg!(co2.len());
        }
    }
    
    dbg!(gamma);
    dbg!(epsilon);

    dbg!(gamma * epsilon);

    dbg!(&oxy);
    dbg!(&co2);

    dbg!(i32::from_str_radix(oxy.iter().next().unwrap(), 2));
    dbg!(i32::from_str_radix(co2.iter().next().unwrap(), 2));

    dbg!(i32::from_str_radix(oxy.iter().next().unwrap(), 2).unwrap() * i32::from_str_radix(co2.iter().next().unwrap(), 2).unwrap());
}
