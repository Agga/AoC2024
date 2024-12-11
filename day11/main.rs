use std::collections::HashMap;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

// 1.   0 -> 1
// 2.   even -> split in two
//      1000 -> 10 and 00 -> 10 + 0
// 3.   multiply by 2024
// 4.   order is preserved

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let mut data: Vec<usize> = input
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut result: Vec<usize> = Vec::new();

    let blinks = 25;

    for _ in 0..blinks {
        for &element in data.iter() {
            // rule 1
            if element == 0 {
                result.push(1);
                continue;
            }

            // rule 2, even number of digits
            let mut l = element.to_string();
            if l.len() % 2 == 0 {
                // is even
                let r = l.split_off(l.len() / 2);
                result.push(l.parse().unwrap());
                result.push(r.parse().unwrap());
                continue;
            }

            // count digits + split

            result.push(element * 2024);
        }

        data = result.clone();
        result.clear();
    }

    data.len()
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    let mut data: Vec<usize> = input
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut result: Vec<usize> = Vec::new();

    let blinks = 75;

    let mut dict: HashMap<usize, Vec<usize>> = HashMap::new();

    for n in 0..blinks {
        for &element in data.iter() {
            dict.entry(element);

            // rule 1
            if element == 0 {
                result.push(1);
                continue;
            }

            // rule 2, even number of digits
            let mut l = element.to_string();
            if l.len() % 2 == 0 {
                // is even
                let r = l.split_off(l.len() / 2);
                result.push(l.parse().unwrap());
                result.push(r.parse().unwrap());
                continue;
            }

            // count digits + split

            result.push(element * 2024);
        }

        data = result.clone();
        result.clear();

        println!("({}/{}) {}", n + 1, blinks, data.len());
    }

    data.len()
}

fn main() {
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("part2 {}", do_part2(INPUT_DATA));
}

#[test]
fn part1() {
    let e: usize = 1000;
    let mut l = e.to_string();
    assert_eq!(4, l.len());
    assert!(l.len() % 2 == 0);
    if l.len() % 2 == 0 {
        let r = l.split_off(l.len() / 2);
        assert_eq!(l, "10");
        assert_eq!(r, "00");
    }

    assert_eq!(55312, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA));
}
