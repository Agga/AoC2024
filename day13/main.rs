use regex::Regex;
use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

const OFFSET: i64 = 10000000000000;

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let re = Regex::new(r"X(\+|=)(\d+), Y(\+|=)(\d+)").expect("Invalid regex");

    let parse_text = |s: &str| -> (usize, usize) {
        if let Some(captures) = re.captures(s) {
            if let (Some(x), Some(y)) = (captures.get(2), captures.get(4)) {
                let x_val = x.as_str().parse().expect("Failed to parse X coordinate");
                let y_val = y.as_str().parse().expect("Failed to parse Y coordinate");
                return (x_val, y_val);
            }
        }

        panic!();
    };

    let mut total_tokens = 0;

    let mut iter = input.lines().filter(|l| !l.is_empty());
    while let Some(a) = iter.next() {
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();

        let button_a = parse_text(a);
        let button_b = parse_text(b);
        let prize = parse_text(c);

        let price_a = 3;
        let price_b = 1;

        let mut cheapest_price: Option<usize> = None;

        let max_tries = 100;
        for index_a in 0..=max_tries {
            for index_b in 0..=max_tries {
                let result = (
                    button_a.0 * index_a + button_b.0 * index_b,
                    button_a.1 * index_a + button_b.1 * index_b,
                );

                if result == prize {
                    let price = price_a * index_a + price_b * index_b;

                    cheapest_price = match cheapest_price {
                        None => Some(price),
                        Some(curr) => {
                            if price < curr {
                                Some(price)
                            } else {
                                Some(curr)
                            }
                        }
                    };
                }
            }
        }

        total_tokens += cheapest_price.unwrap_or(0);
    }

    total_tokens
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> i64 {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )
    .expect("invalid regular expression");

    let calc_linear_combination = |px: i64, py: i64, ax: i64, ay: i64, bx: i64, by: i64| -> i64 {
        let det = ax * by - ay * bx;
        if det == 0 {
            return 0;
        }

        let num_a = px * by - py * bx;
        let num_b = py * ax - px * ay;

        // Check if integer solution exists
        if num_a % det != 0 || num_b % det != 0 {
            return 0;
        }

        let a = num_a / det;
        let b = num_b / det;

        if a >= 0 && b >= 0 {
            return 3 * a + b;
        }
        0
    };

    let mut result = 0;
    for caps in re.captures_iter(&input) {
        let ax: i64 = caps[1].parse().unwrap();
        let ay: i64 = caps[2].parse().unwrap();
        let bx: i64 = caps[3].parse().unwrap();
        let by: i64 = caps[4].parse().unwrap();
        let px: i64 = caps[5].parse().unwrap();
        let py: i64 = caps[6].parse().unwrap();

        result += calc_linear_combination(px + OFFSET, py + OFFSET, ax, ay, bx, by);
    }

    result

}

fn main() {
    let mut now = Instant::now();
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("{:?}", now.elapsed());

    now = Instant::now();
    println!("part2 {}", do_part2(INPUT_DATA));
    println!("{:?}", now.elapsed());
}

#[test]
fn part1() {
    assert_eq!(480, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(875318608908, do_part2(TEST_DATA));
}
