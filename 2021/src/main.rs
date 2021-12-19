
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

fn transform(pt: (i32, i32, i32), rot: i32, trans: (i32, i32, i32)) -> (i32, i32, i32) {
    let xflip = ((rot / 1) % 2) == 1;
    let yflip = ((rot / 2) % 2) == 1;
    let zflip = ((rot / 4) % 2) == 1;
    let permu = rot / 8;

    let mut tf = pt;
    match permu {
        0 => tf = (tf.0, tf.1, tf.2),
        1 => tf = (tf.0, tf.2, tf.1),
        2 => tf = (tf.1, tf.0, tf.2),
        3 => tf = (tf.1, tf.2, tf.0),
        4 => tf = (tf.2, tf.0, tf.1),
        5 => tf = (tf.2, tf.1, tf.0),
        _ => unreachable!(),
    }
    if xflip { tf.0 = -tf.0; }
    if yflip { tf.1 = -tf.1; }
    if zflip { tf.2 = -tf.2; }

    (tf.0 + trans.0, tf.1 + trans.1, tf.2 + trans.2)
}

fn main() {
    let lines = read_lines();

    let mut skanners = Vec::new();
    {
        let mut skaen = Vec::new();
        for lin in lines {
            if lin == "" {
            } else if lin.chars().nth(1).unwrap() == '-' {
                if skaen.len() > 0 {
                    skanners.push(skaen);
                    skaen = Vec::new();
                }
            } else if let Ok((x, y, z)) = scan_fmt!(&lin, "{},{},{}", i32, i32, i32) {
                skaen.push((x, y, z));
            } else {
                unreachable!();
            }
        }

        if skaen.len() > 0 {
            skanners.push(skaen);
        }
    }

    dbg!(skanners.len());

    let mut pointos: HashSet<(i32, i32, i32)> = HashSet::new();
    for pos in &skanners[0] {
        pointos.insert(*pos);
    }

    let mut handled: HashSet<usize> = HashSet::new();
    handled.insert(0);

    let mut scanos: HashSet<(i32, i32, i32)> = HashSet::new();

    while handled.len() < skanners.len() {
        for dst in 0..skanners.len() {
            if handled.contains(&dst) {
                continue;
            }

            let mut inserter = HashSet::new();
            
            dbg!(dst);
            let rhs = &skanners[dst];
            for rot in 0..48 {
                for srcpivot in &pointos {
                    for dstpivot in rhs {
                        let mut trans = transform(*dstpivot, rot, (0, 0, 0));
                        trans.0 = srcpivot.0 - trans.0;
                        trans.1 = srcpivot.1 - trans.1;
                        trans.2 = srcpivot.2 - trans.2;

                        let mut matches = 0;
                        for target in rhs {
                            let rs = transform(*target, rot, trans);
                            if pointos.contains(&rs) {
                                matches += 1;
                            }
                        }
                        
                        if matches >= 6 {
                            dbg!("----------------------", "good!", matches, dst, transform((0, 0, 0), rot, trans), rot, srcpivot, dstpivot, trans);
                            handled.insert(dst);
                            scanos.insert(transform((0, 0, 0), rot, trans));
                            for target in rhs {
                                inserter.insert(transform(*target, rot, trans));
                            }
                            break;
                        }
                    }
                    if inserter.len() > 0 {
                        break;
                    }
                }
                if inserter.len() > 0 {
                    break;
                }
            }

            for elem in inserter {
                pointos.insert(elem);
            }
        }
    }

    dbg!(pointos.len());
    
    let mut best = 0;
    for lhs in &scanos {
        for rhs in &scanos {
            best = cmp::max(best, (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs() + (lhs.2 - rhs.2).abs())
        }
    }
    dbg!(best);
}
