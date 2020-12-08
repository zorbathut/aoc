
use std::io;
use std::cmp;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

#[macro_use] extern crate scan_fmt;

fn read_numbers() -> Vec<i32> {
    let mut rv: Vec<i32> = Vec::new();

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => return rv,
            Ok(_) => {
                input = input.trim().to_string();
                rv.push(input.parse::<i32>().unwrap());
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

fn eval(bags: &HashMap<String, Vec<(i32, String)>>, bag: &str, result: &mut HashMap<String, i32>) -> i32
{
    if result.contains_key(bag) {
        return result[bag]
    }

    let mut contents = 1;

    if bags.contains_key(bag) {
        for child in &bags[bag] {
            contents += eval(bags, &child.1, result) * child.0;
        }
    }

    result.insert(bag.to_string(), contents);

    contents
}

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

fn main() {
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
            instructions.push(Instruction::Nop);
        }
    }

    let mut acc = 0;
    let mut inst: i32 = 0;

    let mut seen = HashSet::new();

    loop {
        if seen.contains(&inst) {
            dbg!(acc);
            return;
        }
        seen.insert(inst);

        match instructions[inst as usize] {
            Instruction::Acc(arg) => { acc += arg; inst += 1; }
            Instruction::Jmp(arg) => { inst += arg; }
            Instruction::Nop => { inst += 1; }
        }
    }
}
