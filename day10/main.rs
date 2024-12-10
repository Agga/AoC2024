use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

pub struct Grid {
    width: i32,
    height: i32,
    data: Vec<usize>,
    start: Vec<Vec2>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut data: Vec<usize> = Vec::with_capacity(input.len());
        let mut start: Vec<Vec2> = Vec::new();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let e = c.to_digit(10).unwrap() as usize;
                if e == 0 {
                    start.push(Vec2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }

                data.push(e);
            }
        }

        let h = input.lines().count() as i32;
        let w = data.len() as i32 / h;

        Grid {
            width: w,
            height: h,
            data: data,
            start: start,
        }
    }

    pub fn index_from(&self, pos: &Vec2) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return Some(index);
        }
        None
    }

    pub fn value_for(&self, pos: &Vec2) -> usize {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return self.data[index];
        }
        panic!("this can't happen")
    }

    pub fn contains(&self, pos: &Vec2) -> bool {
        self.index_from(&pos).is_some()
    }

    pub fn draw(&self) {
        let n = self.width as usize;

        for (i, value) in self.data.iter().enumerate() {
            print!("{} ", value); // Print the current value with a space

            if (i + 1) % n == 0 {
                // Check if we need to insert a new line
                println!();
            }
        }

        if self.data.len() % n != 0 {
            // Print a final newline if the last line was incomplete
            println!("");
        }
    }
}

pub fn check_path(grid: &Grid, trails: &mut HashSet<Vec2>, curr_pos: &Vec2) {
    let curr = grid.value_for(&curr_pos);
    if curr == 9 {
        trails.insert(*curr_pos);
    }

    let directions = [
        Vec2 { x: 1, y: 0 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 1 },
        Vec2 { x: 0, y: -1 },
    ];

    for dir in directions {
        let next_pos = *curr_pos + dir;
        if grid.contains(&next_pos) {
            if (curr + 1) == grid.value_for(&next_pos) {
                check_path(grid, trails, &next_pos);
            }
        }
    }
}

pub fn rate_trail(grid: &Grid, curr_pos: &Vec2) -> usize {
    let curr = grid.value_for(&curr_pos);
    if curr == 9 {
        return 1;
    }

    let directions = [
        Vec2 { x: 1, y: 0 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 1 },
        Vec2 { x: 0, y: -1 },
    ];

    let mut result = 0;
    for dir in directions {
        let next_pos = *curr_pos + dir;
        if grid.contains(&next_pos) {
            if (curr + 1) == grid.value_for(&next_pos) {
                result += rate_trail(grid, &next_pos);
            }
        }
    }

    result
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let grid = Grid::new(input);

    let mut trails = HashSet::new();

    let mut result = 0;
    for pos in grid.start.iter() {
        trails.clear();
        check_path(&grid, &mut trails, &pos);
        result += trails.len();
    }

    result
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    let grid = Grid::new(input);

    let mut result = 0;
    for pos in grid.start.iter() {
        let rating = rate_trail(&grid, pos);
        result += rating;
    }

    result
}

fn main() {
    println!("part1 {}", do_part1(TEST_DATA));
    println!("part2 {}", do_part2(TEST_DATA));

    println!("part1 {}", do_part1(INPUT_DATA));
    println!("part2 {}", do_part2(INPUT_DATA));
}

#[test]
fn part1() {
    assert_eq!(36, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA));
}
