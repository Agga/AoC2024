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
    resonances: HashSet<Vec2>,
}

impl Map {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: i32::try_from(w).unwrap(),
            height: i32::try_from(h).unwrap(),
            data: vec![Field::Empty; w * h],
            antennas: HashMap::new(),
            resonances: HashSet::new(),
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
        if let Some(index) = self.index_form(&pos) {
            self.data[index] = Field::Antenna(c);
        }
        self.antennas.entry(c).or_default().push(pos);
    }

    pub fn add_resonance(&mut self, pos: &Vec2) {
        if let Some(index) = self.index_form(&pos) {
            self.data[index] = match self.data[index] {
                Field::Antenna(_) => Field::Antenna('R'),
                _ => Field::Resonance,
            };

            self.resonances.insert(*pos);
        }
    }

    pub fn index_form(&self, pos: &Vec2) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = usize::try_from(pos.y * self.width + pos.x).unwrap();
            return Some(index);
        }
        None
    }

    pub fn contains(&self, pos: &Vec2) -> bool {
        self.index_form(&pos).is_some()
    }

    pub fn antennas(&self) -> &HashMap<char, Vec<Vec2>> {
        &self.antennas
    }

    pub fn count_resonance(&self) -> usize {
        self.resonances.len()
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

    // TODO figure out how to not copy here
    let antennas = map.antennas().clone();
    for (_, points) in antennas {
        for a in points.iter() {
            for b in points.iter() {
                if a != b {
                    // TODO why do i need to deref here?
                    let resonance0: Vec2 = *a + *a - *b;
                    let resonance1: Vec2 = *b + *b - *a;

                    map.add_resonance(&resonance0);
                    map.add_resonance(&resonance1);
                }
            }
        }
    }

    // map.print();

    map.count_resonance()
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

    // TODO figure out how to not copy here
    let antennas = map.antennas().clone();

    for (_, points) in antennas {
        for a in points.iter() {
            for b in points.iter() {
                if a != b {
                    let dir = *a - *b;
                    let mut pos = *a;
                    while map.contains( &pos ) {
                        map.add_resonance( &pos );
                        pos = pos + dir;
                    }

                    
                    let dir = *b - *a;
                    let mut pos = *b;
                    while map.contains( &pos ) {
                        map.add_resonance( &pos );
                        pos = pos + dir;
                    }
                }
            }
        }
    }

    map.count_resonance()
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
