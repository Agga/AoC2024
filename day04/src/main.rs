use std::env;
use std::fs;
use std::io;

pub struct Puzzle {
    width: i32,
    height: i32,
    field: Vec<char>,
}

// 140 * 140 = 19600

impl Puzzle {
    pub fn new(input: &str) -> Self {
        let mut data = Vec::new();

        for line in input.lines() {
            for c in line.chars() {
                data.push(c);
            }
        }

        let w = i32::try_from(data.len() / input.lines().count());
        let h = i32::try_from(input.lines().count());

        Puzzle {
            width: w.unwrap_or(1),
            height: h.unwrap_or(1),
            field: data,
        }
    }

    pub fn check(&self, x: i32, y: i32, c: char) -> bool {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = usize::try_from(y * self.width + x).unwrap();
            let element = self.field.iter().nth(index);
            return element == Some(&c);
        }

        false
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    // <->
    pub fn left_to_right(&self, x: i32, y: i32) -> bool {
        return self.check(x + 0, y + 0, 'X')
            && self.check(x + 1, y + 0, 'M')
            && self.check(x + 2, y + 0, 'A')
            && self.check(x + 3, y + 0, 'S');
    }

    pub fn right_to_left(&self, x: i32, y: i32) -> bool {
        return self.check(x - 0, y + 0, 'X')
            && self.check(x - 1, y + 0, 'M')
            && self.check(x - 2, y + 0, 'A')
            && self.check(x - 3, y + 0, 'S');
    }

    // ^-v
    pub fn top_to_bottom(&self, x: i32, y: i32) -> bool {
        return self.check(x + 0, y + 0, 'X')
            && self.check(x + 0, y + 1, 'M')
            && self.check(x + 0, y + 2, 'A')
            && self.check(x + 0, y + 3, 'S');
    }

    pub fn bottom_to_top(&self, x: i32, y: i32) -> bool {
        return self.check(x + 0, y - 0, 'X')
            && self.check(x + 0, y - 1, 'M')
            && self.check(x + 0, y - 2, 'A')
            && self.check(x + 0, y - 3, 'S');
    }

    pub fn top_left_to_bottom_right(&self, x: i32, y: i32) -> bool {
        return self.check(x + 0, y + 0, 'X')
            && self.check(x + 1, y + 1, 'M')
            && self.check(x + 2, y + 2, 'A')
            && self.check(x + 3, y + 3, 'S');
    }

    pub fn bottom_right_to_top_left(&self, x: i32, y: i32) -> bool {
        return self.check(x - 0, y - 0, 'X')
            && self.check(x - 1, y - 1, 'M')
            && self.check(x - 2, y - 2, 'A')
            && self.check(x - 3, y - 3, 'S');
    }

    pub fn bottom_left_to_top_right(&self, x: i32, y: i32) -> bool {
        return self.check(x + 0, y - 0, 'X')
            && self.check(x + 1, y - 1, 'M')
            && self.check(x + 2, y - 2, 'A')
            && self.check(x + 3, y - 3, 'S');
    }

    pub fn top_right_to_bottom_left(&self, x: i32, y: i32) -> bool {
        return self.check(x - 0, y + 0, 'X')
            && self.check(x - 1, y + 1, 'M')
            && self.check(x - 2, y + 2, 'A')
            && self.check(x - 3, y + 3, 'S');
    }

    pub fn test(&self, x:i32, y:i32)-> bool{
        return self.left_to_right(x, y)
            || self.right_to_left( x, y )
            || self.top_to_bottom( x, y )
            || self.bottom_to_top( x, y )
            || self.top_left_to_bottom_right( x, y )
            || self.bottom_right_to_top_left( x, y )
            || self.bottom_left_to_top_right( x, y )
            || self.top_right_to_bottom_left( x, y );
    }

    

}

fn do_part1(input: &String) -> i32 {
    let p = Puzzle::new(input);

    let mut result = 0;
    for x in 0..p.width(){
        for y in 0..p.height(){
            if p.test( x, y ){
                result += 1;
            }
        }
    }

    result
}

fn do_part2(_input: &String) -> i32 {
    0
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "src/input.txt";
    let contents = fs::read_to_string(input)?;

    println!("part1 {}", do_part1(&contents));
    println!("part2 {}", do_part2(&contents));

    Ok(())
}
