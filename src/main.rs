
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

fn eval(bags: &HashMap<String, HashSet<String>>, bag: &str, evaluated: &mut HashSet<String>, valid: &mut HashSet<String>)
{
    if evaluated.contains(bag) {
        return
    }

    evaluated.insert(bag.to_string());

    let mut victory = false;

    for child in &bags[bag] {
        eval(bags, &child, evaluated, valid);
        if valid.contains(child) {
            victory = true;
        }
    }

    if victory {
        valid.insert(bag.to_string());
    }
}

fn eval2(bags: &HashMap<String, HashSet<String>>, bag: &str, evaluated: &mut HashSet<String>, valid: &mut HashSet<String>)
{
    if evaluated.contains(bag) {
        return
    }

    evaluated.insert(bag.to_string());
    valid.insert(bag.to_string());

    if bags.contains_key(bag) {
        for child in &bags[bag] {
            eval2(bags, child, evaluated, valid);
        }
    }
}

fn main() {
    let lines = read_lines();

    let mut bags: HashMap<String, HashSet<String>> = HashMap::new();
    let mut bagContain: HashMap<String, HashSet<String>> = HashMap::new();
    let re = Regex::new(r"(?P<count>[0-9]+) (?P<bagtype>[^ ]+ [^ ]+) bag").unwrap();

    for line in lines {
        println!("{:#?}", line);

        let bagname = (line.split(" bags contain ").collect::<Vec<&str>>())[0];
        dbg!(bagname);

        let mut hs: HashSet<String> = HashSet::new();

        if !line.contains("no other bags") {
            for capture in re.captures_iter(&line) {
                let childname = capture.name("bagtype").unwrap().as_str().to_string();
                dbg!(&childname);
                hs.insert(childname.to_string());

                if !bagContain.contains_key(&childname) {
                    bagContain.insert(childname.to_string(), HashSet::new());
                }
                bagContain.get_mut(&childname).unwrap().insert(bagname.to_string());
            }
        }

        bags.insert(bagname.to_string(), hs);
    }

    let mut evaluated = HashSet::new();
    let mut valid = HashSet::new();

    eval2(&bagContain, "shiny gold", &mut evaluated, &mut valid);

    /*let mut evaluated = HashSet::new();
    let mut valid = HashSet::new();
    evaluated.insert("shiny gold".to_string());
    valid.insert("shiny gold".to_string());
    for bag in &bags {
        eval(&bags, &bag.0, &mut evaluated, &mut valid);
    }*/
    dbg!(&valid);

    dbg!(valid.len() - 1);
}
