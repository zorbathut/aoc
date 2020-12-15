
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

fn main() {
    let start: Vec<_> = io::stdin().read_line_direct().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
    let mut lookup = HashMap::new();

    let mut index = 0;
    let mut current = 0;

    for element in start {
        println!("{}: {}", index, element);
        match lookup.insert(element, index) {
            None => current = 0,
            Some(last) => current = index - last,
        }
            
        index += 1;
    }

    while index < 30000000 - 1 {
        if index % 100000 == 0 {
            println!("{}: {}", index, current);
        }
        match lookup.insert(current, index) {
            None => current = 0,
            Some(last) => current = index - last,
        }
            
        index += 1;
    }

    dbg!(current);
}
