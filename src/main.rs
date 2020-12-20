
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

#[derive(Clone, Copy)]
struct Layout {
    u: i32,
    r: i32,
    d: i32,
    l: i32,
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
        Layout{u: invert(self.u), r: self.l, d: invert(self.d), l: self.r}
    }

    fn vflip(self) -> Layout {
        Layout{u: self.d, r: invert(self.r), d: self.u, l: invert(self.l)}
    }

    fn dflip(self) -> Layout {
        Layout{u: self.l, r: self.d, d: self.r, l: self.u}
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

        rv
    }

    fn nil() -> Layout {
        Layout{u:0, r:0, d:0, l:0}
    }
}

fn doitall(x: usize, y: usize, map: &mut [[(i32, Layout); 12]; 12], tiles: &HashMap<i32, Layout>, lookup: &HashMap<i32, Vec<i32>>, used: &mut HashSet<i32>)
{
    if x == 12 {
        doitall(0, y + 1, map, tiles, lookup, used);
        return;
    }

    if y == 12 {
        dbg!("DONE");
        let mut accum = 1i64;
        accum *= map[0][0].0 as i64;
        accum *= map[11][0].0 as i64;
        accum *= map[0][11].0 as i64;
        accum *= map[11][11].0 as i64;
        dbg!(accum);
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

            if valid && y != 0 && map[x][y - 1].1.u != lay.d {
                valid = false;
            }

            if valid {
                map[x][y] = (*til.0, lay);
                doitall(x + 1, y, map, &tiles, lookup, used);
                map[x][y] = (0, Layout::nil());
            }
        }

        used.remove(&til.0);
    }
}

fn main() {
    let chunks = read_groups();

    let mut tiles: HashMap<i32, Layout> = HashMap::new();
    let mut lookup: HashMap<i32, Vec<i32>> = HashMap::new();

    fn poosh(id: i32, val: i32, lookup: &mut HashMap<i32, Vec<i32>>)
    {
        match lookup.entry(id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Vec::new()),
        }.push(val);
    }

    for tile in &chunks {
        let id = scan_fmt!(&tile[0], "Tile {}:", i32).unwrap();

        let mut l = 0;
        let mut r = 0;
        for line in tile.iter().skip(1) {
            l <<= 1;
            r <<= 1;
            if line.chars().nth(0).unwrap() == '#' {
                l += 1;
            }
            if line.chars().nth(line.len() - 1).unwrap() == '#' {
                r += 1;
            }
        }

        let mut u = 0;
        for kar in tile[1].chars() {
            u <<= 1;
            if kar == '#' {
                u += 1;
            }
        }

        let mut d = 0;
        for kar in tile.last().unwrap().chars() {
            d <<= 1;
            if kar == '#' {
                d += 1;
            }
        }

        println!("{}: {} {} {} {}", id, l, r, u, d);

        poosh(id, l, &mut lookup);
        poosh(id, r, &mut lookup);
        poosh(id, u, &mut lookup);
        poosh(id, d, &mut lookup);

        poosh(id, invert(l), &mut lookup);
        poosh(id, invert(r), &mut lookup);
        poosh(id, invert(u), &mut lookup);
        poosh(id, invert(d), &mut lookup);

        tiles.insert(id, Layout{u:u, r:r, d:d, l:l});
    }

    let mut map = [[(0i32, Layout::nil()); 12]; 12];

    doitall(0, 0, &mut map, &tiles, &lookup, &mut HashSet::new())
}
