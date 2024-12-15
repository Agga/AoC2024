use aoc::{Direction, Grid, Vec2};
use std::time::Instant;

#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");
#[allow(dead_code)]
const TEST_DATA1: &str = include_str!("test1.txt");
#[allow(dead_code)]
const TEST_DATA2: &str = include_str!("test2.txt");

#[derive(PartialEq, Clone, Copy)]
pub enum Field {
    Empty,
    Wall,
    Block,
    BlockL,
    BlockR,
    Robot,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Field::Empty => '.',
                Field::Wall => '#',
                Field::Block => 'O',
                Field::BlockL => '[',
                Field::BlockR => ']',
                Field::Robot => '@',
            }
        )
    }
}

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    let fields = "#.O@";
    let commands = "<v^>";

    let mut fields = Vec::new();
    let mut commands = Vec::new();

    let mut curr = Vec2::default();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if let Some(f) = match c {
                '#' => Some(Field::Wall),
                '.' => Some(Field::Empty),
                'O' => Some(Field::Block),
                '@' => Some(Field::Robot),
                _ => None,
            } {
                if f == Field::Robot {
                    curr = Vec2::new(x as i32, y as i32);
                }

                fields.push(f);
            }

            if let Some(c) = match c {
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                _ => None,
            } {
                commands.push(c);
            }
        }
    }

    let width = input.lines().next().unwrap().len();
    let height = fields.len() / width;

    let mut grid = Grid::new(width as i32, height as i32, fields);

    for command in commands.iter() {
        let dir = Vec2::from(command);
        let next = curr + dir;
        match grid.value_for_checked(&next) {
            Field::Wall => {
                // do nothing
            }
            Field::Block => {
                let mut block = next;

                loop {
                    match grid.value_for_checked(&block) {
                        Field::Empty => {
                            grid.set_value_for(&block, Field::Block);
                            grid.set_value_for(&next, Field::Robot);
                            grid.set_value_for(&curr, Field::Empty);
                            curr = next;
                            break;
                        }
                        Field::Wall => {
                            break;
                        }
                        _ => {
                            block = block + dir;
                        }
                    }
                }
            }
            Field::Empty => {
                grid.set_value_for(&curr, Field::Empty);
                grid.set_value_for(&next, Field::Robot);
                curr = next;
            }
            _ => {}
        }
    }

    let n = grid.width as usize;
    let gps: usize = grid
        .data
        .iter()
        .enumerate()
        .filter(|(_, f)| *f == &Field::Block)
        .map(|(i, _)| (i / n) * 100 + (i % n))
        .sum();

    gps
}

pub fn can_move(
    grid: &Grid<Field>,
    pos: &Vec2,
    dir: &Direction,
    points: &mut Vec<(Vec2, Vec2)>,
) -> bool {
    // find block
    let pos0 = *pos;
    let pos1 = match grid.value_for_checked(&pos0) {
        Field::BlockL => pos0 + Vec2::from(Direction::Right),
        Field::BlockR => pos0 + Vec2::from(Direction::Left),
        _ => panic!("can't happen"),
    };

    // next position
    let next0 = pos0 + Vec2::from(dir);
    let next1 = pos1 + Vec2::from(dir);

    // check if we hit a wall
    let mut can_move_to_field = |pos: &Vec2| -> bool {
        let &f = grid.value_for_checked(pos);
        match f {
            Field::Wall => false,
            Field::Empty => true,
            Field::BlockL | Field::BlockR => can_move(grid, pos, dir, points),
            _ => panic!("this should not happen"),
        }
    };

    // lateral check first
    let check0 = next0 == pos1 || can_move_to_field(&next0);
    let check1 = next1 == pos0 || can_move_to_field(&next1);

    if check0 && check1 {
        points.push((pos0, next0));
        points.push((pos1, next1));
        return true;
    }

    false
}

pub fn move_blocks(grid: &mut Grid<Field>, points: &Vec<(Vec2, Vec2)>) {
    let types: Vec<_> = points
        .iter()
        .map(|(from, _)| *grid.value_for_checked(from))
        .collect();

    for (f, _) in points {
        grid.set_value_for(f, Field::Empty);
    }

    for (t, (_, to)) in types.iter().zip(points.iter()) {
        grid.set_value_for(to, *t);
    }
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    let mut fields = Vec::new();
    let mut directions = Vec::new();

    let mut curr = Vec2::default();

    // parse fields
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    fields.push(Field::Wall);
                    fields.push(Field::Wall);
                }
                '.' => {
                    fields.push(Field::Empty);
                    fields.push(Field::Empty);
                }
                'O' => {
                    fields.push(Field::BlockL);
                    fields.push(Field::BlockR);
                }
                '@' => {
                    fields.push(Field::Robot);
                    fields.push(Field::Empty);

                    curr = Vec2 {
                        x: x as i32 * 2,
                        y: y as i32,
                    };
                }
                '<' => {
                    directions.push(Direction::Left);
                }
                '>' => {
                    directions.push(Direction::Right);
                }
                '^' => {
                    directions.push(Direction::Up);
                }
                'v' => {
                    directions.push(Direction::Down);
                }
                _ => {}
            }
        }
    }

    let width = input.lines().next().unwrap().len() * 2;
    let height = fields.len() / width;

    let mut grid = Grid::new(width as i32, height as i32, fields);

    // println!("{:?}", grid);

    for dir in directions.iter() {
        let next = curr + Vec2::from(dir);
        match grid.value_for_checked(&next) {
            Field::Wall => {
                // do nothing
            }
            Field::BlockL | Field::BlockR => {
                let mut points = vec![];
                if can_move(&grid, &next, dir, &mut points) {
                    points.push((curr, next));

                    move_blocks(&mut grid, &points);

                    // advance
                    curr = next;
                }
            }
            Field::Empty => {
                grid.set_value_for(&curr, Field::Empty);
                grid.set_value_for(&next, Field::Robot);
                curr = next;
            }
            _ => {}
        }

        // println!("Move {:?}:", dir);
        // println!("{:?}", grid);

        let count_robots = grid.data.iter().filter(|&&f| f == Field::Robot).count();
        assert!(count_robots == 1);
    }

    // println!("Move {:?}:", command);
    // println!("{:?}", grid);

    let n = grid.width as usize;
    let gps: usize = grid
        .data
        .iter()
        .enumerate()
        .filter(|(_, &f)| f == Field::BlockL)
        .map(|(i, _)| {
            let y = i / n;
            let x = i % n;
            // y * 100 + cmp::min( x, n-x-1)
            y * 100 + x
        })
        .sum();

    gps
}

fn main() {
    let mut now = Instant::now();
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("{:?}", now.elapsed());

    now = Instant::now();
    println!("part2 {}", do_part2(INPUT_DATA));
    println!("{:?}", now.elapsed());
}

#[test]
fn part1() {
    assert_eq!(10092, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(9021, do_part2(TEST_DATA));

    assert_eq!(618, do_part2(TEST_DATA2));

    // reddit test data
    assert_eq!(406, do_part2(include_str!("test1.txt")));
    assert_eq!(509, do_part2(include_str!("test3.txt")));
}
