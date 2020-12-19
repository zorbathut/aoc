
use std::mem;
use std::io;
use std::cmp;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
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

fn main() {
    let chunks = read_groups();
    let mut rules = HashMap::new();

    for inst in &chunks[0] {
        let init: Vec<&str> = inst.split(": ").collect();
        let key = init[0].parse::<i32>().unwrap();
        let tok = init[1];

        if tok.chars().nth(0).unwrap() == '"' {
            rules.insert(key, Rule::Literal(tok.chars().nth(1).unwrap()));
        } else {
            let mut res = Vec::new();
            for (id, piece) in tok.split(" | ").enumerate() {
                let ki = key + 1000 * (id as i32 + 1);
                rules.insert(ki, Rule::Sequence(piece.split(" ").map(|t| t.parse::<i32>().unwrap()).collect()));
                res.push(ki);
            }
            rules.insert(key, Rule::Split(res));
        }
    }

    dbg!(&rules);

    let mut count = 0;
    for line in &chunks[1] {
        let mut state: HashSet<Vec<(i32, usize)>> = HashSet::new();
        state.insert(vec![(0, 0)]);

        for kar in line.chars() {
            // process state here!

            let mut stateNew = HashSet::new();

            for sin in state {
                process_call(sin, kar, &mut stateNew, &rules);
            }

            state = stateNew;
        }

        if state.contains(&Vec::new()) {
            count += 1;
            dbg!(&line);
        }
    }

    dbg!(count);
}
