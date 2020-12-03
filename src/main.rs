
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

    let mut dx: [usize; 5] = [1, 3, 5, 7, 1];
    let mut dy: [usize; 5] = [1, 1, 1, 1, 2];

    let mut accum: i64 = 1;

    for i in 0..dx.len() {
        let mut x = 0;
        let mut y = 0;

        let mut trees = 0;

        while y < lines.len() {
            let spot = lines[y].bytes().nth(x % lines[y].len()).unwrap();
            if spot == '#' as u8 {
                trees += 1;
            }

            x += dx[i];
            y += dy[i];
        }

        accum *= trees;
    }
    
    dbg!(accum);
}
