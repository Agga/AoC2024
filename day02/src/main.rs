
use std::env;
use std::fs;
use std::io;

use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing
}

#[derive(PartialEq, Eq)]
enum Safety{
    Safe,
    Unsafe
}

fn validate_report_part1( report: &Vec<i32> ) -> Safety {

    let mut direction: Option<Direction> = None;

    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        
        // distance too big
        if diff.abs() > 3 {
            return Safety::Unsafe
        }

        // calc direction of current comparison
        let dir = match diff.signum() {
            -1 => Direction::Increasing,
            1 => Direction::Decreasing,
            _ => { return Safety::Unsafe; } // equals 0
        };

        match &direction {
            None => { direction = Some(dir); },
            Some( i) => {
                if i != &dir {
                    return Safety::Unsafe;
                }
            }
        } 
    }

    Safety::Safe    
}

fn validate_report_part2( report: &Vec<i32> ) -> Safety {

    if validate_report_part1( &report ) == Safety::Unsafe {

        for index in 0..report.len() {
            let mut copy = report.clone();
            copy.remove( index );

            if validate_report_part1( &copy) == Safety::Safe {
                return Safety::Safe;
            }
        }

        return Safety::Unsafe;
    }

    Safety::Safe
}

fn main() -> io::Result<()>{

    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "src/input.txt";
    let contents = fs::read_to_string(input)?;

    let mut safe_reports_part1 = 0;
    let mut safe_reports_part2 = 0;
    for line in contents.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|s|s.parse::<i32>().unwrap())
                                    .collect();

        if validate_report_part1( &numbers ) == Safety::Safe {
            safe_reports_part1 += 1;
        }

        if validate_report_part2( &numbers ) == Safety::Safe {
            safe_reports_part2 += 1;
        }
    }

    println!( "number of safe reports part1 {}", safe_reports_part1 );
    println!( "number of safe reports part2 {}", safe_reports_part2 );
    
    Ok(())
}
