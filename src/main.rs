
use std::mem;
use std::io;
use std::cmp;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

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

trait IterExt : Iterator {
    fn count_if<F>(self, f: F) -> usize
    where 
        Self: Sized,
        F: Fn(&Self::Item) -> bool,
    {
        self.filter(f).count()
    }     
}

impl<I: Iterator> IterExt for I {}

fn main() {
    let mut dx: [i32; 4] = [1, 0, -1, 0];
    let mut dy: [i32; 4] = [0, 1, 0, -1];

    let mut direction: i32 = 0;
    let mut cx = 0;
    let mut cy = 0;
    let mut wx = 10;
    let mut wy = -1;

    for line in read_lines() {
        dbg!(&line);

        let (cmd, amount) = scan_fmt!(&line, "{/./}{}", char, i32).unwrap();

        match (cmd, amount) {
            ('N', amt) => wy -= amt,
            ('S', amt) => wy += amt,
            ('E', amt) => wx += amt,
            ('W', amt) => wx -= amt,
            ('F', amt) => { cx += wx * amt; cy += wy * amt; },
            ('L', 0) | ('R', 0) => (),
            ('L', 90) | ('R', 270) => { mem::swap(&mut wx, &mut wy); wy *= -1; }
            ('L', 270) | ('R', 90) => { mem::swap(&mut wx, &mut wy); wx *= -1; }
            ('L', 180) | ('R', 180) => { wx *= -1; wy *= -1; }
            _ => panic!(),
        }

        dbg!(cx, cy, direction);
    }

    dbg!(cx.abs() + cy.abs());
}
