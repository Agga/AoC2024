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

    let result = data
        .iter()
        .enumerate()
        .map(|(i, f)| match f {
            Entry::Emtpy => 0,
            Entry::File(index) => i * index,
        })
        .sum();

    result
}

pub fn do_part2(input: &str) -> usize {
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

    // for d in data.iter() {
    //     print!(
    //         "{}",
    //         match d {
    //             Entry::Emtpy => ".".to_string(),
    //             Entry::File(index) => index.to_string(),
    //         }
    //     )
    // }
    // println!("");

    for file_index in (0..file_index).rev() {
        // find begin/ end of file data
        let e = Entry::File(file_index);
        let file_begin = data.iter().position(|f| f == &e).unwrap();
        let file_end = data.iter().rposition(|f| f == &e).unwrap() + 1;
        let file_size = file_end - file_begin;

        // find empty spot that is big enough
        let count: usize = file_begin;
        let mut i = 0;
        while i < count {
            if data[i] == Entry::Emtpy {
                let begin = i;
                let mut end = begin;
                while end < count && data[end] == Entry::Emtpy {
                    end += 1;
                }

                let space = end - begin;
                if file_size <= space {
                    for n in 0..file_size {
                        data[begin + n] = data[file_begin + n];
                        data[file_begin + n] = Entry::Emtpy;
                    }

                    // for d in data.iter() {
                    //     print!(
                    //         "{}",
                    //         match d {
                    //             Entry::Emtpy => ".".to_string(),
                    //             Entry::File(index) => index.to_string(),
                    //         }
                    //     )
                    // }
                    // println!("");

                    break;
                }

                i = end;
            }

            i += 1;
        }
    }

    let result = data
        .iter()
        .enumerate()
        .map(|(i, f)| match f {
            Entry::Emtpy => 0,
            Entry::File(index) => i * index,
        })
        .sum();

    result
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
    assert_eq!(2858, do_part2(TEST_DATA));
}
