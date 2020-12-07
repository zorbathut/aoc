
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

fn main() {
    let lines = read_lines();

    let mut bags: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    let re = Regex::new(r"(?P<count>[0-9]+) (?P<bagtype>[^ ]+ [^ ]+) bag").unwrap();

    for line in lines {
        println!("{:#?}", line);

        let bagname = (line.split(" bags contain ").collect::<Vec<&str>>())[0];
        dbg!(bagname);

        let mut hs = Vec::new();

        if !line.contains("no other bags") {
            for capture in re.captures_iter(&line) {
                let childname = capture.name("bagtype").unwrap().as_str().to_string();
                let childcount = capture.name("count").unwrap().as_str().parse::<i32>().unwrap();
                
                hs.push((childcount, childname.to_string()));
            }
        }

        bags.insert(bagname.to_string(), hs);
    }

    let mut evaluated = HashMap::new();

    dbg!(eval(&bags, "shiny gold", &mut evaluated) - 1);
}
