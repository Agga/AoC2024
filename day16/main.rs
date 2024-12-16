use std::time::Instant;

use aoc::{Grid, Vec2};

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq, Default)]
enum Field {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '#' => Field::Wall,
            'S' => Field::Start,
            'E' => Field::End,
            '.' => Field::Empty,
            _ => panic!("char -> field panic"),
        }
    }
}

impl From<Field> for char {
    fn from(value: Field) -> Self {
        match value {
            Field::Empty => '.',
            Field::Wall => '#',
            Field::Start => 'S',
            Field::End => 'E',
        }
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let mut start = Vec2::default();
    let mut end = Vec2::default();

    let mut grid_data = vec![];

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let f = Field::from(c);

            match f {
                Field::Start => {
                    start = Vec2::new(x as i32, y as i32);
                }
                Field::End => {
                    end = Vec2::new(x as i32, y as i32);
                }
                _ => {}
            }

            grid_data.push(f);
        }
    }

    let h = input.lines().count();
    let w = grid_data.len() / h;

    let grid = Grid::new(w as i32, h as i32, grid_data);
    println!("{:?}", grid);

    0
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    0
}

fn main() {
    let mut now = Instant::now();
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("{:?}", now.elapsed());

    now = Instant::now();
    println!("part2 {}", do_part2(INPUT_DATA));
    println!("{:?}", now.elapsed());
}

#[allow(dead_code)]
const TEST_DATA0: &str = include_str!("test0.txt");
#[allow(dead_code)]
const TEST_DATA1: &str = include_str!("test1.txt");

#[test]
fn part1() {
    assert_eq!(7036, do_part1(TEST_DATA0));
    assert_eq!(11048, do_part1(TEST_DATA1));
}

#[test]
fn part2() {
    // assert_eq!(0, do_part2(TEST_DATA0));
    // assert_eq!(0, do_part2(TEST_DATA1));
}
