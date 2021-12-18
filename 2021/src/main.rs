
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

lazy_static! {
    static ref REGEX_TILE: regex::Regex = Regex::new(r"(e|w|ne|nw|se|sw)").unwrap();
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

#[derive(Debug)]
#[derive(Clone)]
enum Element {
    Open(),
    Value(i64),
    Close(),
}

fn redoos(k: Vec<Element>) -> Vec<Element> {
    //dbg!("redo");
    let mut n = k;
    loop {
        // search for explode
        let mut depth = 0;
        let mut explo = 0;
        for i in 0..(n.len()) {
            match n[i] {
                Element::Open() => depth += 1,
                Element::Close() => depth -= 1,
                Element::Value(_) => if depth >= 5 { explo = i; break; },
            }
        }

        //dbg!(explo);

        if explo != 0 {
            // doing explosion
            let lhs = if let Element::Value(v) = n[explo] { v } else { unreachable!() };
            let rhs = if let Element::Value(v) = n[explo + 1] { v } else { unreachable!() };
            for i in (0..(explo-1)).rev() {
                match n[i] {
                    Element::Value(v) => {
                        n[i] = Element::Value(lhs + v);
                        break;
                    },
                    _ => (),
                }
            }
            for i in (explo+2)..(n.len()) {
                match n[i] {
                    Element::Value(v) => {
                        n[i] = Element::Value(rhs + v);
                        break;
                    },
                    _ => (),
                }
            }

            n[explo - 1] = Element::Value(0);
            n.remove(explo);    // value
            n.remove(explo);    // value
            n.remove(explo);    // close
            continue;
        }

        // search for split
        let mut splat = false;
        for i in 0..(n.len()) {
            match n[i] {
                Element::Open() => (),
                Element::Close() => (),
                Element::Value(v) => if v >= 10 {
                    n[i] = Element::Close();
                    n.insert(i, Element::Value((v + 1) / 2));
                    n.insert(i, Element::Value(v / 2));
                    n.insert(i, Element::Open());
                    splat = true;
                    break;
                },
            }
        }
        if splat {
            continue;
        }

        // done
        break;
    }

    n
}

fn maggy(n: Vec<Element>) -> i64 {
    if n.len() == 1 {
        let lhs = if let Element::Value(v) = n[0] { v } else { unreachable!() };
        return lhs;
    } else if n.len() == 4 {
        let lhs = if let Element::Value(v) = n[1] { v } else { unreachable!() };
        let rhs = if let Element::Value(v) = n[2] { v } else { unreachable!() };

        return lhs * 3 + rhs * 2;
    } else {
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();
        let mut depth = 0;

        //dbg!(&n);

        for i in 0..(n.len()) {
            match n[i] {
                Element::Open() => depth += 1,
                Element::Close() => {
                    depth -= 1;
                    if depth == 1 {
                        lhs = n[1..(i + 1)].to_vec();
                        rhs = n[(i + 1)..(n.len() - 1)].to_vec();
                        break;
                    }
                },
                Element::Value(_) => {
                    if depth == 1 {
                        lhs = n[1..(i + 1)].to_vec();
                        rhs = n[(i + 1)..(n.len() - 1)].to_vec();
                        break;
                    }
                },
            }
        }

        /*dbg!(lhs.len(), rhs.len());
        dbg!(&lhs);
        dbg!(&rhs);*/

        return maggy(lhs) * 3 + maggy(rhs) * 2;
    }
}

fn sum(lhs: &Vec<Element>, rhs: &Vec<Element>) -> Vec<Element> {
    let mut nusu = Vec::new();
    nusu.push(Element::Open());
    nusu.extend(lhs.clone());
    nusu.extend(rhs.clone());
    nusu.push(Element::Close());

    redoos(nusu)
}

fn main() {
    let lines = read_lines();

    let mut plin: Vec<Vec<Element>> = Vec::new();
    for lin in lines {
        let parse_result = MyParser::parse(Rule::element, &lin).unwrap().next().unwrap().into_inner();
        plin.push(parse_result.filter(|e| e.as_rule() != Rule::comma).map(|e| {
            match e.as_rule() {
                Rule::open => Element::Open(),
                Rule::close => Element::Close(),
                Rule::number => Element::Value(str::parse(e.as_str()).unwrap()),
                _ => unreachable!(),
            }
        }).collect());
    }

    dbg!(&plin);

    let mut best = 0;
    for x in 0..plin.len() {
        for y in 0..plin.len() {
            let amag = maggy(sum(&plin[x], &plin[y]));
            let bmag = maggy(sum(&plin[y], &plin[x]));

            best = cmp::max(best, amag);
            best = cmp::max(best, bmag);
        }
    }

    dbg!(best);
}
