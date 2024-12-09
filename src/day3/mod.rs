use regex::Regex;
use std::fs;

const INPUT: &str = "./src/day3/inputs.txt";
enum Instruction {
    Do,
    Dont,
}
pub fn clean_multiplication_file(input_path: &str) -> i32 {
    let input = fs::read_to_string(input_path).unwrap();
    mult_match(&input)
}

fn instructed_mult_count(input_path: &str) -> i32 {
    let input = fs::read_to_string(input_path).unwrap();
    let instructions_regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)");
    let mut current_instruction = Instruction::Do;
    let mut sum = 0;

    instructions_regex
        .unwrap()
        .find_iter(&input)
        .for_each(|instruction| {
            match instruction.as_str() {
                "do()" => current_instruction = Instruction::Do,
                "don't()" => current_instruction = Instruction::Dont,
                _ => {
                    // we are dealing with a multiplication here
                    match current_instruction {
                        Instruction::Do => {
                            sum += mult_match(instruction.as_str());
                        }
                        Instruction::Dont => {}
                    }
                }
            }
        });

    sum
}

fn mult_match(input: &str) -> i32 {
    let multi_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)");

    // Extract two digits from mul(n1,n2) statements and multiply them together.
    // Return the sum.
    multi_regex
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let (_, [n1, n2]) = c.extract();
            n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day3::{clean_multiplication_file, instructed_mult_count, INPUT};

    #[test]
    fn part1() {
        assert_eq!(188741603, clean_multiplication_file(INPUT));
    }
    #[test]
    fn part2() {
        assert_eq!(67269798, instructed_mult_count(INPUT));
    }
}
