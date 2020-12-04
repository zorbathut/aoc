
use std::io;
use regex::Regex;
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

    let re = Regex::new(r"(?P<key>[a-z]+):(?P<value>[^ ]+)").unwrap();

    let reYears = Regex::new(r"^[\d]{4}$").unwrap();
    let reHeight = Regex::new(r"^(?P<num>\d+)(?P<value>in|cm)").unwrap();
    let reColor = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let reEye = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let rePid = Regex::new(r"^[\d]{9}$").unwrap();

    let mut keys = HashMap::new();
    let mut count = 0;

    for line in lines {
        println!("{:#?}", line);

        if re.is_match(&line) {
            for capture in re.captures_iter(&line) {
                println!("{:#?}", capture);

                keys.insert(capture.name("key").unwrap().as_str().to_string(), capture.name("value").unwrap().as_str().to_string());
            }
        } else {
            println!("FIN");

            if keys.contains_key("byr") && keys.contains_key("iyr") && keys.contains_key("eyr") && keys.contains_key("hgt") && keys.contains_key("hcl") && keys.contains_key("ecl") && keys.contains_key("pid") {
                let mut valid = true;
                if !reYears.is_match(&keys["byr"]) {
                    valid = false;
                }
                if !reYears.is_match(&keys["iyr"]) {
                    valid = false;
                }
                if !reYears.is_match(&keys["eyr"]) {
                    valid = false;
                }
                if !reHeight.is_match(&keys["hgt"]) {
                    valid = false;
                }
                if !reColor.is_match(&keys["hcl"]) {
                    valid = false;
                }
                if !reEye.is_match(&keys["ecl"]) {
                    valid = false;
                }
                if !rePid.is_match(&keys["pid"]) {
                    valid = false;
                }

                if valid {
                    let byr = keys["byr"].parse::<i32>().unwrap();
                    let iyr = keys["iyr"].parse::<i32>().unwrap();
                    let eyr = keys["eyr"].parse::<i32>().unwrap();

                    if byr < 1920 || byr > 2002 || iyr < 2010 || iyr > 2020 || eyr < 2020 || eyr > 2030 {
                        valid = false;
                    }

                    let hgt = reHeight.captures(&keys["hgt"]).unwrap();
                    let hgam = hgt.name("num").unwrap().as_str().parse::<i32>().unwrap();
                    let hgun = hgt.name("value").unwrap().as_str();

                    if hgun == "cm" && (hgam < 150 || hgam > 193) {
                        valid = false;
                    }
                    if hgun == "in" && (hgam < 59 || hgam > 76) {
                        valid = false;
                    }

                    if valid {
                        count += 1;
                        println!("VALID");    
                    }
                }
            }

            keys.clear();
        }
    }

    println!("{:#?}", count);
}
