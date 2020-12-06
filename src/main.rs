
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

fn main() {
    let lines = read_lines();

    let mut best = 0;

    let mut first = true;
    let mut seats: HashSet<char> = HashSet::new();

    for line in lines {
        println!("{:#?}", line);

        if line.len() == 0 {
            best += seats.len();
            println!("ADD: {:#?}", seats.len());
            seats.clear();
            first = true;
        } else if first {
            for kar in line.chars() {
                seats.insert(kar);
            }
            first = false;
        } else {
            let mut seatsb: HashSet<char> = HashSet::new();
            for kar in line.chars() {
                seatsb.insert(kar);
            }

            seats = seats.intersection(&seatsb).copied().collect();
            println!("SEE: {:#?}", seats);
        }
    }

    println!("{:#?}", best);
}
