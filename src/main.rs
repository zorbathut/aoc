
use std::io;
use regex::Regex;

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
                println!("{:#?}", input);
                rv.push(input);
            },
            Err(_) => return rv,
        }
    }
}

fn main() {
    let lines = read_lines();

    let re = Regex::new(r"^(?P<min>[\d]+)-(?P<max>[\d]+) (?P<char>.): (?P<pw>.*)$").unwrap();

    let mut valid = 0;

    for pwline in &lines {
        let captures = re.captures(pwline).unwrap();

        println!("{:#?}", captures);

        let min = captures.name("min").unwrap().as_str().parse::<i32>().unwrap();
        let max = captures.name("max").unwrap().as_str().parse::<i32>().unwrap();
        let kar = captures.name("char").unwrap().as_str();
        let pw = captures.name("pw").unwrap().as_str();

        let counted = pw.matches(kar).count() as i32;

        println!("{}: {} {} {} {}: {}", pwline, min, max, kar, pw, counted);
        if counted >= min && counted <= max {
            valid += 1;
        }
    }

    println!("{}", valid);
}
