
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
    let mut vals = read_lines();

    let mut current: Vec<Vec<char>> = vals.into_iter().map(|lin| lin.chars().collect()).collect();
    
    let mut dx: [i32; 8] = [0, 0, 1, -1, 1, 1, -1, -1];
    let mut dy: [i32; 8] = [1, -1, 0, 0, 1, -1, 1, -1];

    loop {
        let mut next = Vec::new();

        for row in 0..current.len() {
            let mut nextline = Vec::new();
            for col in 0..current[row].len() {
                let mut adjacencies = 0;
                for d in 0..dx.len() {
                    let tr = (row as i32) + dx[d];
                    let tc = (col as i32) + dy[d];
                    if tr < 0 || tr >= current.len() as i32 {
                        continue;
                    }
                    if tc < 0 || tc >= current[tr as usize].len() as i32 {
                        continue;
                    }

                    if current[tr as usize][tc as usize] == '#' {
                        adjacencies += 1;
                    }
                }

                if current[row][col] == '#' && adjacencies >= 4 {
                    nextline.push('L');
                } else if current[row][col] == 'L' && adjacencies == 0 {
                    nextline.push('#');
                } else {
                    nextline.push(current[row][col]);
                }
            }
            next.push(nextline);
        }

        if next == current {
            let result: usize = current.iter().map(|line| line.iter().count_if(|&&c| c == '#')).sum();
            dbg!(result);
            break;
        }

        println!("---");
        for lin in next.iter().map(|line| line.iter().collect::<String>()) {
            println!("{}", lin);
        }
        
        current = next;
    }

    
}
