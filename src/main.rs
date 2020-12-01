
use std::io;

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

fn main() {
    let numbers = read_numbers();
    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if x + y + z == 2020 {
                    println!("{} {} {}", x, y, z);
                }
            }
        }
    }
}
