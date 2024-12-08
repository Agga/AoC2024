use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

fn do_part1(input: &String) -> i32 {
    let mut order_rules: HashMap<&str, HashSet<&str>> = HashMap::new();

    // parse all rules
    for rule in input.lines().filter(|line| line.len() == 5) {
        let mut s = rule.split('|');
        let page0 = s.next().unwrap();
        let page1 = s.next().unwrap();

        order_rules.entry(page1).or_default().insert(page0);
    }

    let mut result = 0;
    for manual in input.lines().filter(|l| l.len() > 5) {
        let mut valid_manual = true;
        let pages: Vec<&str> = manual.split(',').collect();

        for (n, p) in pages.iter().enumerate() {
            if let Some(rules) = order_rules.get(p) {
                for a in pages.iter().skip(n + 1) {
                    if rules.contains(a) {
                        valid_manual = false;
                        break;
                    }
                }
            }
        }

        if valid_manual {
            let middle = pages.len() / 2;
            result += pages.get(middle).unwrap().parse::<i32>().unwrap();
        }
    }

    result
}

fn do_part2(input: &String) -> i32 {
    let mut order_rules: HashMap<&str, HashSet<&str>> = HashMap::new();

    // parse all rules
    for rule in input.lines().filter(|line| line.len() == 5) {
        let mut s = rule.split('|');
        let page0 = s.next().unwrap();
        let page1 = s.next().unwrap();

        order_rules.entry(page1).or_default().insert(page0);
    }

    let mut result = 0;
    for manual in input.lines().filter(|l| l.len() > 5) {
        let mut broken = false;
        let mut pages: Vec<&str> = manual.split(',').collect();

        for (n, p) in pages.iter().enumerate() {
            if let Some(rules) = order_rules.get(p) {
                for a in pages.iter().skip(n + 1) {
                    if rules.contains(a) {
                        broken = true;
                        break;
                    }
                }
            }
        }

        if broken {
            pages.sort_by(|a, b|{
                if order_rules.get(a).map(|s|s.contains(b)).unwrap_or(false) {
                    return Ordering::Less;
                }
                else {
                    return Ordering::Greater;
                }
            });

            let middle = pages.len() / 2;
            result += pages.get(middle).unwrap().parse::<i32>().unwrap();
        }
    }

    result
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "day05/input.txt";
    let contents = fs::read_to_string(input)?;

    println!("part1 {}", do_part1(&contents));
    println!("part2 {}", do_part2(&contents));

    Ok(())
}
