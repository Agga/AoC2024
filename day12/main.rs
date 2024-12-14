use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use itertools::Itertools;

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

#[derive(Default)]
pub struct Region {
    plant_type: char,
    fields: HashSet<Vec2>,
    edges: HashMap<Vec2, Vec<Vec2>>,
    perimeter: usize,
    area: usize,
    sides: usize,
}

pub struct Grid {
    width: i32,
    height: i32,
    data: Vec<char>,
}

#[allow(dead_code)]
    pub struct Blub<T> {
    width: i32,
    height: i32,
    data: Vec<T>,
}

impl<T> Blub<T> {
    pub fn new<F: Fn(Vec2, char) -> T>(input: &str, func: F) -> Self {
        let mut data: Vec<T> = Vec::new();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let pos = Vec2 {
                    x: x as i32,
                    y: y as i32,
                };
                data.push(func(pos, c));
            }
        }

        let h = input.lines().count();
        let w = data.len() / h;

        Blub {
            width: w as i32,
            height: h as i32,
            data: data,
        }
    }
}

const DIRECTIONS: [Vec2; 4] = [
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: 0, y: -1 },
];

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut data: Vec<char> = Vec::with_capacity(input.len());

        for (_, l) in input.lines().enumerate() {
            for (_, c) in l.chars().enumerate() {
                data.push(c);
            }
        }

        let h = input.lines().count() as i32;
        let w = data.len() as i32 / h;

        Grid {
            width: w,
            height: h,
            data: data,
        }
    }

    pub fn index_for(&self, pos: &Vec2) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return Some(index);
        }
        None
    }

    pub fn value_for(&self, pos: &Vec2) -> char {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return self.data[index];
        }
        panic!("this can't happen")
    }

    pub fn contains(&self, pos: &Vec2) -> bool {
        self.index_for(&pos).is_some()
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

    pub fn add_field(&self, region: &mut Region, pos: Vec2) {
        if region.fields.insert(pos) {
            region.area += 1;

            for dir in DIRECTIONS {
                let next = pos + dir;
                if self.contains(&next) && self.value_for(&next) == region.plant_type {
                    self.add_field(region, next);
                } else {
                    region.edges.entry(dir).or_default().push(pos);
                }
            }
        }
    }

    pub fn create_region(&self, plant: char, pos: &Vec2) -> Region {
        let mut region = Region::default();
        region.plant_type = plant;

        self.add_field(&mut region, *pos);
        region.perimeter = region.edges.iter().map(|(_, p)| p.len()).sum();

        for (dir, fields) in region.edges.iter() {
            let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();

            if dir.x == 0 {
                // grab all unique y positions
                edges = fields.iter().fold(HashMap::new(), |mut acc, p| {
                    acc.entry(p.y).or_default().push(p.x);
                    acc
                });
            }

            if dir.y == 0 {
                // grab all unique y positions
                edges = fields.iter().fold(HashMap::new(), |mut acc, p| {
                    acc.entry(p.x).or_default().push(p.y);
                    acc
                });
            }

            let mut sides = 0;

            for (_, fields) in edges.iter_mut() {
                fields.sort();
                let shared_edges: usize = fields
                    .iter()
                    .tuple_windows()
                    .filter(|(&a, &b)| i32::abs(a - b) == 1)
                    .count();

                sides += fields.len() - shared_edges;
            }

            region.sides += sides;
        }
        region
    }

    pub fn part1(&self) -> usize {
        let mut regions = Vec::new();
        let mut visited = HashSet::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vec2 { x, y };
                if !visited.contains(&pos) {
                    let region = self.create_region(self.value_for(&pos), &pos);
                    visited.extend(region.fields.clone());
                    regions.push(region);
                }
            }
        }

        let mut total_price = 0;
        for r in regions.iter() {
            let price = r.area * r.perimeter;

            // println!(
            //     "A region of {} plants with price {} * {} = {}.",
            //     r.plant_type, r.area, r.perimeter, price
            // );

            total_price += price;
        }

        total_price
    }

    pub fn part2(&self) -> usize {
        let mut regions = Vec::new();
        let mut visited = HashSet::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vec2 { x, y };
                if !visited.contains(&pos) {
                    let region = self.create_region(self.value_for(&pos), &pos);
                    visited.extend(region.fields.clone());
                    regions.push(region);
                }
            }
        }

        let mut total_price = 0;
        for r in regions.iter() {
            let price = r.area * r.sides;

            // println!(
            //     "A region of {} plants with price {} * {} = {}.",
            //     r.plant_type, r.area, r.sides, price
            // );

            total_price += price;
        }

        total_price
    }
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let s = Blub::new(input, |pos, c| (pos, c));

    let grid = Grid::new(input);
    grid.part1()
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.part2()
}

fn main() {
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("part2 {}", do_part2(INPUT_DATA));
}

#[test]
fn part1() {
    assert_eq!(1930, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(1206, do_part2(TEST_DATA));
}
