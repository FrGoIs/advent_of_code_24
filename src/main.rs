use crate::day1::get_distance_and_similarity;
use crate::day2::get_reports_safety_count;
use crate::day3::clean_multiplication_file;

mod day1;
mod day2;
mod day3;

fn main() {
    println!("Day 1: {:?}", get_distance_and_similarity());
    println!("Day 2: {:?}", get_reports_safety_count());
    println!("Day 3: {:?}", clean_multiplication_file());
}
