use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
    time::Instant,
};

use aoc::Vec2;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[allow(unused_variables)]
pub fn do_part1(input: &str, width: i32, height: i32, blinks: i32) -> usize {
    let re =
        Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").expect("invalid regular expression");

    #[derive(Debug)]
    struct Robot {
        position: Vec2,
        velocity: Vec2,
    }

    let robots: Vec<_> = re
        .captures_iter(&input)
        .map(|caps| {
            let px: i32 = caps[1].parse().unwrap();
            let py: i32 = caps[2].parse().unwrap();
            let vx: i32 = caps[3].parse().unwrap();
            let vy: i32 = caps[4].parse().unwrap();

            Robot {
                position: Vec2::new(px, py),
                velocity: Vec2::new(vx, vy),
            }
        })
        .collect();

    let time = Vec2::broadcast(blinks);
    let wrap = |v: i32, max: i32| (v % max + max) % max;

    let new_positions: HashMap<_, usize> = robots.iter().fold(HashMap::new(), |mut map, robot| {
        let mut pos = robot.position + robot.velocity * time;
        pos.x = wrap(pos.x, width);
        pos.y = wrap(pos.y, height);

        let entry = map.entry(pos).or_insert(0);
        *entry += 1;

        map
    });

    let half_w = width / 2;
    let half_h = height / 2;

    println!("blinks {}", blinks);

    for y in 0..height {
        for x in 0..width {
            let mut count = match new_positions.get(&Vec2 { x, y }) {
                Some(&count) => count,
                None => 0,
            };

            if x == half_w || y == half_h {
                count = 0;
            }

            if count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }

    let mut quadrants = [0usize; 4];
    // println!("{:?}", quadrants);

    for (pos, count) in new_positions
        .iter()
        .filter(|(&pos, _)| pos.x != half_w && pos.y != half_h)
    {
        let mut index = 0;
        index += if pos.x > half_w { 1 } else { 0 };
        index += if pos.y > half_h { 2 } else { 0 };
        quadrants[index] += count;
    }

    // println!("{:?}", quadrants);

    let safety_factor: usize = quadrants.iter().fold(1, |a, b| a * b);
    safety_factor
}

#[allow(unused_variables)]
pub fn do_part2(input: &str, width: i32, height: i32) -> usize {
    let re =
        Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").expect("invalid regular expression");

    #[derive(Debug)]
    struct Robot {
        position: Vec2,
        velocity: Vec2,
    }

    let robots: Vec<_> = re
        .captures_iter(&input)
        .map(|caps| {
            let px: i32 = caps[1].parse().unwrap();
            let py: i32 = caps[2].parse().unwrap();
            let vx: i32 = caps[3].parse().unwrap();
            let vy: i32 = caps[4].parse().unwrap();

            Robot {
                position: Vec2::new(px, py),
                velocity: Vec2::new(vx, vy),
            }
        })
        .collect();

    let half_w = width / 2;
    let half_h = height / 2;
    let wrap = |v: i32, max: i32| (v % max + max) % max;
    let calculate_positions = |robots: &Vec<Robot>, blinks: i32| -> Vec<Vec2> {
        let time = Vec2::broadcast(blinks);
        robots
            .iter()
            .map(|robot| {
                let mut pos = robot.position + robot.velocity * time;
                pos.x = wrap(pos.x, width);
                pos.y = wrap(pos.y, height);
                pos
            })
            // .filter(|pos| pos.x != half_w && pos.y != half_h)
            .collect()
    };

    let something = |positions: &Vec<Vec2>| -> HashMap<Vec2, usize> {
        return positions.iter().fold(HashMap::new(), |mut map, &pos| {
            *map.entry(pos).or_insert(0) += 1;
            map
        });
    };

    let f = std::fs::File::create("day14/maybe_tree.txt").expect("test");
    let mut f = BufWriter::new(f);

    // fs::write( "test.txt", "" );

    let mut draw = |map: &HashMap<Vec2, usize>, blinks| {
        writeln!(&mut f, "blinks {}", blinks).expect("msg");
        for y in 0..height {
            for x in 0..width {
                if let Some(count) = map.get(&Vec2 { x, y }) {
                    write!(&mut f, "{}", count).expect("a");
                } else {
                    write!(&mut f, ".").expect("b");
                }
            }
            writeln!(&mut f).expect("c");
        }
    };

    let all_blinks = vec![1, 2, 3, 4, 5, 100, 1000, 6512, 10000 ];

    for blinks in all_blinks {
        println!("blinks {}", blinks);
        let pos = calculate_positions(&robots, blinks);
        let map = something(&pos);

        draw(&map, blinks);
    }

    0
}

fn main() {
    let mut now = Instant::now();
    println!("part1 {}", do_part1(INPUT_DATA, 101, 103, 100));
    println!("{:?}", now.elapsed());

    now = Instant::now();
    println!("part2 {}", do_part2(INPUT_DATA, 101, 103));
    println!("{:?}", now.elapsed());
}

#[test]
fn part1() {
    assert_eq!(12, do_part1(TEST_DATA, 11, 7, 100));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA, 101, 103));
}
