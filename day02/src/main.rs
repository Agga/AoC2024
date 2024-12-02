
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()>{

    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "src/input.txt";
    let contents = fs::read_to_string(input)?;

    for line in contents.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|s|s.parse::<i32>().unwrap())
                                    .collect();
    }
    
    Ok(())
}
