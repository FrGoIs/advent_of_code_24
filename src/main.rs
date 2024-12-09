use crate::day1::get_distance_and_similarity;
use crate::day2::get_reports_safety_count;
use crate::day3::clean_multiplication_file;
use crate::day4::count_sequence_every_dir;
use crate::day5::get_pages_and_rules;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    // println!("Day 1: {:?}", get_distance_and_similarity());
    // println!("Day 2: {:?}", get_reports_safety_count());
    // println!(
    //     "Day 3: {:?}",
    //     clean_multiplication_file("./src/day3/inputs.txt")
    // );
    // println!(
    //     "Day 4: {:?}",
    //     count_sequence_every_dir("./src/day4/test.txt", "XMAS")
    // );
    println!("Day 5: {:?}", get_pages_and_rules("./src/day5/inputs.txt"));
}
