use std::fmt;

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }

    pub fn broadcast(v: i32) -> Self {
        Vec2 { x: v, y: v }
    }
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

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl std::ops::Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<Direction> for Vec2 {
    fn from(value: Direction) -> Vec2 {
        match value.into() {
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Right => Vec2 { x: 1, y: 0 },
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Down => Vec2 { x: 0, y: 1 },
        }
    }
}

impl From<&Direction> for Vec2 {
    fn from(value: &Direction) -> Vec2 {
        match value {
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Right => Vec2 { x: 1, y: 0 },
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Down => Vec2 { x: 0, y: 1 },
        }
    }
}

pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn for_each_field<F: Fn(Vec2, char)>(input: &str, func: F) {
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                func(
                    Vec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                );
            }
        }
    }

    pub fn new(width: i32, height: i32, data: Vec<T>) -> Self {
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn index_for(&self, pos: &Vec2) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            let index = pos.y * self.width + pos.x;
            return Some(index as usize);
        }
        None
    }

    pub fn contains(&self, pos: &Vec2) -> bool {
        self.index_for(pos).is_some()
    }

    pub fn value_for(&self, pos: &Vec2) -> Option<&T> {
        if let Some(index) = self.index_for(pos) {
            return Some(&self.data[index]);
        }
        None
    }

    pub fn value_for_checked(&self, pos: &Vec2) -> &T {
        if let Some(index) = self.index_for(pos) {
            return &self.data[index];
        }
        panic!("check position first via contains");
    }

    pub fn set_value_for(&mut self, pos: &Vec2, value: T ) {
        if let Some(index) = self.index_for(pos) {
            self.data[index] = value;
        }
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.width as usize;
        writeln!(f, "Grid {{ w: {} h: {} }}", self.width, self.height).expect("success");
        for(i, item) in self.data.iter().enumerate() {
            write!( f, "{:?}", item ).expect("success");
            if (i + 1) % n == 0 {
                writeln!(f).expect("success");
            }
        }
        Ok(())
    }
}
