
use std::mem;
use std::io;
use std::cmp;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use num::integer;

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

fn main() {
    let mut state = HashSet::new();

    for row in read_lines().iter().enumerate() {
        for col in row.1.chars().enumerate() {
            if col.1 == '#' {
                state.insert((row.0 as i32, col.0 as i32, 0, 0));
            }
        } 
    }

    let dw = vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let dx = vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let dy = vec![-1, -1, -1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 1, 1];
    let dz = vec![-1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 1, 0, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 1, 0, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1];

    for i in 0..6 {
        let nx = state.iter().map(|s| s.0).min().unwrap() - 1;
        let xx = state.iter().map(|s| s.0).max().unwrap() + 1;
        let ny = state.iter().map(|s| s.1).min().unwrap() - 1;
        let xy = state.iter().map(|s| s.1).max().unwrap() + 1;
        let nz = state.iter().map(|s| s.2).min().unwrap() - 1;
        let xz = state.iter().map(|s| s.2).max().unwrap() + 1;
        let nw = state.iter().map(|s| s.3).min().unwrap() - 1;
        let xw = state.iter().map(|s| s.3).max().unwrap() + 1;

        let mut newstate = HashSet::new();

        for x in nx..=xx {
            for y in ny..=xy {
                for z in nz..=xz {
                    for w in nw..=xw {
                        let mut ct = 0;
                        for d in 0..80 {
                            if state.contains(&(x + dx[d], y + dy[d], z + dz[d], w + dw[d])) {
                                ct += 1;
                            }
                        }
                        
                        if state.contains(&(x, y, z, w)) && (ct == 2 || ct == 3) {
                            newstate.insert((x, y, z, w));
                        } else if !state.contains(&(x, y, z, w)) && ct == 3 {
                            newstate.insert((x, y, z, w));
                        }
                    }
                }
            }
        }

        state = newstate;
    }

    dbg!(state.len());
}
