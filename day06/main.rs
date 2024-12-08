use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::io;

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

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Vec2 {
    pub fn rotate(&self) -> Vec2 {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }
}
#[derive(Clone)]
pub struct Map {
    width: i32,
    height: i32,
    start: Vec2,
    field: Vec<char>,
}

#[derive(PartialEq, Eq)]
pub enum Field {
    Empty,
    Player,
    Wall,
    OutOfBounds,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let data: Vec<char> = input.chars().filter(|&c| c != '\n' && c != '\r').collect();
        let h = input.lines().count();
        let w = data.len() / h;
        let start = data
            .iter()
            .position(|&c| c == '^')
            .map(|i| {
                let y = i / w;
                let x = i - y * w;
                Vec2 {
                    x: i32::try_from(x).unwrap(),
                    y: i32::try_from(y).unwrap(),
                }
            })
            .unwrap();

        Map {
            width: i32::try_from(w).unwrap(),
            height: i32::try_from(h).unwrap(),
            start: start,
            field: data,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn test(&self, pos: &Vec2) -> Field {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return match self.field[index] {
                '#' => Field::Wall,
                '^' => Field::Player,
                _ => Field::Empty,
            };
        }

        return Field::OutOfBounds;
    }

    pub fn set(&mut self, pos: &Vec2, c: char) {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            self.field[index] = c;
        }
    }

    pub fn print(&self) {
        let mut index: usize = 0;
        for _y in 0..self.height {
            for _x in 0..self.width {
                print!("{}", self.field[index]);
                index += 1;
            }
            println!("");
        }
    }

    pub fn set_wall(&mut self, pos: &Vec2, v: bool) {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            self.field[index] = if v { '#' } else { '.' };
        }
    }

    pub fn check(&self, pos: &Vec2) -> Option<char> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return Some(self.field[index]);
        }
        None
    }

    pub fn mark_visited(&mut self, pos: &Vec2) {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            self.field[index] = 'A';
        }
    }

    pub fn count_visited(&self) -> usize {
        self.field.iter().filter(|c| c == &&'A').count()
    }
}

fn do_part1(input: &String) -> usize {
    let m = Map::new(input);

    let mut pos = m.start();
    let mut dir = Vec2 { x: 0, y: -1 };

    let mut steps: HashSet<Vec2> = HashSet::new();

    while m.check(&pos).is_some() {
        if m.test(&(pos + dir)) == Field::Wall {
            dir = dir.rotate();
        }
        steps.insert(pos);

        // move to next field
        pos = pos + dir;
    }

    steps.len()
}

fn do_part2_is_time_loop(map: &Map) -> bool {
    let mut pos = map.start();
    let mut dir = Vec2 { x: 0, y: -1 };
    let mut turns: HashSet<(Vec2, Vec2)> = HashSet::new();

    while map.check(&pos).is_some() {
        let next_pos = pos + dir;
        if map.test(&next_pos) == Field::Wall {
            // create new turn
            let turn = (pos, dir);

            // check if we did this turn before
            if turns.get(&turn).is_some() {
                return true;
            }

            turns.insert(turn);
            dir = dir.rotate();
        }
        // move to next field
        pos = pos + dir
    }

    false
}

fn do_part2(input: &String) -> usize {
    let mut map = Map::new(input);
    let mut timeloops: usize = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Vec2 { x, y };
            if map.test(&pos) == Field::Empty {
                map.set(&pos, '#'); // set extra wall

                if do_part2_is_time_loop(&map) {
                    timeloops += 1;
                }

                map.set(&pos, '.'); // remove extra wall
            }
        }
    }

    timeloops
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let test_input = fs::read_to_string("day06/test.txt")?;
    let input = fs::read_to_string("day06/input.txt")?;

    println!("test {}", do_part2(&test_input));

    println!("part1 {}", do_part1(&input));

    //TODO fix logic, correct answer is 1770, this yields 1845 as of now, so we are counting something double
    //TODO optimize by checking less fields
    println!("part2 {}", do_part2(&input)); // correct answer is 1770, still wrong, do later

    Ok(())
}
