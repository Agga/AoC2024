use regex::Regex;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

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

                    println!("A presses {index_a} B presses {index_b}");
                }
            }
        }

        println!("button A {} -> {:?}", a, button_a);
        println!("button B {} -> {:?}", b, button_b);
        println!("prize {:?}", prize);
        println!("cheapest {:?}", cheapest_price);

        total_tokens += cheapest_price.unwrap_or(0);
    }

    total_tokens
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
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

                    println!("A presses {index_a} B presses {index_b}");
                }
            }
        }

        println!("button A {} -> {:?}", a, button_a);
        println!("button B {} -> {:?}", b, button_b);
        println!("prize {:?}", prize);
        println!("cheapest {:?}", cheapest_price);

        total_tokens += cheapest_price.unwrap_or(0);
    }

    //total_tokens
    0
}

fn main() {
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("part2 {}", do_part2(INPUT_DATA));
}

#[test]
fn part1() {
    assert_eq!(480, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA));
}
