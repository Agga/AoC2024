#[allow(dead_code)]
const INPUT_DATA: &str = include_str!("input.txt");
#[allow(dead_code)]
const TEST_DATA: &str = include_str!("test.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Entry {
    File(usize),
    Emtpy,
}

pub fn do_part1(input: &str) -> usize {
    let mut file_index = 0;
    let mut is_file = true;

    let mut data: Vec<Entry> = input
        .chars()
        .flat_map(|c| {
            let repeats = c.to_digit(10).unwrap() as usize;
            let mut entry = Entry::Emtpy;

            if is_file {
                entry = Entry::File(file_index);
                file_index += 1;
            }

            is_file = !is_file;

            vec![entry; repeats]
        })
        .collect();



    let mut l = 0;
    let mut r = data.len() - 1;
    while l < r {
        if data[l] == Entry::Emtpy {
            data[l] = data[r];
            data[r] = Entry::Emtpy;

            while data[r] == Entry::Emtpy {
                r -= 1;
            }
        }

        l += 1;
    }

    let result = data.iter().enumerate().map(|(i, f)|{
        match f {
            Entry::Emtpy => 0,
            Entry::File(index) => i * index,
        }
    }).sum();


    result
}

enum Stuff{
    File( usize, usize ),
    Empty( usize ),
}

pub fn do_part2(input: &str) -> usize {
    0
}

fn main() {
    println!("part1 {}", do_part1(INPUT_DATA));
    println!("part2 {}", do_part2(INPUT_DATA));
}

#[test]
fn part1() {
    assert_eq!(1928, do_part1(TEST_DATA));
}

#[test]
fn part2() {
    assert_eq!(0, do_part2(TEST_DATA));
}
