
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

#[derive(Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn read_program() -> Vec<Instruction> {
    let lines = read_lines();

    let mut instructions: Vec<Instruction> = Vec::new();

    let re = Regex::new(r"(?P<inst>[a-z]+) (?P<arg>[+-][\d]+)").unwrap();

    for line in lines {
        println!("{:#?}", line);

        let captures = re.captures(&line).unwrap();

        let inst = captures.name("inst").unwrap().as_str();
        let arg = captures.name("arg").unwrap().as_str().parse::<i32>().unwrap();
        
        if inst == "acc" {
            instructions.push(Instruction::Acc(arg));
        } else if inst == "jmp" {
            instructions.push(Instruction::Jmp(arg));
        } else if inst == "nop" {
            instructions.push(Instruction::Nop(arg));
        }
    }

    instructions
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

fn blitzmem(addr: u64, val: u64, mutable: u64, index: usize, memory: &mut HashMap<u64, u64>)
{
    if index == 36 {
        memory.insert(addr, val);
        return;
    }

    let ofs = 1u64 << index;
    if mutable & ofs != 0 {
        blitzmem(addr & !ofs, val, mutable, index + 1, memory);
        blitzmem(addr | ofs, val, mutable, index + 1, memory);
    } else {
        blitzmem(addr, val, mutable, index + 1, memory);
    }
}

fn blitz(validtickets: &Vec<Vec<i32>>, groups: &Vec<Vec<(i32, i32)>>, yourticket: &Vec<i32>, mapping: &mut Vec<usize>) {
    if mapping.len() == 20 {
        let mut accum: i64 = 1;
        for i in 0..6 {
            accum *= yourticket[mapping[i]] as i64;
        }
        dbg!(accum);
        return;
    }

    if mapping.len() > 12 {
        dbg!(mapping.len());
    }

    for i in 0..groups.len() {
        if mapping.contains(&i) {
            continue;
        }

        let mut valid = true;
        for ticket in validtickets {
            let mut matched = false;
            for clause in &groups[mapping.len()] {
                if clause.0 <= ticket[i] && ticket[i] <= clause.1 {
                    matched = true;
                }
            }

            if !matched {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        mapping.push(i);
        
        blitz(validtickets, groups, yourticket, mapping);

        mapping.pop();
    }
}

fn bpm(matches: &Vec<Vec<bool>>, group: usize, seen: &mut Vec<bool>, assignments: &mut Vec<Option<usize>>) -> bool
{
    for entry in 0..matches.len() {
        if !seen[entry] && matches[group][entry] {
            seen[entry] = true;

            if assignments[entry].is_none() || bpm(matches, assignments[entry].unwrap(), seen, assignments) {
                assignments[entry] = Some(group);
                return true;
            }
        }
    }

    return false;
}

fn bpm_driver(matches: &Vec<Vec<bool>>) -> Vec<Option<usize>> {
    let mut rv = vec![None; matches.len()];
    for i in 0..matches.len() {
        bpm(matches, i, &mut vec![false; matches.len()], &mut rv);
    }

    rv
}

lazy_static! {
    static ref REGEX_PARENS: regex::Regex = Regex::new(r"\(([^()]+)\)").unwrap();
    static ref REGEX_PLUS: regex::Regex = Regex::new(r"([0-9]+) ([+]) ([0-9]+)").unwrap();
    static ref REGEX_TIMES: regex::Regex = Regex::new(r"([0-9]+) ([*]) ([0-9]+)").unwrap();
}

fn eval(mut input: String) -> String {
    for _ in 0..10 {
        input = REGEX_PARENS.replace_all(&input, |captures: &regex::Captures| eval(captures[1].to_string())).to_string();
    }

    for _ in 0..10 {
        input = REGEX_PLUS.replace(&input, |captures: &regex::Captures| {
            match &captures[2] {
                "+" => (captures[1].parse::<i64>().unwrap() + captures[3].parse::<i64>().unwrap()).to_string(),
                "*" => (captures[1].parse::<i64>().unwrap() * captures[3].parse::<i64>().unwrap()).to_string(),
                _ => panic!(),
            }
        }).to_string();
    }

    for _ in 0..10 {
        input = REGEX_TIMES.replace(&input, |captures: &regex::Captures| {
            match &captures[2] {
                "+" => (captures[1].parse::<i64>().unwrap() + captures[3].parse::<i64>().unwrap()).to_string(),
                "*" => (captures[1].parse::<i64>().unwrap() * captures[3].parse::<i64>().unwrap()).to_string(),
                _ => panic!(),
            }
        }).to_string();
    }

    input
}

#[derive(Debug)]
pub enum Rule {
    Literal(char),
    Split(Vec<i32>),
    Sequence(Vec<i32>),
}

fn process_return(mut sin: Vec<(i32, usize)>, stateNew: &mut HashSet<Vec<(i32, usize)>>, rules: &HashMap<i32, Rule>)
{
    sin.pop();

    while sin.len() > 0 {
        match &rules[&sin.last().unwrap().0] {
            Rule::Sequence(seq) => {
                if seq.len() > sin.last().unwrap().1 {
                    stateNew.insert(sin);
                    return;
                }
            }
            _ => panic!(),
        }

        sin.pop();
    }

    stateNew.insert(sin);
}

fn process_call(mut sin: Vec<(i32, usize)>, kar: char, stateNew: &mut HashSet<Vec<(i32, usize)>>, rules: &HashMap<i32, Rule>)
{
    //dbg!(&sin, kar);

    if sin.len() == 0 {
        // fell off the end.
        return;
    }

    // 
    let last = sin.last().unwrap();
    let rule = &rules[&last.0];
    //dbg!("MATCHHERE", rule, kar);
    match rule {
        Rule::Literal(k2) => {
            if kar == *k2 {
                // a good match, continue
                process_return(sin, stateNew, rules);
            }
        },
        Rule::Split(split) => {
            for sp in split {
                let mut sincopy = sin.clone();
                sincopy.pop();
                sincopy.push((*sp, 0));
                process_call(sincopy, kar, stateNew, rules);
            }
        },
        Rule::Sequence(links) => {
            let mut sincopy = sin.clone();
            let link = links[sincopy.last_mut().unwrap().1];
            sincopy.last_mut().unwrap().1 += 1;
            sincopy.push((link, 0));
            process_call(sincopy, kar, stateNew, rules);
        }
    }
}

fn invert(mut inp: i32) -> i32 {
    let mut result = 0;
    for i in 0..10 {
        result <<= 1;
        result |= inp & 1;
        inp >>= 1;
    }

    result
}

const SIZE: usize = 12;

#[derive(Clone, Copy)]
struct Layout {
    u: i32,
    r: i32,
    d: i32,
    l: i32,
    rot: i32,
}

impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layout")
         .field("u", &format!("{:010b}", self.u))
         .field("r", &format!("{:010b}", self.r))
         .field("d", &format!("{:010b}", self.d))
         .field("l", &format!("{:010b}", self.l))
         .finish()
    }
}

