use std::env;
use std::fs;
use std::io;

pub struct Puzzle {
    width: i32,
    height: i32,
    field: Vec<char>,
}

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

    pub fn xmas(&self, x: i32, y: i32, a: i32, b: i32) -> i32 {
        let xmas = ['X', 'M', 'A', 'S'];
        let result = (0..4)
            .zip(xmas)
            .all(|(i, c)| self.check(x + i * a, y + i * b, c));
        if result {
            1
        } else {
            0
        }
    }

    // directional test functions
    pub fn right(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, 1, 0);
    }

    pub fn left(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, -1, 0);
    }

    pub fn bottom(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, 0, 1);
    }

    pub fn top(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, 0, -1);
    }

    pub fn top_left(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, -1, -1);
    }

    pub fn top_right(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, 1, -1);
    }

    pub fn bottom_left(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, -1, 1);
    }

    pub fn bottom_right(&self, x: i32, y: i32) -> i32 {
        return self.xmas(x, y, 1, 1);
    }

    pub fn test_part1(&self, x: i32, y: i32) -> i32 {
        let mut result: i32 = 0;

        result += self.left(x, y);
        result += self.right(x, y);
        result += self.top(x, y);
        result += self.top_left(x, y);
        result += self.top_right(x, y);
        result += self.bottom(x, y);
        result += self.bottom_left(x, y);
        result += self.bottom_right(x, y);

        result
    }

    pub fn test_part2(&self, x: i32, y: i32) -> i32 {
        if self.check(x, y, 'A') {
            // top left <-> bottom_right
            let a = self.check(x - 1, y - 1, 'M') && self.check(x + 1, y + 1, 'S');
            let b = self.check(x - 1, y - 1, 'S') && self.check(x + 1, y + 1, 'M');

            // bottom_left <-> top_right
            let c = self.check(x - 1, y + 1, 'M') && self.check(x + 1, y - 1, 'S');
            let d = self.check(x - 1, y + 1, 'S') && self.check(x + 1, y - 1, 'M');

            if (a || b) && (c || d) {
                return 1;
            }
        }
        0
    }
}

fn do_part1(input: &String) -> i32 {
    let p = Puzzle::new(input);

    let mut result = 0;
    for x in 0..p.width() {
        for y in 0..p.height() {
            result += p.test_part1(x, y);
        }
    }

    result
}

fn do_part2(input: &String) -> i32 {
    let p = Puzzle::new(input);

    let mut result = 0;
    for x in 0..p.width() {
        for y in 0..p.height() {
            result += p.test_part2(x, y);
        }
    }

    result
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let input = "day04/input.txt";
    let contents = fs::read_to_string(input)?;

    println!("part1 {}", do_part1(&contents));
    println!("part2 {}", do_part2(&contents));

    Ok(())
}
