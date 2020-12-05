
use std::io;
use std::cmp;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

fn read_numbers() -> Vec<i32> {
    let mut rv: Vec<i32> = Vec::new();

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => return rv,
            Ok(_) => {
                input = input.trim().to_string();
                println!("{:#?}", input);
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

    let mut seats = HashSet::new();

    for line in lines {
        println!("{:#?}", line);

        let mut pos = 0;

        for kar in line.chars() {
            pos <<= 1;

            if kar == 'B' || kar == 'R' {
                pos += 1;
            }
        }

        best = cmp::max(best, pos);
        seats.insert(pos);
    }

    for i in 1..2000 {
        if seats.contains(&(i - 1)) && seats.contains(&(i + 1)) && !seats.contains(&i) {
            println!("{}", i);
        }
    }

    println!("{:#?}", best);
}
