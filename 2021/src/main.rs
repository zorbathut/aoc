
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
struct Packet {
    version: u32,
    typ: Typ,
    data: PacketPayload,
}

#[derive(Debug)]
enum PacketPayload {
    Literal(u64),
    Nested(Vec<Packet>),
}

#[derive(Debug)]
enum Typ {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LesserThan,
    EqualTo,
}

fn consume(stream: &mut VecDeque<bool>, bits: u32) -> u32 {
    let mut result = 0u32;
    for i in 0..bits {
        result <<= 1;
        if stream.pop_front().unwrap() {
            result += 1;
        }
    }

    result
}

fn pt(typ: u32) -> Typ {
    match typ {
        0 => Typ::Sum,
        1 => Typ::Product,
        2 => Typ::Minimum,
        3 => Typ::Maximum,
        4 => Typ::Literal,
        5 => Typ::GreaterThan,
        6 => Typ::LesserThan,
        7 => Typ::EqualTo,
        _ => unreachable!(),
    }
}

fn parsesingle(bitstream: &mut VecDeque<bool>) -> Packet {
    let ver = consume(bitstream, 3);
    let typ = consume(bitstream, 3);

    if typ == 4 {
        dbg!("t4");
        let mut val = 0u64;
        loop {
            
            let more = consume(bitstream, 1);
            let chunk = consume(bitstream, 4);
            val = (val << 4) + (chunk as u64);

            dbg!(more);

            if more == 0 {
                break;
            }
        }
        
        dbg!(val);

        Packet {
            version: ver,
            typ: pt(typ),
            data: PacketPayload::Literal(val),
        }
    } else {
        if consume(bitstream, 1) == 0 {
            let bits = consume(bitstream, 15);
            let mut eaten = VecDeque::new();
            for i in 0..bits {
                eaten.push_back(bitstream.pop_front().unwrap());
            }

            Packet {
                version: ver,
                typ: pt(typ),
                data: PacketPayload::Nested(parse(&mut eaten)),
            }
        } else {
            let packets = consume(bitstream, 11);
            let mut pacvec = Vec::new();
            for i in 0..packets {
                pacvec.push(parsesingle(bitstream));
            }

            Packet {
                version: ver,
                typ: pt(typ),
                data: PacketPayload::Nested(pacvec),
            }
        }
    }
}

fn parse(stream: &mut VecDeque<bool>) -> Vec<Packet> {
    let mut rv = Vec::new();
    while stream.len() > 7 {
        dbg!(stream.len());
        rv.push(parsesingle(stream));
        dbg!(stream.len());
    }

    rv
}

impl Packet {
    fn versum(&self) -> u32 {
        self.version + match &self.data {
            PacketPayload::Literal(_) => 0,
            PacketPayload::Nested(packets) => packets.iter().map(|p| p.versum()).sum(),
        }
    }

    fn eval(&self) -> u64 {
        match &self.data {
            PacketPayload::Literal(v) => *v,
            PacketPayload::Nested(packets) => {
                let mut peval = packets.iter().map(|p| p.eval()).collect::<Vec<u64>>();
                let res = match &self.typ {
                    Typ::Sum => peval.iter().sum(),
                    Typ::Product => peval.iter().product(),
                    Typ::Minimum => *peval.iter().min().unwrap(),
                    Typ::Maximum => *peval.iter().max().unwrap(),
                    Typ::Literal => unreachable!(),
                    Typ::GreaterThan => if peval[0] > peval[1] { 1 } else { 0 },
                    Typ::LesserThan => if peval[0] < peval[1] { 1 } else { 0 },
                    Typ::EqualTo => if peval[0] == peval[1] { 1 } else { 0 },
                };
                dbg!(&self.typ, &peval, res);
                res
            }
        }
    }
}

fn main() {
    let mut bytestream = hex::decode(&read_lines()[0]).unwrap();
    let mut bitstream = VecDeque::new();
    for b in bytestream.iter() {
        for i in (0..8).rev() {
            bitstream.push_back((b & (1 << i)) != 0);
        }
    }

    //dbg!(&bytestream);
    //dbg!(&bitstream);

    let mut packets = parse(&mut bitstream);
    dbg!(&packets);

    dbg!(packets[0].eval());
    dbg!(packets.iter().map(|p| p.versum()).sum::<u32>());
    dbg!(packets.len());
}
