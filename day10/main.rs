#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[allow(unused_variables)]
pub fn do_part1(input: &str) -> usize {
    0
}

#[allow(unused_variables)]
pub fn do_part2(input: &str) -> usize {
    0
}

fn main() {
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("part2 {}", do_part2(INPUT_DATA));
}

#[test]
fn part1() {
    assert_eq!(0, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA));
}
