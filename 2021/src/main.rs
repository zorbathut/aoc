
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

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Boxo {
    sx: i64,
    ex: i64,
    sy: i64,
    ey: i64,
    sz: i64,
    ez: i64,
}

fn maybeadd(nubox: &mut Vec<Boxo>, bok: Boxo) {
    if bok.sx < bok.ex && bok.sy < bok.ey && bok.sz < bok.ez {
        nubox.push(bok);
    }
}

fn main() {
    let mut boxiz: Vec<Boxo> = Vec::new();
    
    for line in read_lines() {
        if let Ok((mode, sx, mut ex, sy, mut ey, sz, mut ez)) = scan_fmt!(&line, "{} x={}..{},y={}..{},z={}..{}", String, i64, i64, i64, i64, i64, i64) { // types
            ex += 1;
            ey += 1;
            ez += 1;

            // remove
            let mut nubox = Vec::new();
            for boxx in boxiz {
                //dbg!(boxx, sx, ex, sy, ey, sz, ez);
                if sx >= boxx.ex || sy >= boxx.ey || sz >= boxx.ez || ex <= boxx.sx || ey <= boxx.sy || ez <= boxx.sz {
                    //dbg!("fakery");
                    nubox.push(boxx);
                } else {
                    //dbg!("trubox");
                    maybeadd(&mut nubox, Boxo { sx: boxx.sx, ex: sx, sy: boxx.sy, ey: boxx.ey, sz: boxx.sz, ez: boxx.ez });
                    maybeadd(&mut nubox, Boxo { sx: ex, ex: boxx.ex, sy: boxx.sy, ey: boxx.ey, sz: boxx.sz, ez: boxx.ez });

                    maybeadd(&mut nubox, Boxo { sx: cmp::max(sx, boxx.sx), ex: cmp::min(ex, boxx.ex), sy: boxx.sy, ey: sy, sz: boxx.sz, ez: boxx.ez });
                    maybeadd(&mut nubox, Boxo { sx: cmp::max(sx, boxx.sx), ex: cmp::min(ex, boxx.ex), sy: ey, ey: boxx.ey, sz: boxx.sz, ez: boxx.ez });

                    maybeadd(&mut nubox, Boxo { sx: cmp::max(sx, boxx.sx), ex: cmp::min(ex, boxx.ex), sy: cmp::max(sy, boxx.sy), ey: cmp::min(ey, boxx.ey), sz: boxx.sz, ez: sz });
                    maybeadd(&mut nubox, Boxo { sx: cmp::max(sx, boxx.sx), ex: cmp::min(ex, boxx.ex), sy: cmp::max(sy, boxx.sy), ey: cmp::min(ey, boxx.ey), sz: ez, ez: boxx.ez });
                }
            }

            if mode == "on" {
                nubox.push(Boxo { sx: sx, ex: ex, sy: sy, ey: ey, sz: sz, ez: ez });
            }

            boxiz = nubox;

            //dbg!(&boxiz);
            dbg!(boxiz.iter().map(|b| (b.ex - b.sx) * (b.ey - b.sy) * (b.ez - b.sz)).sum::<i64>());
        } else {
            unreachable!();
        }
    }
}
