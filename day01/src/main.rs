use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "src/input.txt";

    let contents = fs::read_to_string(input)?;

    let mut left = vec![];
    let mut right = vec![];

    for line in contents.lines() {
        let mut words = line.split_whitespace();
        left.push(words.next().unwrap().parse::<usize>().unwrap());
        right.push(words.next().unwrap().parse::<usize>().unwrap());
    }

    left.sort();
    right.sort();

    let result_part1: usize = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(r.clone()))
        .sum();

    println!("part01 {}", result_part1);

    let result_part2: usize = left
        .iter()
        .map(|number| number * right.iter().filter(|r| &number == r).count())
        .sum();

    println!("part02 {}", result_part2);

    Ok(())
}