impl Layout {
    fn hflip(self) -> Layout {
        Layout{u: invert(self.u), r: self.l, d: invert(self.d), l: self.r, rot: 0}
    }

    fn vflip(self) -> Layout {
        Layout{u: self.d, r: invert(self.r), d: self.u, l: invert(self.l), rot: 0}
    }

    fn dflip(self) -> Layout {
        Layout{u: self.l, r: self.d, d: self.r, l: self.u, rot: 0}
    }

    fn rot(self, rot: i32) -> Layout {
        let mut rv = self;

        if rot & (1 << 0) != 0 {
            rv = rv.hflip();
        }
        if rot & (1 << 1) != 0 {
            rv = rv.vflip();
        }
        if rot & (1 << 2) != 0 {
            rv = rv.dflip();
        }

        rv.rot = rot;

        rv
    }

    fn nil() -> Layout {
        Layout{u:0, r:0, d:0, l:0, rot:0}
    }
}

fn doitall(x: usize, y: usize, map: &mut [[(i32, Layout); SIZE]; SIZE], tiles: &HashMap<i32, Layout>, payloads: &HashMap<i32, [[bool; 8]; 8]>, used: &mut HashSet<i32>)
{
    if x == SIZE {
        println!("{}, {}", x, y);
        doitall(0, y + 1, map, tiles, payloads, used);
        return;
    }

    if y == SIZE {
        dbg!("DONE");
        // accumulate this dumb thing

        let mut image = [[false; 8*SIZE]; 8*SIZE];
        let mut imagecount = 0;
        for my in 0..SIZE {
            for mx in 0..SIZE {
                dbg!(map[mx][my]);
                let rot = map[mx][my].1.rot;
                let payload = &payloads[&map[mx][my].0];

                for ty in 0..8 {
                    for tx in 0..8 {
                        let mut rx = tx;
                        let mut ry = ty;

                        if rot & (1 << 0) != 0 {
                            rx = 7 - rx;
                        }
                        if rot & (1 << 1) != 0 {
                            ry = 7 - ry;
                        }
                        if rot & (1 << 2) != 0 {
                            mem::swap(&mut rx, &mut ry);
                        }

                        image[mx * 8 + rx][my * 8 + ry] = payload[ty][tx];
                        if payload[ty][tx] {
                            imagecount += 1;
                        }
                    }
                }
            }
        }

        println!("{}", image.iter().map(|lin| lin.iter().map(|&kar| if kar { '#' } else { '.' }).collect::<String>() + "\n").collect::<String>());

        let serpent = [
            (0, 1),
            (1, 2),
            (4, 2),
            (5, 1),
            (6, 1),
            (7, 2),
            (10, 2),
            (11, 1),
            (12, 1),
            (13, 2),
            (16, 2),
            (17, 1),
            (18, 1),
            (18, 0),
            (19, 1),
        ];

        let mut smon = 0;
        for tx in 0..image.len() {
            for ty in 0..image[0].len() {
                let mut valid = true;
                for s in serpent.iter() {
                    let px = tx + s.0;
                    let py = ty + s.1;
                    if px >= image.len() || py >= image[0].len() || !image[py][px] {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    println!("found smon {} {}", tx, ty);
                    smon += serpent.len();
                }
            }
        }

        dbg!(imagecount - smon);

        return;
    }

    for til in tiles {
        if used.contains(&til.0) {
            continue;
        }

        used.insert(*til.0);

        for rot in 0..8 {
            let mut valid = true;

            let lay = til.1.rot(rot);

            if x != 0 && map[x - 1][y].1.r != lay.l {
                valid = false;
            }

            if valid && y != 0 && map[x][y - 1].1.d != lay.u {
                valid = false;
            }

            if valid {
                map[x][y] = (*til.0, lay);
                doitall(x + 1, y, map, &tiles, &payloads, used);
                map[x][y] = (0, Layout::nil());
            }
        }

        used.remove(&til.0);
    }
}

#[derive(Debug)]
pub struct Recipe {
    pub ingredients: HashSet<String>,
    pub allergens: HashSet<String>,
}

fn wanner(mut lhsd: Vec<usize>, mut rhsd: Vec<usize>) -> i32 {
    let mut matches = HashSet::new();

    // TODO infiniloop
    loop {
        let smatch = (lhsd.clone(), rhsd.clone());
        if matches.contains(&smatch) {
            dbg!("EXIT");
            return 0;
        }
        matches.insert(smatch);

        let lhs = lhsd[0];
        let rhs = rhsd[0];

        lhsd.remove(0);
        rhsd.remove(0);

        let mut won = false;
        let mut winner = 0;

        if lhs <= lhsd.len() && rhs <= rhsd.len() {
            let rw = wanner(lhsd.iter().take(lhs).copied().collect(), rhsd.iter().take(rhs).copied().collect());

            if rw == 0 {
                lhsd.push(lhs);
                lhsd.push(rhs);
            } else {
                rhsd.push(rhs);
                rhsd.push(lhs);
            }
        } else {
            if lhs > rhs {
                lhsd.push(lhs);
                lhsd.push(rhs);
            } else {
                rhsd.push(rhs);
                rhsd.push(lhs);
            }
        }

        if lhsd.len() == 0 || rhsd.len() == 0 {
            won = true;
            winner = if lhsd.len() == 0 { 1 } else { 0 };
        }

        if won {
            let result = if winner == 0 { &lhsd } else { &rhsd };

            let mut accum = 0;
            for (idx, item) in result.iter().enumerate() {
                accum += item * (result.len() - idx) as usize;
            }

            dbg!(accum);

            return winner;
        }
    }
}

lazy_static! {
    static ref REGEX_TILE: regex::Regex = Regex::new(r"(e|w|ne|nw|se|sw)").unwrap();
}

fn getloop(subject: i64, public: i64) -> i64 {
    let mut val = 1;
    for i in 1.. {
        val = (val * subject) % 20201227;

        if val == public {
            return i;
        }
    }

    panic!();
}

fn transform(subject: i64, lop: i64) -> i64 {
    let mut val = 1;
    for _ in 0..lop {
        val = (val * subject) % 20201227;
    }

    val
}

fn main() {
    let dirs = read_directions();

    let mut depth = 0;
    let mut dist = 0;
    let mut aim = 0;

    for dir in dirs {
        match dir {
            Direction::Forward(amt) => { dist = dist + amt; depth = depth + aim * amt },
            Direction::Up(amt) => aim = aim - amt,
            Direction::Down(amt) => aim = aim + amt,
        }
    }
    
    dbg!(depth * dist);
}
