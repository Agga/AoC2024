use std::env;
use std::fs;
use std::io;

#[derive(Clone, Debug, Copy)]
enum Op {
    Add,
    Mul,
    And,
}

fn print_combination(result: usize, arguments: &Vec<usize>, operations: &Vec<Op>) {
    let ops = operations.iter().map(|op| match op {
        Op::Add => "+",
        Op::Mul => "*",
        Op::And => "||",
    });

    print!("{} = {}", result, arguments[0]);
    for (op, arg) in ops.zip(arguments.iter().skip(1)) {
        print!(" {} {}", op, arg);
    }
    println!(""); // flush
}

fn is_valid_combination(result: usize, arguments: &Vec<usize>, operations: &Vec<Op>) -> bool {
    assert_eq!(operations.len() + 1, arguments.len());

    let mut val = arguments[0];
    let mut op_index: usize = 0;

    fn and(a: usize, b: usize) -> usize {
        format!("{a}{b}").parse::<usize>().unwrap()
    }

    for i in 1..arguments.len() {
        val = match operations[op_index] {
            Op::Add => val + arguments[i],
            Op::Mul => val * arguments[i],
            Op::And => and(val, arguments[i]),
        };

        if val > result {
            return false;
        }

        op_index += 1;
    }

    result == val
}

fn do_any_part(input: &String, valid_operations: &[Op]) -> usize {
    let mut calibration_result: usize = 0;

    let base: usize = valid_operations.len();

    for line in input.lines() {
        let mut s = line.split(':');

        let r: usize = s.next().unwrap().parse().unwrap();
        let args = s
            .next()
            .map(|e| {
                e.split_whitespace()
                    .map(|arg| arg.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .unwrap();

        let op_count = args.len() - 1;
        let max_combinations = base.pow(u32::try_from(op_count).unwrap());

        let mut current_ops = vec![Op::Add; op_count];

        for i in 0..max_combinations {
            let mut bits: usize = i;
            let mut op_index = 0;

            while bits > 0 {
                current_ops[op_index] = valid_operations[ bits % base ]; 
                op_index += 1;
                bits /= 3;
            }

            if is_valid_combination(r, &args, &current_ops) {
                // print_combination(r, &args, &current_ops);
                calibration_result += r;
                break;
            }
        }
    }

    calibration_result
}

fn do_part1(input: &String) -> usize {
    do_any_part( &input, &[Op::Add, Op::Mul])
}

fn do_part2(input: &String) -> usize {
    do_any_part( &input, &[Op::Add, Op::Mul, Op::And])
}

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let test_input = fs::read_to_string("src/test.txt")?;
    let input = fs::read_to_string("src/input.txt")?;

    println!("test1 {}", do_part1(&test_input));
    println!("test2 {}", do_part2(&test_input));

    println!("part1 {}", do_part1(&input));
    println!("part2 {}", do_part2(&input));

    Ok(())
}
