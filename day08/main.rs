use std::collections::HashMap;
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

impl Vec2 {
    pub fn rotate(&self) -> Vec2 {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Antenna(char),
    Resonance,
}

struct Map {
    width: i32,
    height: i32,
    data: Vec<Field>,
    antennas: HashMap<char, Vec<Vec2>>,
}

impl Map {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: i32::try_from(w).unwrap(),
            height: i32::try_from(h).unwrap(),
            data: vec![Field::Empty; w * h],
            antennas: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut index: usize = 0;
        for _ in 0..self.height {
            for _ in 0..self.width {
                print!(
                    "{}",
                    match self.data[index] {
                        Field::Empty => '.',
                        Field::Antenna(c) => c,
                        Field::Resonance => '#',
                    }
                );
                index += 1;
            }
            println!("");
        }
    }

    pub fn add_antenna(&mut self, pos: Vec2, c: char) {
        if let Some(index) = self.index_from(&pos) {
            self.data[index] = Field::Antenna(c);
        }
        self.antennas.entry(c).or_default().push(pos);
    }

    pub fn add_resonance(&mut self, pos: &Vec2) {
        if let Some(index) = self.index_from(&pos) {
            self.data[index] = Field::Resonance;
        }
    }

    pub fn index_from(&self, pos: &Vec2) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return Some(index);
        }
        None
    }

    pub fn contains(&self, pos: &Vec2) -> bool {
        self.index_from(&pos).is_some()
    }

    pub fn antennas(&self) -> &HashMap<char, Vec<Vec2>> {
        &self.antennas
    }
}

fn do_part1(input: &String) -> usize {
    let h = input.lines().count();
    let w = h;

    let mut map = Map::new(w, h);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                map.add_antenna(
                    Vec2 {
                        x: i32::try_from(x).unwrap(),
                        y: i32::try_from(y).unwrap(),
                    },
                    c,
                );
            }
        }
    }

    let mut resonances = HashSet::new();
    
    for (_, points) in map.antennas() {
        for &a in points {
            for &b in points {
                if a != b {
                    let resonance0: Vec2 = a + a - b;
                    let resonance1: Vec2 = b + b - a;

                    if map.contains( &resonance0 ){
                        resonances.insert( resonance0 );
                    }

                    if map.contains( &resonance1 ){
                        resonances.insert( resonance1 );
                    }
                }
            }
        }
    }

    for pos in resonances.iter() {
        map.add_resonance(pos);
    }

    resonances.len()
}

fn do_part2(input: &String) -> usize {
    let h = input.lines().count();
    let w = h;

    let mut map = Map::new(w, h);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                map.add_antenna(
                    Vec2 {
                        x: i32::try_from(x).unwrap(),
                        y: i32::try_from(y).unwrap(),
                    },
                    c,
                );
            }
        }
    }

    let mut resonances = HashSet::new();
    
    for (_, points) in map.antennas() {
        for &a in points.iter() {
            for &b in points.iter() {
                if a != b {
                    let dir = a - b;
                    let mut pos = a;
                    while map.contains( &pos ) {
                        resonances.insert( pos );
                        pos = pos + dir;
                    }

                    
                    let dir = b - a;
                    let mut pos = b;
                    while map.contains( &pos ) {
                        resonances.insert( pos );
                        pos = pos + dir;
                    }
                }
            }
        }
    }

    for pos in resonances.iter(){
        map.add_resonance(pos);
    }

    resonances.len()
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = fs::read_to_string("day08/input.txt")?;
    println!("part1 {}", do_part1(&input));
    println!("part2 {}", do_part2(&input));

    Ok(())
}

#[cfg(test)]
mod abc{
    use super::*;

    #[test]
    fn day08() {
        println!("Current working directory: {}", env::current_dir().unwrap().display() );

        let input = fs::read_to_string("day08/test.txt").unwrap();

        assert_eq!( 14, do_part1(&input));
        assert_eq!( 34, do_part2(&input));
    }
}
