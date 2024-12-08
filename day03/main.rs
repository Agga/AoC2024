use std::env;
use std::fs;
use std::io;

use regex::Regex;

fn do_part1(input: &String) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let nu = Regex::new(r"\d+").unwrap();

    let mut result = 0;
    for mul in re.find_iter(&input) {
        let mut iter = nu.find_iter(mul.as_str());
        let num0 = iter.next().unwrap().as_str().parse::<i32>().unwrap();
        let num1 = iter.next().unwrap().as_str().parse::<i32>().unwrap();
        result += num0 * num1;
    }

    result
}

fn do_part2(input: &String) -> i32 {
    let bla = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let num = Regex::new(r"\d+").unwrap();

    let mut action = true;
    let mut result = 0;
    for op in bla.find_iter(&input) {
        match op.len() {
            4 => {
                action = true;
            }
            7 => {
                action = false;
            }
            _ => {
                if action {
                    let mut iter = num.find_iter(op.as_str());
                    let num0 = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                    let num1 = iter.next().unwrap().as_str().parse::<i32>().unwrap();
                    result += num0 * num1;
                }
            }
        }
    }
    result
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "day03/input.txt";
    let contents = fs::read_to_string(input)?;

    println!("part1 {}", do_part1(&contents));
    println!("part2 {}", do_part2(&contents));

    Ok(())
}
